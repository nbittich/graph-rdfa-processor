use super::cmp_files_with_extraction;
use crate::HtmlExtraction;

const INPUT_OUTPUT_DIR: &str = "examples/html_extraction";

const SOURCE: &str = "http://www.w3.org/ns/prov#value";
const TARGET: &str = "http://lblod.data.gift/vocabularies/besluit/extractedDecisionContent";

/// Real meeting.burger (Ranst) agenda: two besluiten, each with a `prov:value` holding
/// the whole decision as html. Expected: the usual triples + one
/// `besluit extractedDecisionContent "<inner html>"^^rdf:HTML` per `prov:value`.
#[test]
#[serial_test::serial]
fn meetingburger_agenda_extracts_decision_html() {
    cmp_files_with_extraction(
        "example0001",
        INPUT_OUTPUT_DIR,
        "http://example.com",
        Some(HtmlExtraction {
            source: SOURCE,
            target: TARGET,
        }),
    );
}
