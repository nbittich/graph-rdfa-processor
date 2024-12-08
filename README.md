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

    RdfaGraph::parse_str(html, base, well_known_prefix).unwrap()

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
