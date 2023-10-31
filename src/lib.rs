use std::{
    borrow::Cow,
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter},
    sync::Arc,
    u64,
};

use constants::{
    BNODE_ID_GENERATOR, COMMON_PREFIXES, DEFAULT_WELL_KNOWN_PREFIX, NODE_NS_TYPE,
    NODE_RDFA_PATTERN_TYPE, NODE_RDF_XSD_STRING,
};
use itertools::Itertools;
use scraper::ElementRef;
use url::Url;
mod constants;
use log::{debug, error};
use uuid::Uuid;

use crate::constants::NODE_RDF_XML_LITERAL;
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct RdfaGraph<'a>(Vec<Statement<'a>>);

#[derive(Debug, Default, Clone)]
pub struct Context<'a> {
    base: &'a str,
    vocab: Option<&'a str>,
    lang: Option<&'a str>,
    in_rel: Option<Node<'a>>,
    in_rev: Option<Node<'a>>,
    current_node: Option<Node<'a>>,
    prefixes: HashMap<&'a str, &'a str>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Literal<'a> {
    datatype: Option<Box<Node<'a>>>,
    value: Cow<'a, str>,
    lang: Option<&'a str>,
}

#[derive(Debug, Clone, Eq, PartialOrd, Ord, Hash)]
pub enum Node<'a> {
    Iri(Cow<'a, str>),
    Literal(Literal<'a>),
    Ref(Arc<Node<'a>>),
    List(Vec<Node<'a>>),
    BNode(u64),
    RefBNode((&'a str, Uuid)),
}
impl Node<'_> {
    pub fn is_empty(&self) -> bool {
        match self {
            Node::Iri(iri) => iri.is_empty(),
            Node::Literal(l) => {
                l.value.is_empty()
                    && l.datatype.as_ref().filter(|li| !li.is_empty()).is_none()
                    && l.lang.filter(|lan| lan.is_empty()).is_none()
            }
            Node::Ref(r) => r.is_empty(),
            Node::List(list) => list.iter().all(|l| l.is_empty()),
            Node::BNode(_) => false,
            Node::RefBNode((s, _)) => s.is_empty(),
        }
    }
}

impl PartialEq for Node<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Iri(l0), Self::Iri(r0)) => l0 == r0,
            (Self::Literal(l0), Self::Literal(r0)) => l0 == r0,
            (Self::Ref(l0), Self::Ref(r0)) => l0 == r0,
            (Self::Ref(l0), rhs) => l0.as_ref() == rhs,
            (lhs, Self::Ref(r0)) => lhs == r0.as_ref(),
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::BNode(l0), Self::BNode(r0)) => l0 == r0,
            (Self::RefBNode(l0), Self::RefBNode(r0)) => l0 == r0,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Statement<'a> {
    subject: Node<'a>,
    predicate: Node<'a>,
    object: Node<'a>,
}

impl Display for Node<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Iri(iri) => f.write_str(&format!("<{}>", iri)),
            Node::Ref(iri) => f.write_str(&format!("{}", iri)),
            Node::Literal(Literal {
                datatype,
                lang,
                value,
            }) => {
                let mut s = if value
                    .as_ref()
                    .chars()
                    .any(|c| c.is_ascii_control() || c.is_control())
                {
                    format!(r#""""{value}""""#)
                } else {
                    format!(r#""{value}""#)
                };

                if let Some(datatype) = datatype
                    .as_ref()
                    .filter(|dt| dt.as_ref() != &*NODE_RDF_XSD_STRING)
                {
                    s.push_str(&format!(r#"^^{datatype}"#));
                } else if let Some(lang) = lang {
                    s.push_str(&format!(r#"@{lang}"#));
                }
                f.write_str(&s)
            }
            Node::BNode(id) => {
                // todo maybe this should use the base?
                f.write_str(&format!("<{}{}>", DEFAULT_WELL_KNOWN_PREFIX, id))
            }
            Node::RefBNode((id, uuid)) => {
                if let Ok(v) = id.parse::<u64>() {
                    if v <= BNODE_ID_GENERATOR.load(std::sync::atomic::Ordering::SeqCst) {
                        f.write_str(&format!("<{}{}>", DEFAULT_WELL_KNOWN_PREFIX, uuid))
                    } else {
                        f.write_str(&format!("<{}{}>", DEFAULT_WELL_KNOWN_PREFIX, id))
                    }
                } else {
                    f.write_str(&format!("<{}{}>", DEFAULT_WELL_KNOWN_PREFIX, id))
                }
            }
            e => {
                error!("fixme! format for {e:?} not implemented");
                Err(std::fmt::Error)
            }
        }
    }
}

impl Display for Statement<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Statement {
            subject,
            predicate,
            object,
        } = self;
        f.write_str(&format!(r#"{subject} {predicate} {object}."#))
    }
}
impl Display for RdfaGraph<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .0
                .iter()
                .map(Statement::to_string)
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

impl<'a> RdfaGraph<'a> {
    pub fn parse(
        input: &ElementRef<'a>,
        initial_context: Context<'a>,
    ) -> Result<RdfaGraph<'a>, Box<dyn Error>> {
        let mut triples = vec![];
        traverse_element(input, None, initial_context, &mut triples)?;

        triples = copy_pattern(triples)?;
        // copy patterns

        Ok(RdfaGraph(triples))
    }
}

pub fn copy_pattern(triples: Vec<Statement<'_>>) -> Result<Vec<Statement<'_>>, Box<dyn Error>> {
    let (pattern_type, pattern): (Vec<Statement>, Vec<Statement>) = triples
        .into_iter()
        .partition(|stmt| stmt.object == *NODE_RDFA_PATTERN_TYPE);

    let (pattern_predicate, pattern): (Vec<Statement>, Vec<Statement>) = pattern
        .into_iter()
        .partition(|stmt| pattern_type.iter().any(|s| s.subject == stmt.subject));

    let (pattern_subject, mut triples): (Vec<Statement>, Vec<Statement>) = pattern
        .into_iter()
        .partition(|stmt| pattern_predicate.iter().any(|s| s.subject == stmt.object));

    for Statement {
        subject, object, ..
    } in pattern_subject
    {
        for Statement {
            predicate,
            object: obj,
            ..
        } in pattern_predicate
            .iter()
            .filter(|stmt| object == stmt.subject)
        {
            push_to_vec_if_not_present(
                &mut triples,
                Statement {
                    subject: subject.clone(),
                    predicate: predicate.clone(),
                    object: obj.clone(),
                },
            )
        }
    }
    Ok(triples)
}

fn push_to_vec_if_not_present<T: PartialEq>(array: &mut Vec<T>, value: T) {
    if !array.contains(&value) {
        array.push(value);
    }
}
pub fn traverse_element<'a>(
    element_ref: &ElementRef<'a>,
    parent: Option<&Context<'a>>,
    mut ctx: Context<'a>,
    stmts: &mut Vec<Statement<'a>>,
) -> Result<Option<Node<'a>>, Box<dyn Error>> {
    let elt = element_ref.value();

    // extract attrs
    ctx.vocab = elt
        .attr("vocab")
        .or_else(|| parent.as_ref().and_then(|p| p.vocab));

    if let Some(prefix) = elt.attr("prefix") {
        ctx.prefixes = parse_prefixes(prefix);
    } else if let Some(parent) = parent {
        ctx.prefixes = parent.prefixes.clone();
    }

    let resource = elt.attr("resource");
    ctx.lang = elt
        .attr("lang")
        .or_else(|| elt.attr("xml:lang"))
        .or_else(|| parent.and_then(|p| p.lang));
    let about = elt
        .attr("about")
        .and_then(|a| resolve_uri(a, &ctx, true).ok());
    let property = elt.attr("property");
    let mut rel = elt.attr("rel").and_then(|r| {
        resolve_uri(r, &ctx, false)
            .map(|n| Node::Ref(Arc::new(n)))
            .ok()
    });
    let parent_in_rel = parent.and_then(|c| c.in_rel.clone());
    let parent_in_rev = parent.and_then(|c| c.in_rev.clone());
    let rev = elt.attr("rev").and_then(|r| {
        resolve_uri(r, &ctx, false)
            .map(|n| Node::Ref(Arc::new(n)))
            .ok()
    });
    let src_or_href = elt
        .attr("href")
        .or_else(|| elt.attr("src"))
        .and_then(|v| resolve_uri(v, &ctx, true).ok());
    let type_ofs = elt
        .attr("typeof")
        .map(|t| parse_property_or_type_of(t, &ctx));
    let predicates = property.map(|p| parse_property_or_type_of(p, &ctx));

    let current_node = if let Some(resource) = resource {
        let object = Node::Ref(Arc::new(resolve_uri(resource, &ctx, true)?));
        if let Some(predicates) = &predicates {
            let subject = about
                .map(|a| Node::Ref(Arc::new(a)))
                .or_else(|| parent.and_then(|p| p.current_node.clone()))
                .ok_or("no parent node")?;
            for predicate in predicates {
                push_to_vec_if_not_present(
                    stmts,
                    Statement {
                        subject: subject.clone(),
                        predicate: predicate.clone(),
                        object: object.clone(),
                    },
                );
            }
            if type_ofs.is_some() {
                object
            } else {
                subject
            }
        } else if let Some(rel) = rel.take() {
            let subject = about
                .map(|a| Node::Ref(Arc::new(a)))
                .or_else(|| parent.and_then(|p| p.current_node.clone()))
                .ok_or("no parent node")?;
            push_to_vec_if_not_present(
                stmts,
                Statement {
                    subject: subject.clone(),
                    predicate: rel,
                    object: object.clone(),
                },
            );
            object
        } else {
            object
        }
    } else if let Some(about) = about {
        // handle about case. set the context.
        // if property is present, children become objects of current.
        let subject = Node::Ref(Arc::new(about));

        if let Some(predicates) = &predicates {
            for predicate in predicates {
                if let Some(content) = element_ref.attr("content") {
                    push_to_vec_if_not_present(
                        stmts,
                        Statement {
                            subject: subject.clone(),
                            predicate: predicate.clone(),
                            object: Node::Ref(Arc::new(Node::Literal(Literal {
                                datatype: elt
                                    .attr("datatype")
                                    .and_then(|dt| resolve_uri(dt, &ctx, false).ok())
                                    .map(Box::new),
                                value: Cow::Borrowed(content),
                                lang: elt.attr("lang").or(ctx.lang),
                            }))),
                        },
                    );
                }
                if let (Some(src_or_href), Some(rel)) = (&src_or_href, &rel) {
                    if !rel.is_empty() {
                        push_to_vec_if_not_present(
                            stmts,
                            Statement {
                                subject: subject.clone(),
                                predicate: rel.clone(),
                                object: src_or_href.clone(),
                            },
                        );
                    }
                    if let Some(rev) = &rev {
                        push_to_vec_if_not_present(
                            stmts,
                            Statement {
                                subject: src_or_href.clone(),
                                predicate: rev.clone(),
                                object: subject.clone(),
                            },
                        )
                    }
                } else {
                    push_to_vec_if_not_present(
                        stmts,
                        Statement {
                            subject: subject.clone(),
                            predicate: predicate.clone(),
                            object: Node::Ref(Arc::new(extract_literal(element_ref, &ctx)?)),
                        },
                    );
                }
            }
        } else if let (Some(src_or_href), Some(rel)) = (&src_or_href, &rel) {
            if !rel.is_empty() {
                push_to_vec_if_not_present(
                    stmts,
                    Statement {
                        subject: subject.clone(),
                        predicate: rel.clone(),
                        object: src_or_href.clone(),
                    },
                );
            }
            if let Some(rev) = &rev {
                push_to_vec_if_not_present(
                    stmts,
                    Statement {
                        subject: src_or_href.clone(),
                        predicate: rev.clone(),
                        object: subject.clone(),
                    },
                )
            }
        }
        subject
    } else if let (Some(src_or_href), Some(rel)) = (&src_or_href, &rel) {
        // https://www.w3.org/TR/rdfa-core/#using-href-or-src-to-set-the-object
        let subject = parent
            .and_then(|p| p.current_node.clone())
            .unwrap_or_else(|| {
                Node::BNode(BNODE_ID_GENERATOR.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
            });

        push_to_vec_if_not_present(
            stmts,
            Statement {
                subject: subject.clone(),
                predicate: rel.clone(),
                object: src_or_href.clone(),
            },
        );
        subject
    } else if type_ofs.is_some() {
        // for some reasons it seems that if there is a typeof but no
        // about and no resource, it becomes an anon node
        // this might be incorrect
        let node =
            Node::BNode(BNODE_ID_GENERATOR.fetch_add(1, std::sync::atomic::Ordering::SeqCst));
        let subject = parent
            .and_then(|p| p.current_node.clone())
            .unwrap_or_else(|| {
                Node::BNode(BNODE_ID_GENERATOR.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
            });
        if let Some(predicates) = &predicates {
            for predicate in predicates {
                push_to_vec_if_not_present(
                    stmts,
                    Statement {
                        subject: subject.clone(),
                        predicate: predicate.clone(),
                        object: node.clone(),
                    },
                );
            }
        }
        node
    } else {
        let subject = src_or_href
            .filter(|_| parent_in_rel.is_some() || parent_in_rev.is_some())
            .or_else(|| parent.and_then(|p| p.current_node.clone()))
            .unwrap_or(resolve_uri(ctx.base, &ctx, true)?);
        if let Some(predicates) = &predicates {
            for predicate in predicates {
                push_to_vec_if_not_present(
                    stmts,
                    Statement {
                        subject: subject.clone(),
                        predicate: predicate.clone(),
                        object: Node::Ref(Arc::new(extract_literal(element_ref, &ctx)?)),
                    },
                );
            }
        }
        subject
    };

    if let Some(type_ofs) = type_ofs {
        for type_of in type_ofs {
            push_to_vec_if_not_present(
                stmts,
                Statement {
                    subject: current_node.clone(),
                    predicate: NODE_NS_TYPE.clone(),
                    object: type_of,
                },
            )
        }
    }
    ctx.current_node = Some(current_node.clone());
    ctx.in_rel = rel;
    ctx.in_rev = rev;
    if let Some(in_rel) = parent_in_rel {
        let parent = parent
            .and_then(|p| p.current_node.clone())
            .ok_or("in_rel: no parent node")?;

        push_to_vec_if_not_present(
            stmts,
            Statement {
                subject: parent,
                predicate: in_rel,
                object: current_node.clone(),
            },
        );
    }
    if let Some(in_rev) = parent_in_rev {
        let parent = parent
            .and_then(|p| p.current_node.clone())
            .ok_or("in_rel: no parent node")?;
        push_to_vec_if_not_present(
            stmts,
            Statement {
                object: parent,
                predicate: in_rev,
                subject: current_node.clone(),
            },
        );
    }
    if element_ref.has_children() {
        for child in element_ref.children() {
            if let Some(c) = ElementRef::wrap(child) {
                if let Some(in_rel) = &ctx.in_rel {
                    // Triples are also 'completed' if any one of @property, @rel or @rev are present.
                    // However, unlike the situation when @about or @typeof are present, all predicates are attached to one bnode
                    if (c.attr("property").is_some()
                        || c.attr("rel").is_some()
                        || c.attr("rev").is_some())
                        && (c.attr("about").is_none() && c.attr("typeof").is_none())
                    {
                        let b_node = Node::BNode(
                            BNODE_ID_GENERATOR.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                        );
                        push_to_vec_if_not_present(
                            stmts,
                            Statement {
                                subject: current_node.clone(),
                                predicate: in_rel.clone(),
                                object: b_node.clone(),
                            },
                        );
                        ctx.current_node = Some(b_node);
                        ctx.in_rel = None;
                    }
                }
                let child_ctx = Context {
                    base: ctx.base,
                    ..Default::default()
                };
                let _ = traverse_element(&c, Some(&ctx), child_ctx, stmts)?;
            }
        }
    }
    Ok(ctx.current_node.clone())
}
pub fn extract_literal<'a>(
    element_ref: &ElementRef<'a>,
    ctx: &Context<'a>,
) -> Result<Node<'a>, &'static str> {
    let elt_val = element_ref.value();
    let datatype = elt_val
        .attr("datatype")
        .filter(|dt| !dt.is_empty())
        .and_then(|dt| match resolve_uri(dt, ctx, false) {
            Ok(d) => Some(Box::new(d)),
            Err(e) => {
                eprintln!("could not parse {dt}. error {e}");
                None
            }
        });
    let lang = elt_val
        .attr("lang")
        .or_else(|| elt_val.attr("xml:lang"))
        .or(ctx.lang)
        .filter(|_| datatype.is_none());

    if let Some(value) = elt_val.attr("href").or(elt_val.attr("src")) {
        resolve_uri(value, ctx, true)
    } else if let Some(content) = elt_val.attr("content") {
        Ok(Node::Literal(Literal {
            datatype,
            value: Cow::Borrowed(content),
            lang,
        }))
    } else if datatype
        .as_ref()
        .filter(|dt| dt.as_ref() == &*NODE_RDF_XML_LITERAL)
        .is_some()
    {
        Ok(Node::Literal(Literal {
            value: Cow::Owned(element_ref.inner_html()),
            datatype,
            lang: None,
        }))
    } else {
        let texts = element_ref.text().collect::<Vec<_>>();
        let text = if texts.is_empty() {
            Cow::Borrowed("")
        } else if texts.len() == 1 {
            Cow::Borrowed(texts[0])
        } else {
            Cow::Owned(texts.iter().map(|t| t.to_string()).collect())
        };
        Ok(Node::Literal(Literal {
            datatype,
            value: text,
            lang,
        }))
    }
}

pub fn resolve_uri<'a>(
    uri: &'a str,
    ctx: &Context<'a>,
    is_resource: bool,
) -> Result<Node<'a>, &'static str> {
    let iri = Url::parse(uri);

    match iri {
        Ok(iri) if !iri.cannot_be_a_base() || iri.is_special() => Ok(Node::Iri(Cow::Borrowed(uri))),

        // Curie
        Ok(iri) => {
            if uri.starts_with("mail") || uri.starts_with("tel") {
                Ok(Node::Iri(Cow::Borrowed(uri)))
            } else if let Some(value) = ctx.prefixes.get(iri.scheme()) {
                let iri = uri.replace(':', "").trim().replacen(iri.scheme(), value, 1);
                Ok(Node::Iri(Cow::Owned(iri)))
            } else if let Some(value) = COMMON_PREFIXES.get(iri.scheme()) {
                let iri = uri.replace(':', "").trim().replacen(iri.scheme(), value, 1);
                Ok(Node::Iri(Cow::Owned(iri)))
            } else {
                Ok(Node::Iri(Cow::Owned(uri.to_string())))
            }
        }
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            if let Ok((prefix, reference)) = parse_safe_curie(uri) {
                if prefix.trim() == "_" {
                    return Ok(Node::RefBNode((reference, Uuid::new_v4())));
                }
            }
            if is_resource || uri.starts_with('#') || uri.starts_with('/') {
                let trailing_white_space = if ctx.base.ends_with('/')
                    || ctx.base.ends_with('#')
                    || uri.starts_with('/')
                    || uri.starts_with('#')
                {
                    ""
                } else {
                    "/"
                };
                Ok(Node::Iri(Cow::Owned(
                    [ctx.base, trailing_white_space, uri].join(""),
                )))
            } else if let Some(vocab) = ctx.vocab {
                Ok(Node::Iri(Cow::Owned([vocab, uri].join(""))))
            } else {
                debug!("could not determine base/vocab {:?}", ctx);
                Ok(Node::Iri(Cow::Borrowed(uri)))
            }
        }
        Err(e) => {
            eprintln!("invalid uri {uri}. error: {e}");
            Err("could not resolve uri")
        }
    }
}

fn parse_safe_curie(s: &str) -> Result<(&str, &str), &'static str> {
    let mut s = s.trim();
    if s.starts_with('[') {
        if !s.ends_with(']') {
            return Err("invalid SafeCurie");
        }
        s = &s[1..s.len() - 1];
    }
    s.split_once(':').ok_or("not a curie")
}

fn parse_prefixes(s: &str) -> HashMap<&str, &str> {
    s.split_whitespace()
        .map(|s| s.trim())
        .tuples::<(_, _)>()
        .filter_map(|(s, p)| {
            if let Ok((s, _)) = parse_safe_curie(s) {
                Some((s, p))
            } else {
                error!("fixme! couldn't parse curie for {s}, {p}");
                None
            }
        })
        .collect()
}

fn parse_property_or_type_of<'a>(s: &'a str, ctx: &Context<'a>) -> Vec<Node<'a>> {
    s.split_whitespace()
        .filter_map(|uri| resolve_uri(uri, ctx, false).ok())
        .map(|n| Node::Ref(Arc::new(n)))
        .collect_vec()
}
