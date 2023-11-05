use std::{path::PathBuf};

use scraper::Html;
use tortank::turtle::turtle_doc::TurtleDoc;

use crate::{
    constants::{self, BNODE_ID_GENERATOR},
    Context, RdfaGraph,
};

mod earl_html5;
mod rdfa_core;
mod rdfa_primer;

fn cmp_files(test_name: &str, input_output_dir: &str, base: &str) {
    let _ = env_logger::try_init();

    println!("running test {test_name}");
    // reset bnode id generator
    BNODE_ID_GENERATOR.store(1, std::sync::atomic::Ordering::SeqCst);

    let path_buf = PathBuf::from(input_output_dir);
    let path_to_html = path_buf.join(format!("{test_name}.html"));
    if !path_to_html.exists() {
        panic!("{path_to_html:?} does not exist");
    }
    let path_to_ttl = path_buf.join(format!("{test_name}.ttl"));
    if !path_to_ttl.exists() {
        panic!("{path_to_ttl:?} does not exist");
    }
    let html = std::fs::read_to_string(path_to_html).unwrap();
    let html = html.trim_end();
    println!("============ HTML ============");
    println!("{html}");
    let ttl = std::fs::read_to_string(&path_to_ttl).unwrap();

    let ttl = ttl.trim_end();
    println!("============ Expected result ============");
    println!("{ttl}");
    let document = Html::parse_document(html);
    let root = document.root_element();

    let root_ctx = Context {
        base,
        ..Default::default()
    };
    let graph = RdfaGraph::parse(&root, root_ctx).unwrap().to_string();

    println!("============ Actual result ============");
    println!("{graph}");

    // trick to keep the whitespaces at the right place
    // uncomment line below and comment the last line if test doesn't work
    //std::fs::write(path_to_ttl, &graph);

    let ttl =
        TurtleDoc::try_from((ttl, Some(constants::DEFAULT_WELL_KNOWN_PREFIX.to_string()))).unwrap();
    let graph = TurtleDoc::try_from((
        graph.as_str(),
        Some(constants::DEFAULT_WELL_KNOWN_PREFIX.to_string()),
    ))
    .unwrap();
    let diff = ttl.difference(&graph).unwrap();
    //  diff = diff.add(graph.difference(&ttl).unwrap());
    if !diff.is_empty() {
        println!("============ Difference ============");
        println!("{diff}");
    }
    assert!(diff.is_empty());
}
