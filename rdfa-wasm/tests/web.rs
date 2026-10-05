#![cfg(target_arch = "wasm32")]

use rdfa_wasm::{html_to_rdfa, rdfa_to_turtle}; // use your actual crate name
use tortank::turtle::turtle_doc::TurtleDoc;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const BNODE_TTL: &str =
    r#"<https://example.com/a> <https://example.com/knows> [ <https://example.com/name> "Bob" ] ."#;

const HTML: &str = r#"
<div vocab="http://schema.org/" typeof="Person" resource="https://example.com/alice">
  <span property="name">Alice</span>
</div>
"#;

/// Both sides go through the same parser, then must have no statement
/// the other lacks.
fn assert_same_ttl(actual: &str, expected: &str) {
    let actual_doc = TurtleDoc::try_from((actual, None, None))
        .unwrap_or_else(|e| panic!("cannot parse actual: {e}\n{actual}"));
    let expected_doc = TurtleDoc::try_from((expected, None, None))
        .unwrap_or_else(|e| panic!("cannot parse expected: {e}\n{expected}"));

    let missing = expected_doc.difference(&actual_doc).unwrap();
    assert!(
        missing.is_empty(),
        "missing from actual:\n{missing}\n\nactual was:\n{actual}"
    );

    let unexpected = actual_doc.difference(&expected_doc).unwrap();
    assert!(
        unexpected.is_empty(),
        "unexpected in actual:\n{unexpected}\n\nactual was:\n{actual}"
    );

    assert_eq!(actual_doc.len(), expected_doc.len());
}

#[wasm_bindgen_test]
fn rdfa_to_turtle_custom_uuid_fn() {
    let uuid_fn = js_sys::Function::new_no_args("return '1'");

    let ttl = rdfa_to_turtle(BNODE_TTL, Some(uuid_fn));

    assert_same_ttl(
        &ttl,
        r#"
        <https://example.com/a> <https://example.com/knows> _:1 .
        _:1 <https://example.com/name> "Bob" .
        "#,
    );
}

#[wasm_bindgen_test]
fn rdfa_to_turtle_uuid_fn_called_per_blank_node() {
    js_sys::Reflect::set(&js_sys::global(), &"__rdfa_uuid".into(), &0.into()).unwrap();
    let uuid_fn = js_sys::Function::new_no_args(
        "globalThis.__rdfa_uuid += 1; return String(globalThis.__rdfa_uuid)",
    );

    let ttl = rdfa_to_turtle(
        r#"<https://example.com/a> <https://example.com/knows> [ <https://example.com/name> "Bob" ], [ <https://example.com/name> "Eve" ] ."#,
        Some(uuid_fn),
    );

    assert_same_ttl(
        &ttl,
        r#"
        <https://example.com/a> <https://example.com/knows> _:1 , _:2 .
        _:1 <https://example.com/name> "Bob" .
        _:2 <https://example.com/name> "Eve" .
        "#,
    );
}

#[wasm_bindgen_test]
fn rdfa_to_turtle_without_blank_nodes_roundtrips() {
    let input = r#"<https://example.com/a> <https://example.com/name> "Alice" ."#;

    assert_same_ttl(&rdfa_to_turtle(input, None), input);
}

#[wasm_bindgen_test]
fn html_to_rdfa_then_rdfa_to_turtle() {
    let rdfa = html_to_rdfa(HTML, "https://example.com/", "", None);
    let ttl = rdfa_to_turtle(&rdfa, None);

    assert_same_ttl(
        &ttl,
        r#"
        @prefix schema: <http://schema.org/>.
        <https://example.com/> <http://www.w3.org/ns/rdfa#usesVocabulary> schema: .
        <https://example.com/alice> a <http://schema.org/Person> .
        <https://example.com/alice> <http://schema.org/name> "Alice" .
        "#,
    );
}
