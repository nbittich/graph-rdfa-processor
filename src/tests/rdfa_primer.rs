use std::path::PathBuf;

use crate::{constants::BNODE_ID_GENERATOR, Context, RdfaGraph};
use scraper::Html;
use serial_test::serial;
use test_case::test_case;
const INPUT_OUTPUT_DIR: &str = "examples/rdfa_primer";

#[test_case("example02" ; "2.1.1.1  Hints on Social Networking Sites: rdfa_primer_02")]
#[test_case("example04" ; "2.1.1.2  Links with Flavor               : rdfa_primer_04")]
#[test_case("example06" ; "2.1.1.3  Setting a Default Vocabulary    : rdfa_primer_06")]
#[test_case("example07" ; "2.1.1.3  Setting a Default Vocabulary    : rdfa_primer_07")]
#[test_case("example08" ; "2.1.1.3  Setting a Default Vocabulary    : rdfa_primer_08")]
#[test_case("example09" ; "2.1.1.3  Setting a Default Vocabulary    : rdfa_primer_09")]
#[test_case("example10" ; "2.1.1.4  Multiple Items per Page         : rdfa_primer_10")]
#[test_case("example11" ; "2.1.1.4  Multiple Items per Page         : rdfa_primer_11")]
#[test_case("example15" ; "2.1.2.1  Contact Information             : rdfa_primer_15")]
#[test_case("example17" ; "2.1.2.2  Describing Social Networks      : rdfa_primer_17")]
#[test_case("example18" ; "2.1.2.2  Describing Social Networks      : rdfa_primer_18")]
#[test_case("example19" ; "2.1.2.2  Describing Social Networks      : rdfa_primer_19")]
#[test_case("example20" ; "2.1.2.2  Describing Social Networks      : rdfa_primer_20")]
#[test_case("example22" ; "2.1.3    Repeated Patterns               : rdfa_primer_22")]
#[test_case("example23" ; "2.1.4    Internal References             : rdfa_primer_23")]
#[test_case("example24" ; "2.1.4    Internal References             : rdfa_primer_24")]
#[test_case("example25" ; "2.1.4    Internal References             : rdfa_primer_25")]
#[test_case("example26" ; "2.1.4    Internal References             : rdfa_primer_26")]
#[test_case("example27" ; "2.1.5    Using Multiple Vocabularies     : rdfa_primer_27")]
#[test_case("example28" ; "2.1.5    Using Multiple Vocabularies     : rdfa_primer_28")]
#[test_case("example29" ; "2.1.5    Using Multiple Vocabularies     : rdfa_primer_29")]
#[serial]
fn test(test_name: &str) {
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
    // std::fs::write(path_to_ttl, graph);
    assert_eq!(ttl, graph);
}
