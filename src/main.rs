#![allow(unused)]
use std::{
    borrow::{BorrowMut, Cow},
    collections::HashMap,
    default,
    error::Error,
    fmt::{Display, Formatter},
    rc::Rc,
};

use constants::{
    COMMON_PREFIXES, DEFAULT_WELL_KNOWN_PREFIX, NODE_NS_TYPE, NODE_RDFA_COPY_PREDICATE,
    NODE_RDFA_PATTERN_TYPE, NS_TYPE, RDFA_COPY_PREDICATE, RDFA_PATTERN_TYPE,
};
use scraper::{ElementRef, Html, Selector};
use url::Url;
use uuid::Uuid;
mod constants;
mod utils;

#[cfg(test)]
mod tests;

macro_rules! select {
    ($e:literal) => {
        &Selector::parse($e)?
    };
}

#[derive(Debug)]
pub struct RdfaGraph<'a>(Vec<Statement<'a>>);

#[derive(Debug, Default, Clone)]
pub struct Context<'a> {
    base: &'a str,
    vocab: Option<&'a str>,
    parent: Option<Rc<Context<'a>>>,
    current_node: Option<Node<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct Literal<'a> {
    datatype: Option<Box<Node<'a>>>,
    value: Cow<'a, str>,
    lang: Option<&'a str>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub enum Node<'a> {
    Iri(Cow<'a, str>),
    Literal(Literal<'a>),
    Ref(Box<Node<'a>>),
    List(Vec<Node<'a>>),
    BNode(Uuid),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct Statement<'a> {
    pub subject: Node<'a>,
    pub predicate: Node<'a>,
    pub object: Node<'a>,
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
                let mut s = format!(r#""{value}""#);
                if let Some(datatype) = datatype {
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
            e => {
                eprintln!("fixme! format for {e:?} not implemented");
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
        traverse_element(&input, initial_context, &mut triples)?;
        copy_pattern(&mut triples);
        // copy patterns

        Ok(RdfaGraph(triples))
    }
}

pub fn copy_pattern<'a>(triples: &mut Vec<Statement<'a>>) -> Result<(), Box<dyn Error>> {
    let mut copy_patterns_subject = triples
        .iter()
        .filter_map(|stmt| {
            if stmt.predicate == *NODE_NS_TYPE && stmt.object == *NODE_RDFA_PATTERN_TYPE {
                Some(stmt.subject.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // triples.retain(|stmt| stmt.object != *NODE_RDFA_PATTERN_TYPE);

    let copy_patterns = triples
        .iter()
        .filter_map(|stmt| {
            if copy_patterns_subject
                .iter()
                .any(|c| &stmt.subject == c && stmt.object != *NODE_RDFA_PATTERN_TYPE)
            {
                Some((
                    stmt.subject.clone(),
                    (stmt.predicate.clone(), stmt.object.clone()),
                ))
            } else {
                None
            }
        })
        .fold(HashMap::new(), |mut map, (k, v)| {
            map.entry(k).or_insert_with(|| Vec::new()).push(v);
            map
        });

    let copy_patterns_predicate_subject = triples
        .iter()
        .filter_map(|stmt| {
            if stmt.predicate == *NODE_RDFA_COPY_PREDICATE {
                Some(stmt.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    // patch where there is a copy pattern predicate

    for Statement {
        subject, object, ..
    } in copy_patterns_predicate_subject
    {
        let to_copy = copy_patterns.get(&object).ok_or("missing copy pattern!")?;
        for (pred, obj) in to_copy.iter() {
            triples.push(Statement {
                subject: subject.clone(),
                predicate: pred.clone(),
                object: obj.clone(),
            })
        }
    }

    triples.retain(|stmt| {
        copy_patterns_subject.iter().any(|s| &stmt.subject != s)
            && stmt.predicate != *NODE_RDFA_COPY_PREDICATE
    });
    Ok(())
}
pub fn traverse_element<'a>(
    element_ref: &ElementRef<'a>,
    mut ctx: Context<'a>,
    stmts: &mut Vec<Statement<'a>>,
) -> Result<Option<Node<'a>>, Box<dyn Error>> {
    let elt = element_ref.value();

    // extract attrs
    let vocab = elt
        .attr("vocab")
        .or(ctx.parent.as_ref().and_then(|p| p.vocab.clone()));
    let resource = elt.attr("resource");
    let about = elt.attr("about");
    let property = elt.attr("property");
    let rel = elt.attr("rel");
    let href = elt.attr("href");
    let type_of = elt.attr("typeof");

    ctx.vocab = vocab;

    let predicate = if let Some(property) = property {
        Some(resolve_uri(property, &ctx.vocab, ctx.base, true)?)
    } else {
        None
    };
    let current_node = if let Some(resource) = resource {
        // handle resource case. set the context.
        // if property is present, this becomes an object of the parent.
        let object = resolve_uri(resource, &ctx.vocab, ctx.base, false)?;
        if let Some(predicate) = &predicate {
            let subject = ctx
                .parent
                .as_ref()
                .and_then(|p| p.current_node.clone())
                .ok_or("no parent node")?;
            stmts.push(Statement {
                subject,
                predicate: predicate.clone(),
                object: object.clone(),
            });
        }
        object
    } else if let Some(about) = about {
        // handle about case. set the context.
        // if property is present, children become objects of current.
        let subject = resolve_uri(about, &ctx.vocab, ctx.base, false)?;

        if let Some(predicate) = &predicate {
            stmts.push(Statement {
                subject: subject.clone(),
                predicate: predicate.clone(),
                object: extract_literal(element_ref, &ctx.vocab, ctx.base, true)?,
            })
        }
        subject
    } else if type_of.is_some() {
        let node = Node::BNode(Uuid::new_v4());
        let subject = ctx
            .parent
            .as_ref()
            .and_then(|p| p.current_node.clone())
            .unwrap_or_else(|| Node::BNode(Uuid::new_v4()));
        if let Some(predicate) = &predicate {
            stmts.push(Statement {
                subject: subject.clone(),
                predicate: predicate.clone(),
                object: node.clone(),
            })
        }
        node
    } else {
        let subject = ctx
            .parent
            .as_ref()
            .and_then(|p| p.current_node.clone())
            .unwrap_or_else(|| Node::BNode(Uuid::new_v4()));

        if let Some(predicate) = &predicate {
            stmts.push(Statement {
                subject: subject.clone(),
                predicate: predicate.clone(),
                object: extract_literal(element_ref, &ctx.vocab, ctx.base, true)?,
            })
        }
        subject
    };

    if let Some(type_of) = type_of {
        stmts.push(Statement {
            subject: current_node.clone(),
            predicate: NODE_NS_TYPE.clone(),
            object: resolve_uri(type_of, &ctx.vocab, ctx.base, false)?,
        })
    }
    ctx.current_node = Some(current_node);

    if element_ref.has_children() {
        let mut child_ctx = Context {
            parent: Some(Rc::new(ctx.clone())),
            base: ctx.base,
            ..Default::default()
        };
        for child in element_ref.children() {
            if let Some(c) = ElementRef::wrap(child) {
                traverse_element(&c, child_ctx.clone(), stmts)?;
            } else if let Some(text) = child.value().as_text() {
                // do smth with text
            }
        }
    }
    Ok(ctx.current_node.clone())
}

fn extract_literal<'a>(
    element_ref: &ElementRef<'a>,
    vocab: &Option<&'a str>,
    base: &'a str,
    is_property: bool,
) -> Result<Node<'a>, &'static str> {
    let elt_val = element_ref.value();
    let datatype = elt_val
        .attr("datatype")
        .map(|dt| match resolve_uri(dt, vocab, base, false) {
            Ok(d) => Some(Box::new(d)),
            Err(e) => {
                eprintln!("could not parse {dt}. error {e}");
                None
            }
        })
        .flatten(); //todo lang

    if let Some(href) = elt_val.attr("href") {
        resolve_uri(href, vocab, base, false)
    } else if let Some(content) = elt_val.attr("content") {
        Ok(Node::Literal(Literal {
            datatype,
            value: Cow::Borrowed(content),
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
            lang: None,
        }))
    }
}

fn resolve_uri<'a>(
    uri: &'a str,
    vocab: &Option<&'a str>,
    base: &'a str,
    is_property: bool,
) -> Result<Node<'a>, &'static str> {
    let iri = Url::parse(uri);
    match iri {
        Ok(iri) if !iri.cannot_be_a_base() || iri.is_special() => Ok(Node::Iri(Cow::Borrowed(uri))),

        // Curie
        Ok(iri) => {
            if uri.starts_with("mail") || uri.starts_with("tel") {
                Ok(Node::Iri(Cow::Borrowed(uri)))
            }
            // todo handle other cases
            else if let Some(value) = COMMON_PREFIXES.get(iri.scheme()) {
                let iri = uri.replace(":", "").replacen(iri.scheme(), value, 1);
                Ok(Node::Iri(Cow::Owned(iri)))
            } else {
                Ok(Node::Iri(Cow::Borrowed("fixme! I'm a prefix")))
            }
        }
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            if let Some(vocab) = vocab {
                if !is_property {
                    Ok(Node::Iri(Cow::Owned([base, uri].join(""))))
                } else {
                    Ok(Node::Iri(Cow::Owned([vocab, uri].join("")))) // todo check if uri with base is
                                                                     // valid
                }
            } else if !is_property {
                // use base for resource
                Ok(Node::Iri(Cow::Owned([base, uri].join(""))))
            } else {
                Err("could not determine base/vocab")
            }
        }
        Err(e) => {
            eprintln!("invalid uri {uri}. error: {e}");
            Err("could not resolve uri")
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // let buf = include_str!("test-page.html");
    // let document = Html::parse_document(buf);
    // let mut graph: RdfaGraph = Default::default();
    //
    // let root = document.root_element();
    //
    // graph.lang = root.value().attr("lang");
    //
    // let body = root.select(select!("body")).last().ok_or("no body")?;
    // dbg!(graph);

    Ok(())
    // let selector = Selector::parse(r#"div[property="prov:value"]"#).unwrap();
    // let mut s = document.select(&selector);
    // dbg!(s.next().unwrap().html());
}
