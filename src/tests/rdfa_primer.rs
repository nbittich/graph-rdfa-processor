use scraper::Html;
use url::Url;

use crate::{Context, RdfaGraph};

#[test]
fn test_example2() {
    let html = include_str!("../../examples/example02.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let root_ctx = Default::default();
    let stmts = RdfaGraph::parse(&root, root_ctx).unwrap();
    println!("{}", stmts);
}
#[test]
fn test_example4() {
    let html = include_str!("../../examples/example04.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let root_ctx = Default::default();
    let stmts = RdfaGraph::parse(&root, root_ctx).unwrap();

    println!("{}", stmts);
}

#[test]
fn test_example6() {
    let html = include_str!("../../examples/example06.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let root_ctx = Default::default();
    let stmts = RdfaGraph::parse(&root, root_ctx).unwrap();

    println!("{}", stmts);
}

#[test]
fn test_example7() {
    let html = include_str!("../../examples/example07.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let root_ctx = Default::default();
    let stmts = RdfaGraph::parse(&root, root_ctx).unwrap();

    println!("{}", stmts);
}

#[test]
fn test_example8() {
    let html = include_str!("../../examples/example08.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let root_ctx = Default::default();
    let stmts = RdfaGraph::parse(&root, root_ctx).unwrap();

    println!("{}", stmts);
}

#[test]
fn test_example9() {
    let html = include_str!("../../examples/example09.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let root_ctx = Default::default();
    let stmts = RdfaGraph::parse(&root, root_ctx).unwrap();

    println!("{}", stmts);
}

#[test]
fn test_example10() {
    let html = include_str!("../../examples/example10.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let root_ctx = Default::default();
    let stmts = RdfaGraph::parse(&root, root_ctx).unwrap();

    println!("{}", stmts);
}

#[test]
fn test_example11() {
    let html = include_str!("../../examples/example11.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let root_ctx = Default::default();
    let stmts = RdfaGraph::parse(&root, root_ctx).unwrap();

    println!("{}", stmts);
}

#[test]
fn test_example15() {
    let html = include_str!("../../examples/example15.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let root_ctx = Default::default();
    let stmts = RdfaGraph::parse(&root, root_ctx).unwrap();

    println!("{}", stmts);
}

#[test]
fn test_example17() {
    let html = include_str!("../../examples/example17.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let root_ctx = Default::default();
    let stmts = RdfaGraph::parse(&root, root_ctx).unwrap();

    println!("{}", stmts);
}
#[test]
fn test_example18() {
    let html = include_str!("../../examples/example18.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let root_ctx = Default::default();
    let stmts = RdfaGraph::parse(&root, root_ctx).unwrap();

    println!("{}", stmts);
}

#[test]
fn test_example19() {
    let html = include_str!("../../examples/example19.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let root_ctx = Default::default();
    let stmts = RdfaGraph::parse(&root, root_ctx).unwrap();

    println!("{}", stmts);
}

#[test]
fn test_example20() {
    let html = include_str!("../../examples/example20.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let root_ctx = Default::default();
    let stmts = RdfaGraph::parse(&root, root_ctx).unwrap();

    println!("{}", stmts);
}

#[test]
fn test_example22() {
    let html = include_str!("../../examples/example22.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let mut root_ctx = Context {
        base: "http://test.org",
        ..Default::default()
    };
    let stmts = RdfaGraph::parse(&root, root_ctx).unwrap();

    println!("{}", stmts);
}
#[test]
fn test_example23() {
    let html = include_str!("../../examples/example23.html");

    let document = Html::parse_document(html);
    let root = document.root_element();

    let mut root_ctx = Context {
        base: "http://test.org",
        ..Default::default()
    };
    let stmts = RdfaGraph::parse(&root, root_ctx).unwrap();

    println!("{}", stmts);
}
