use std::path::PathBuf;

use crate::{constants::BNODE_ID_GENERATOR, Context, RdfaGraph};
use scraper::Html;
use serial_test::serial;
use test_case::test_case;
const INPUT_OUTPUT_DIR: &str = "examples/earl_html5";

// https://rdfa.info/earl-reports/

#[test_case("example0001"  ; "Predicate establishment with @property                           : earl_reports_html5_0001")]
#[test_case("example0006"  ; "@rel and @rev                                                    : earl_reports_html5_0006")]
#[test_case("example0007"  ; "@rel, @rev, @property, @content                                  : earl_reports_html5_0007")]
#[test_case("example0008"  ; "empty string @about                                              : earl_reports_html5_0008")]
#[serial]
fn test(test_name: &str) {
    let _ = env_logger::try_init();

    println!("running test {test_name}");
    // reset bnode id generator
    BNODE_ID_GENERATOR.store(1, std::sync::atomic::Ordering::SeqCst);

    let path_buf = PathBuf::from(INPUT_OUTPUT_DIR);
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
        base: "http://rdfa.info/test-suite/test-cases/rdfa1.1/html5",
        ..Default::default()
    };
    let graph = RdfaGraph::parse(&root, root_ctx).unwrap().to_string();

    println!("============ Actual result ============");
    println!("{graph}");

    // trick to keep the whitespaces at the right place
    // uncomment line below and comment the last line if test doesn't work
    //std::fs::write(path_to_ttl, graph);
    assert_eq!(ttl, graph);
}
