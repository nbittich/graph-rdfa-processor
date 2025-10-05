use scraper::Html;
use tortank::{turtle::turtle_doc::TurtleDoc, utils::DEFAULT_WELL_KNOWN_PREFIX};

use crate::{RdfaGraph, constants::reset_fake_uuid_gen, structs::Context};

#[test]
#[serial_test::serial]
pub fn test_host_instead_of_base() {
    let example = r#"
        <!DOCTYPE html>
        <html lang="en">
          <body prefix="dct: http://purl.org/dc/terms/ eli: http://data.europa.eu/eli/ontology# prov: http://www.w3.org/ns/prov# mandaat: http://data.vlaanderen.be/ns/mandaat# persoon: https://data.vlaanderen.be/ns/persoon# org: http://www.w3.org/ns/org# besluit: http://data.vlaanderen.be/ns/besluit# skos: http://www.w3.org/2004/02/skos/core#" vocab="http://data.vlaanderen.be/ns/besluit#">
             <div typeof="besluit:BehandelingVanAgendapunt" resource='ranst.meetingburger.net/rmw/09795852-b9a1-4389-b391-d4bac55627a0#puntbehandelingc9ecabeb-930b-4a4c-8f89-39f79eea98a8'>
             </div>
          </body>
        </html>
    "#;
    let _ = env_logger::try_init();
    // reset bnode id generator
    reset_fake_uuid_gen();

    let document = Html::parse_document(example);
    let root = document.root_element();

    let empty_ref_node_substitute = "00000000-0000-0000-0000-000000000000";
    let root_ctx = Context {
        base: "https://ranst.meetingburger.net/rmw/09795852-b9a1-4389-b391-d4bac55627a0/agenda",
        empty_ref_node_substitute,
        ..Default::default()
    };
    let graph = RdfaGraph::parse(&root, root_ctx).unwrap().to_string();

    let expected = TurtleDoc::try_from((r#"
        <https://ranst.meetingburger.net/rmw/09795852-b9a1-4389-b391-d4bac55627a0/agenda> <http://www.w3.org/ns/rdfa#usesVocabulary> <http://data.vlaanderen.be/ns/besluit#>.
        <https://ranst.meetingburger.net/rmw/09795852-b9a1-4389-b391-d4bac55627a0#puntbehandelingc9ecabeb-930b-4a4c-8f89-39f79eea98a8> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://data.vlaanderen.be/ns/besluit#BehandelingVanAgendapunt>.

    "#, Some(DEFAULT_WELL_KNOWN_PREFIX.to_string()))).unwrap();
    let actual =
        TurtleDoc::try_from((graph.as_str(), Some(DEFAULT_WELL_KNOWN_PREFIX.to_string()))).unwrap();
    assert!(expected.difference(&actual).unwrap().is_empty());
}
