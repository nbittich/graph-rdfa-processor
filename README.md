# RDFa processor library (WIP)

Rust and wasm library to extract rdf statements (in n-triples format) from an html file
based on rdfa annotations.

## Examples

### Rust usage:

```rust
use graph_rdfa_processor::RdfaGraph;
    let html = r#"
    <div prefix="foaf: http://xmlns.com/foaf/0.1/" about="http://www.example.org/#somebody" rel="foaf:knows">
        <p about="http://danbri.org/foaf.rdf#danbri" typeof="foaf:Person" property="foaf:name">Dan Brickley</p>
	  </div>
    "#;
    let base = "http://example.com";
    let well_known_prefix = Some("http://example.org/.well_known");

    RdfaGraph::parse_str(html, base, well_known_prefix, None).unwrap()

```

### Node usage:

`npm i @nbittich/rdfa-wasm`

```js
const { html_to_rdfa } = require("@nbittich/rdfa-wasm");

const example = `
           
<!doctype html>
<html>
  <head>
    <title>Test 0224</title>
  </head>
  <body>
    <div about="">
      <ol rel="rdf:value" inlist="">
        <li>git remote add origin git@github.com:nbittich/node-rdfa-wasm-example.git<a href="foo">Foo</a></li>
        <li><a href="bar">Bar</a></li>
      </ol>
    </div>
  </body>
</html>

`;

console.log(html_to_rdfa(example, "http://data.lblod.info", ""));
```

### Extracting the html behind a property (optional)

Given a source and a target predicate, the inner html of every element carrying
`property=<source>` is additionally stored under `<target>` as an `rdf:HTML` literal
(the original `<source>` triple is kept).

```js
const nt = html_to_rdfa(
  html,
  "http://data.lblod.info",
  "",
  undefined, // optional uuid function
  "http://www.w3.org/ns/prov#value", // html_source_property
  "http://lblod.data.gift/vocabularies/besluit/extractedDecisionContent", // html_target_property
);
// <subject> <http://lblod.data.gift/vocabularies/besluit/extractedDecisionContent> """<p>..</p>"""^^<http://www.w3.org/1999/02/22-rdf-syntax-ns#HTML>.
```

Both properties must be given together (giving only one throws). In Rust, use
`RdfaGraph::parse_str_with_html_extraction(html, base, wkp, None, Some(HtmlExtraction { source, target }))`.

### Web usage (not published on npm yet):

```js
 <script type="module">
    import init, {html_to_rdfa} from "./rdfa-wasm/pkg/rdfa_wasm.js";

    async function run() {
      await init();
      let html =`
        <div prefix="foaf: http://xmlns.com/foaf/0.1/" about="http://www.example.org/#somebody" rel="foaf:knows">
            <p about="http://danbri.org/foaf.rdf#danbri" typeof="foaf:Person" property="foaf:name">Dan Brickley</p>
	      </div>
      `;
      let base = "http://example.com";
      let well_known_prefix = "http://example.org/.well_known";
      let res = html_to_rdfa(html, base, well_known_prefix);

    }
    run();
  </script>

```

- covers:

  - [RDFa 1.1 Primer - Third Edition](https://www.w3.org/TR/rdfa-primer/)
  - [RDFa Core](https://www.w3.org/TR/rdfa-core/)
  - [Earl-Reports](https://rdfa.info/earl-reports/#RDFa-rdfa1.1-tests-for-html5)

- used [RDFa/Play](https://rdfa.info/play/) for comparing.
- [Demo](https://nbittich.github.io/graph-rdfa-processor/)
