use std::{borrow::Cow, collections::HashMap, error::Error, sync::Arc};

mod constants;
mod structs;
#[cfg(test)]
mod tests;

use constants::{
    BNODE_ID_GENERATOR, COMMON_PREFIXES, NODE_NS_TYPE, NODE_RDFA_PATTERN_TYPE, RESERVED_KEYWORDS,
};
use itertools::Itertools;
use log::{debug, error};
use scraper::{ElementRef, Selector};
use url::Url;
use uuid::Uuid;

use crate::constants::NODE_RDF_XML_LITERAL;
use structs::{Context, Literal, Node, Statement};

pub use structs::RdfaGraph;

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

    ctx.vocab = elt
        .attr("vocab")
        .or_else(|| parent.as_ref().and_then(|p| p.vocab));

    ctx.base = element_ref
        .select(&Selector::parse("base")?)
        .next()
        .and_then(|e| e.attr("href"))
        .unwrap_or(ctx.base);

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
    let mut rels = elt.attr("rel").map(|r| parse_property_or_type_of(r, &ctx));
    let parent_in_rel = parent.and_then(|c| c.in_rel.clone());
    let parent_in_rev = parent.and_then(|c| c.in_rev.clone());
    let revs = elt.attr("rev").map(|r| parse_property_or_type_of(r, &ctx));
    let src_or_href = elt
        .attr("href")
        .or_else(|| elt.attr("src"))
        .and_then(|v| resolve_uri(v, &ctx, true).ok());
    let type_ofs = elt
        .attr("typeof")
        .map(|t| parse_property_or_type_of(t, &ctx));
    let predicates = property.map(|p| parse_property_or_type_of(p, &ctx));

    let current_node = if let Some(resource) = resource {
        let object = about
            .as_ref()
            .filter(|_| parent_in_rel.is_some() || parent_in_rev.is_some())
            .map(|a| Node::Ref(Arc::new(a.clone())))
            .unwrap_or(Node::Ref(Arc::new(resolve_uri(resource, &ctx, true)?)));
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
        } else if let Some(rels) = rels.take() {
            let subject = about
                .map(|a| Node::Ref(Arc::new(a)))
                .or_else(|| parent.and_then(|p| p.current_node.clone()))
                .ok_or("no parent node")?;
            for rel in rels {
                push_to_vec_if_not_present(
                    stmts,
                    Statement {
                        subject: subject.clone(),
                        predicate: rel,
                        object: object.clone(),
                    },
                );
            }
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
                if let (Some(src_or_href), Some(rels)) = (&src_or_href, &rels) {
                    for rel in rels {
                        push_to_vec_if_not_present(
                            stmts,
                            Statement {
                                subject: subject.clone(),
                                predicate: rel.clone(),
                                object: src_or_href.clone(),
                            },
                        );
                    }
                    if let Some(revs) = &revs {
                        for rev in revs {
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
        } else if let (Some(src_or_href), Some(rels)) = (&src_or_href, &rels) {
            for rel in rels {
                push_to_vec_if_not_present(
                    stmts,
                    Statement {
                        subject: subject.clone(),
                        predicate: rel.clone(),
                        object: src_or_href.clone(),
                    },
                );
            }
            if let Some(revs) = &revs {
                for rev in revs {
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
        }
        subject
    } else if let (Some(src_or_href), Some(rels)) = (&src_or_href, &rels) {
        // https://www.w3.org/TR/rdfa-core/#using-href-or-src-to-set-the-object
        let subject = parent
            .and_then(|p| p.current_node.clone())
            .unwrap_or_else(|| {
                Node::BNode(BNODE_ID_GENERATOR.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
            });
        for rel in rels {
            push_to_vec_if_not_present(
                stmts,
                Statement {
                    subject: subject.clone(),
                    predicate: rel.clone(),
                    object: src_or_href.clone(),
                },
            );
        }
        subject
    } else if type_ofs.is_some() {
        let child_with_rdfa_tag = element_ref
            .select(&Selector::parse(
                "[href], [src], [resource], [typeof], [property]",
            )?)
            .count();

        let node = if child_with_rdfa_tag == 0 {
            resolve_uri(ctx.base, &ctx, true)?
        } else {
            Node::BNode(BNODE_ID_GENERATOR.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
        };

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
            .clone()
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
        let sub = src_or_href.unwrap_or_else(|| current_node.clone());
        for type_of in type_ofs {
            push_to_vec_if_not_present(
                stmts,
                Statement {
                    subject: sub.clone(),
                    predicate: NODE_NS_TYPE.clone(),
                    object: type_of,
                },
            )
        }
    }
    ctx.current_node = Some(current_node.clone());
    ctx.in_rel = rels.clone();
    ctx.in_rev = revs.clone();
    if let Some(in_rels) = parent_in_rel {
        let parent = parent
            .and_then(|p| p.current_node.clone())
            .ok_or("in_rel: no parent node")?;

        for in_rel in in_rels {
            push_to_vec_if_not_present(
                stmts,
                Statement {
                    subject: parent.clone(),
                    predicate: in_rel,
                    object: current_node.clone(),
                },
            );
        }
    }
    if let Some(in_revs) = parent_in_rev {
        let parent = parent
            .and_then(|p| p.current_node.clone())
            .ok_or("in_rel: no parent node")?;
        for in_rev in in_revs {
            push_to_vec_if_not_present(
                stmts,
                Statement {
                    object: parent.clone(),
                    predicate: in_rev,
                    subject: current_node.clone(),
                },
            );
        }
    }
    for child in element_ref
        .children()
        // to skip when there are no attributes, see e.g earl_html5/example0084.html
        .flat_map(|c| {
            if c.value()
                .as_element()
                .filter(|e| e.attrs().count() == 0)
                .is_some()
            {
                c.children().collect_vec().into_iter()
            } else {
                vec![c].into_iter()
            }
        })
    {
        if let Some(c) = ElementRef::wrap(child) {
            let mut replace_by_bnode = None;
            if let Some(in_rels) = &ctx.in_rel {
                // Triples are also 'completed' if any one of @property, @rel or @rev are present.
                if (c.attr("property").is_some()
                    || c.attr("rel").is_some()
                    || c.attr("rev").is_some())
                    && (c.attr("about").is_none() && c.attr("typeof").is_none())
                {
                    let b_node = Node::BNode(
                        BNODE_ID_GENERATOR.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                    );
                    for in_rel in in_rels {
                        push_to_vec_if_not_present(
                            stmts,
                            Statement {
                                subject: current_node.clone(),
                                predicate: in_rel.clone(),
                                object: b_node.clone(),
                            },
                        );
                    }
                    replace_by_bnode = Some(b_node);
                }
            }
            if let Some(in_revs) = &ctx.in_rev {
                // Triples are also 'completed' if any one of @property, @rel or @rev are present.
                if (c.attr("property").is_some()
                    || c.attr("rel").is_some()
                    || c.attr("rev").is_some())
                    && (c.attr("about").is_none() && c.attr("typeof").is_none())
                {
                    let b_node = replace_by_bnode.unwrap_or_else(|| {
                        Node::BNode(
                            BNODE_ID_GENERATOR.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                        )
                    });
                    for in_rev in in_revs {
                        push_to_vec_if_not_present(
                            stmts,
                            Statement {
                                subject: b_node.clone(),
                                predicate: in_rev.clone(),
                                object: current_node.clone(),
                            },
                        );
                    }

                    replace_by_bnode = Some(b_node);
                }
            }
            if let Some(b_node) = replace_by_bnode {
                ctx.current_node = Some(b_node);
                ctx.in_rel = None;
                ctx.in_rev = None;
            }
            // However, unlike the situation when @about or @typeof are present, all predicates are attached to one bnode
            if c.attr("about").is_some() || c.attr("typeof").is_some() {
                ctx.in_rel = rels.clone();
                ctx.in_rev = revs.clone();
                ctx.current_node = Some(current_node.clone());
            }

            let child_ctx = Context {
                base: ctx.base,
                ..Default::default()
            };
            let _ = traverse_element(&c, Some(&ctx), child_ctx, stmts)?;
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
        let texts = element_ref
            .text()
            .filter(|t| !t.trim().is_empty())
            .collect::<Vec<_>>();
        let text = if texts.is_empty() {
            Cow::Borrowed("")
        } else if texts.len() == 1 {
            let text = {
                if texts[0].lines().filter(|l| !l.trim().is_empty()).count() == 1 {
                    texts[0].trim()
                } else {
                    texts[0]
                }
            };
            Cow::Borrowed(text)
        } else {
            let text = texts.iter().map(|t| t.to_string()).join("");
            Cow::Owned(text)
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
                    let uuid = if cfg!(test) {
                        Uuid::nil()
                    } else {
                        Uuid::new_v4()
                    };
                    return Ok(Node::RefBNode((reference.trim(), uuid)));
                } else if prefix.trim().is_empty() && !reference.is_empty() {
                    return Ok(Node::Iri(Cow::Owned(
                        [COMMON_PREFIXES[""], reference].join(""),
                    )));
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
            } else if RESERVED_KEYWORDS.binary_search(&uri).ok().is_some() {
                Ok(Node::Iri(Cow::Owned([COMMON_PREFIXES[""], uri].join(""))))
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
