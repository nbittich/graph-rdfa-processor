use std::{ops::Add, path::PathBuf};

use scraper::Html;
use tortank::turtle::turtle_doc::TurtleDoc;

use crate::{Context, RdfaGraph, constants::reset_fake_uuid_gen};

mod earl_html5;
mod other;
mod rdfa_core;
mod rdfa_primer;
const DEBUG: bool = true;
const WRITE_RESULT_TO_FILE: bool = true;
const WRITE_DIFF_TO_FILE: bool = true;

const DEFAULT_WELL_KNOWN_PREFIX: &str = "http://data.lblod.info/.well-known/genid#";
fn cmp_files(test_name: &str, input_output_dir: &str, base: &str) {
    let _ = env_logger::try_init();

    println!("running test {test_name}");
    // reset bnode id generator
    reset_fake_uuid_gen();

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

    if DEBUG {
        println!("============ HTML ============");
        println!("{html}");
    }
    let ttl = std::fs::read_to_string(&path_to_ttl).unwrap();

    let ttl = ttl.trim_end();

    let document = Html::parse_document(html);
    let root = document.root_element();

    let empty_ref_node_substitute = "00000000-0000-0000-0000-000000000000";
    let root_ctx = Context {
        base,
        empty_ref_node_substitute,
        ..Default::default()
    };
    let graph = RdfaGraph::parse(&root, root_ctx).unwrap().to_string();

    if WRITE_RESULT_TO_FILE {
        std::fs::write("/tmp/res.ttl", &graph).expect("could not write file");
    }
    let ttl = TurtleDoc::try_from((ttl, Some(DEFAULT_WELL_KNOWN_PREFIX.to_string()))).unwrap();
    if DEBUG {
        println!("============ Expected result ============");
        println!("{ttl}");
    }
    let graph =
        TurtleDoc::try_from((graph.as_str(), Some(DEFAULT_WELL_KNOWN_PREFIX.to_string()))).unwrap();
    if DEBUG {
        println!("============ Actual result ============");
        println!("{graph}");
    }
    let mut diff = ttl.difference(&graph).unwrap();
    diff = diff.add(graph.difference(&ttl).unwrap());
    if !diff.is_empty() && DEBUG {
        println!("============ Difference ============");
        println!("{diff}");
    }
    if WRITE_DIFF_TO_FILE {
        std::fs::write("/tmp/diff.ttl", diff.to_string()).expect("could not write file");
    }
    assert_eq!(0, diff.len());
}
