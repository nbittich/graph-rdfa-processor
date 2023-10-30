use std::path::PathBuf;

use crate::{constants::BNODE_ID_GENERATOR, Context, RdfaGraph};
use scraper::Html;
use serial_test::serial;
use test_case::test_case;
const INPUT_OUTPUT_DIR: &str = "examples/rdfa_core";

#[test_case("example002"  ; "2.1 The RDFa Attributes                            : rdfa_core_002")]
#[test_case("example081"  ; "8.2 Completing incomplete triples                  : rdfa_core_081")]
#[test_case("example082"  ; "8.2 Completing incomplete triples                  : rdfa_core_082")]
#[test_case("example083"  ; "8.2 Completing incomplete triples                  : rdfa_core_083")]
#[test_case("example084"  ; "8.2 Completing incomplete triples                  : rdfa_core_084")]
#[test_case("example088"  ; "8.2 Completing incomplete triples                  : rdfa_core_088")]
#[test_case("example091"  ; "8.2 Completing incomplete triples                  : rdfa_core_091")]
#[test_case("example094"  ; "8.2 Completing incomplete triples                  : rdfa_core_094")]
#[test_case("example094b" ; "8.2 Completing incomplete triples                  : rdfa_core_094b")]
#[test_case("example106"  ; "8.3.1.1.1 Language Tags                            : rdfa_core_106")]
#[test_case("example107"  ; "8.3.1.1.1 Language Tags                            : rdfa_core_107")]
#[test_case("example107b" ; "8.3.1.1.1 Language Tags                            : rdfa_core_107b")]
#[test_case("example108"  ; "8.3.1.2 Typed Literals                             : rdfa_core_108")]
#[test_case("example111"  ; "8.3.1.3 XML Literals                               : rdfa_core_111")]
#[test_case("example113"  ; "8.3.1.3 XML Literals                               : rdfa_core_113")]
#[test_case("example118"  ; "8.3.2.2 Using @href or @src to set the object      : rdfa_core_118")]
#[test_case("example117"  ; "8.3.2.2 Using @href or @src to set the object      : rdfa_core_117")]
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
        base: "http://test.org",
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
