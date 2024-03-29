use graph_rdfa_processor::RdfaGraph;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn html_to_rdfa(html: &str, base: &str, well_known_prefix: &str) -> String {
    let wkp = {
        let wkp = well_known_prefix.trim();
        if wkp.is_empty() {
            None
        } else {
            Some(wkp)
        }
    };
    RdfaGraph::parse_str(html, base, wkp).unwrap()
}
