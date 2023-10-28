#![allow(unused)]
use std::{
    borrow::{BorrowMut, Cow},
    default,
    error::Error,
    rc::Rc,
};

use scraper::{ElementRef, Html, Selector};
use uuid::Uuid;
mod constants;
mod utils;

macro_rules! select {
    ($e:literal) => {
        &Selector::parse($e)?
    };
}

#[derive(Debug, Default)]
struct RdfaGraph<'a> {
    lang: Option<&'a str>,
    statements: Vec<Statement<'a>>,
}

#[derive(Debug, Default, Clone)]
struct Context<'a> {
    vocab: Option<&'a str>,
    type_of: Option<&'a str>,
    parent: Option<Rc<Context<'a>>>,
    current_node: Option<Node<'a>>,
}

#[derive(Debug, Clone)]
pub struct Literal<'a> {
    datatype: Option<Box<Node<'a>>>,
    value: Cow<'a, str>,
    lang: Option<&'a str>,
}

#[derive(Debug, Clone)]
pub enum Node<'a> {
    Iri(&'a str),
    Literal(Literal<'a>),
    Ref(Box<Node<'a>>),
    List(Vec<Node<'a>>),
    BNode(Uuid),
}

#[derive(Debug, Clone)]
pub struct Statement<'a> {
    pub subject: Node<'a>,
    pub predicate: Node<'a>,
    pub object: Node<'a>,
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

fn traverse_element<'a>(
    element_ref: &ElementRef<'a>,
    mut ctx: Context<'a>,
    stmts: &mut Vec<Statement<'a>>,
) -> Result<Option<Node<'a>>, Box<dyn Error>> {
    let elt = element_ref.value();

    // extract attrs
    let vocab = elt.attr("vocab");
    let resource = elt.attr("resource");
    let about = elt.attr("about");
    let property = elt.attr("property");
    let rel = elt.attr("rel");
    let href = elt.attr("href");
    let type_of = elt.attr("typeof");

    ctx.vocab = vocab;
    ctx.type_of = type_of;

    let predicate = if let Some(property) = property {
        Some(resolve_uri(property))
    } else {
        None
    };
    let current_node = if let Some(resource) = resource {
        // handle resource case. set the context.
        // if property is present, this becomes an object of the parent.
        let object = resolve_uri(resource);
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
        let subject = resolve_uri(about);

        if let Some(predicate) = &predicate {
            stmts.push(Statement {
                subject: subject.clone(),
                predicate: predicate.clone(),
                object: extract_literal(element_ref),
            })
        }
        subject
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
                object: extract_literal(element_ref),
            })
        }
        subject
    };

    ctx.current_node = Some(current_node);

    if element_ref.has_children() {
        let mut child_ctx = Context {
            parent: Some(Rc::new(ctx.clone())),
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

fn extract_literal<'a>(element_ref: &ElementRef<'a>) -> Node<'a> {
    let elt_val = element_ref.value();
    let datatype = elt_val
        .attr("datatype")
        .map(|dt| resolve_uri(dt))
        .map(Box::new); //todo lang

    if let Some(href) = elt_val.attr("href") {
        resolve_uri(href)
    } else if let Some(content) = elt_val.attr("content") {
        Node::Literal(Literal {
            datatype,
            value: Cow::Borrowed(content),
            lang: None,
        })
    } else {
        let texts = element_ref.text().collect::<Vec<_>>();
        let text = if texts.is_empty() {
            Cow::Borrowed("")
        } else if texts.len() == 1 {
            Cow::Borrowed(texts[0])
        } else {
            Cow::Owned(texts.iter().map(|t| t.to_string()).collect())
        };
        Node::Literal(Literal {
            datatype,
            value: text,
            lang: None,
        })
    }
}

fn resolve_uri<'a>(uri: &'a str) -> Node<'a> {
    Node::Iri(uri)
}

#[cfg(test)]
mod test {
    use scraper::Html;

    use crate::{traverse_element, Context};

    #[test]
    fn test_example2() {
        let html = include_str!("../examples/example2.html");

        let document = Html::parse_document(html);
        let root = document.root_element();

        let mut stmts = vec![];
        let root_ctx = Default::default();
        traverse_element(&root, root_ctx, &mut stmts).unwrap();
        dbg!(stmts);
    }
    #[test]
    fn test_example4() {
        let html = include_str!("../examples/example4.html");

        let document = Html::parse_document(html);
        let root = document.root_element();

        let mut stmts = vec![];
        let root_ctx = Default::default();
        traverse_element(&root, root_ctx, &mut stmts).unwrap();
        dbg!(stmts);
    }
}
