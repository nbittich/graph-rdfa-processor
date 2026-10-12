use tortank::turtle::turtle_doc::TurtleDoc;

use crate::{HtmlExtraction, RdfaGraph};

const SOURCE: &str = "http://www.w3.org/ns/prov#value";
const TARGET: &str = "http://lblod.data.gift/vocabularies/besluit/extractedDecisionContent";
const BASE: &str = "http://example.com/";

const HTML: &str = r#"
<div prefix="prov: http://www.w3.org/ns/prov#" resource="http://example.com/decision">
  <div property="prov:value"><p>Hello <b>world</b></p></div>
</div>
"#;

// nested blocks, inline formatting, a link, a void element, an entity.
// at most one attribute per element so serialization order can't matter.
const COMPLEX_CONTENT: &str = r#"<h2 class="title">Besluit &amp; motivering</h2><p>Gelet op <a href="http://example.com/wet">de wet</a>, <strong>artikel 1</strong>:</p><ul><li>eerste <em>punt</em></li><li>tweede<br>punt</li></ul><div class="nested"><p>diep <span>genest <b>geneste <i>tekst</i></b></span></p></div>"#;

// whitespace/newlines between tags must be preserved as well
const MULTILINE_CONTENT: &str = r#"<h2 class="title">Besluit</h2>
  <p>Gelet op de wet:</p>
  <ul>
    <li>eerste punt</li>
    <li>tweede punt</li>
  </ul>
<p>Einde</p>"#;

fn wrap(content: &str) -> String {
    format!(
        r#"<div prefix="prov: http://www.w3.org/ns/prov#" resource="http://example.com/decision"><div property="prov:value">{content}</div></div>"#
    )
}

fn html_triple(content: &str) -> String {
    format!(
        r#"<http://example.com/decision> <{TARGET}> """{content}"""^^<http://www.w3.org/1999/02/22-rdf-syntax-ns#HTML>."#
    )
}

fn extract(html: &str) -> String {
    RdfaGraph::parse_str_with_html_extraction(
        html,
        BASE,
        None,
        None,
        Some(HtmlExtraction {
            source: SOURCE,
            target: TARGET,
        }),
    )
    .unwrap()
}

#[test]
#[serial_test::serial]
fn html_extraction_stores_inner_html_as_rdf_html() {
    let res = extract(HTML);

    let expected_str = format!(
        r#"{}
<http://example.com/decision> <http://www.w3.org/ns/prov#value> """Hello world"""."#,
        html_triple("<p>Hello <b>world</b></p>")
    );
    let actual = TurtleDoc::try_from((res.as_str(), None, None)).unwrap();
    let expected = TurtleDoc::try_from((expected_str.as_str(), None, None)).unwrap();

    assert!(
        actual.difference(&expected).unwrap().is_empty(),
        "unexpected triples in output:\n{res}"
    );
    assert!(
        expected.difference(&actual).unwrap().is_empty(),
        "missing triples in output:\n{res}"
    );
}

#[test]
#[serial_test::serial]
fn html_extraction_keeps_the_whole_complex_html() {
    let res = extract(&wrap(COMPLEX_CONTENT));

    // exact, whole inner html as a single rdf:HTML literal
    assert!(res.contains(&html_triple(COMPLEX_CONTENT)), "{res}");

    // same check through the turtle parser, and nothing else was emitted
    // (prov:value + extracted content only)
    let expected_str = html_triple(COMPLEX_CONTENT);
    let actual = TurtleDoc::try_from((res.as_str(), None, None)).unwrap();
    let expected = TurtleDoc::try_from((expected_str.as_str(), None, None)).unwrap();
    assert!(expected.difference(&actual).unwrap().is_empty(), "{res}");
    assert_eq!(actual.len(), 2, "{res}");
}

#[test]
#[serial_test::serial]
fn html_extraction_preserves_multiline_html() {
    let res = extract(&wrap(MULTILINE_CONTENT));
    assert!(res.contains(&html_triple(MULTILINE_CONTENT)), "{res}");
}

#[test]
#[serial_test::serial]
fn html_extraction_is_opt_in() {
    let res = RdfaGraph::parse_str(HTML, BASE, None, None).unwrap();
    assert!(!res.contains(TARGET), "{res}");

    let res = RdfaGraph::parse_str_with_html_extraction(HTML, BASE, None, None, None).unwrap();
    assert!(!res.contains(TARGET), "{res}");
}

#[test]
#[serial_test::serial]
fn html_extraction_ignores_other_properties() {
    let html = r#"
    <div prefix="prov: http://www.w3.org/ns/prov# dct: http://purl.org/dc/terms/" resource="http://example.com/decision">
      <span property="dct:title"><i>title</i></span>
    </div>
    "#;
    let res = extract(html);
    assert!(!res.contains(TARGET), "{res}");
}