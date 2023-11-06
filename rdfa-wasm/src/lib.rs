use graph_rdfa_processor::RdfaGraph;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn html_to_rdfa(html: &str, base: &str) -> String {
    RdfaGraph::parse_str(html, base).unwrap()
}
