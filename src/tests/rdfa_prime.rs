use scraper::Html;
use url::Url;

use crate::{traverse_element, Context, RdfaGraph};

#[test]
fn test_example2() {
    let html = include_str!("../../examples/example2.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let mut stmts = vec![];
    let root_ctx = Default::default();
    traverse_element(&root, root_ctx, &mut stmts).unwrap();
    println!("{}", RdfaGraph(stmts));
}
#[test]
fn test_example4() {
    let html = include_str!("../../examples/example4.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let mut stmts = vec![];
    let root_ctx = Default::default();
    traverse_element(&root, root_ctx, &mut stmts).unwrap();

    println!("{}", RdfaGraph(stmts));
}

#[test]
fn test_example6() {
    let html = include_str!("../../examples/example6.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let mut stmts = vec![];
    let root_ctx = Default::default();
    traverse_element(&root, root_ctx, &mut stmts).unwrap();

    println!("{}", RdfaGraph(stmts));
}

#[test]
fn test_example7() {
    let html = include_str!("../../examples/example7.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let mut stmts = vec![];
    let root_ctx = Default::default();
    traverse_element(&root, root_ctx, &mut stmts).unwrap();

    println!("{}", RdfaGraph(stmts));
}

#[test]
fn test_example8() {
    let html = include_str!("../../examples/example8.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let mut stmts = vec![];
    let root_ctx = Default::default();
    traverse_element(&root, root_ctx, &mut stmts).unwrap();

    println!("{}", RdfaGraph(stmts));
}

#[test]
fn test_example9() {
    let html = include_str!("../../examples/example9.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let mut stmts = vec![];
    let root_ctx = Default::default();
    traverse_element(&root, root_ctx, &mut stmts).unwrap();

    println!("{}", RdfaGraph(stmts));
}

#[test]
fn test_example10() {
    let html = include_str!("../../examples/example10.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let mut stmts = vec![];
    let root_ctx = Default::default();
    traverse_element(&root, root_ctx, &mut stmts).unwrap();

    println!("{}", RdfaGraph(stmts));
}

#[test]
fn test_example11() {
    let html = include_str!("../../examples/example11.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let mut stmts = vec![];
    let root_ctx = Default::default();
    traverse_element(&root, root_ctx, &mut stmts).unwrap();

    println!("{}", RdfaGraph(stmts));
}

#[test]
fn test_example15() {
    let html = include_str!("../../examples/example15.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let mut stmts = vec![];
    let root_ctx = Default::default();
    traverse_element(&root, root_ctx, &mut stmts).unwrap();

    println!("{}", RdfaGraph(stmts));
}
