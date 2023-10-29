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
                writeln!(f, "fixme! format for {e:?} not implemented");
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
        traverse_element(input, initial_context, &mut triples)?;

        triples = copy_pattern(triples)?;
        // copy patterns

        Ok(RdfaGraph(triples))
    }
}

pub fn copy_pattern<'a>(triples: Vec<Statement<'a>>) -> Result<Vec<Statement<'a>>, Box<dyn Error>> {
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
            triples.push(Statement {
                subject: subject.clone(),
                predicate: predicate.clone(),
                object: obj.clone(),
            })
        }
    }
    Ok(triples)
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
        .or(ctx.parent.as_ref().and_then(|p| p.vocab));
    let resource = elt.attr("resource");
    let about = elt.attr("about");
    let property = elt.attr("property");
    let rel = elt.attr("rel");
    let href = elt.attr("href");
    let type_of = elt.attr("typeof");

    ctx.vocab = vocab;

    let predicate = if let Some(property) = property {
        Some(resolve_uri(property, &ctx.vocab, ctx.base, false)?)
    } else {
        None
    };
    let current_node = if let Some(resource) = resource {
        // handle resource case. set the context.
        // if property is present, this becomes an object of the parent.
        let object = resolve_uri(resource, &ctx.vocab, ctx.base, true)?;
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
        let subject = resolve_uri(about, &ctx.vocab, ctx.base, true)?;

        if let Some(predicate) = &predicate {
            stmts.push(Statement {
                subject: subject.clone(),
                predicate: predicate.clone(),
                object: extract_literal(element_ref, &ctx.vocab, ctx.base, true)?,
            })
        }
        subject
    } else if type_of.is_some() {
        // for some reasons it seems that if there is a typeof but no
        // about and no resource, it becomes an anon node
        // this might be incorrect
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

pub fn extract_literal<'a>(
    element_ref: &ElementRef<'a>,
    vocab: &Option<&'a str>,
    base: &'a str,
    is_property: bool,
) -> Result<Node<'a>, &'static str> {
    let elt_val = element_ref.value();
    let datatype =
        elt_val
            .attr("datatype")
            .and_then(|dt| match resolve_uri(dt, vocab, base, true) {
                Ok(d) => Some(Box::new(d)),
                Err(e) => {
                    eprintln!("could not parse {dt}. error {e}");
                    None
                }
            }); //todo lang

    if let Some(href) = elt_val.attr("href") {
        resolve_uri(href, vocab, base, true)
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

pub fn resolve_uri<'a>(
    uri: &'a str,
    vocab: &Option<&'a str>,
    base: &'a str,
    is_resource: bool,
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
                let iri = uri.replace(':', "").replacen(iri.scheme(), value, 1);
                Ok(Node::Iri(Cow::Owned(iri)))
            } else {
                Ok(Node::Iri(Cow::Borrowed("fixme! I'm a prefix")))
            }
        }
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            if is_resource {
                Ok(Node::Iri(Cow::Owned([base, uri].join(""))))
            } else if let Some(vocab) = vocab {
                Ok(Node::Iri(Cow::Owned([vocab, uri].join(""))))
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
