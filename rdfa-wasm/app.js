async function loadWasmContext() {
  const module = await import("./pkg/rdfa_wasm.js");
  await module.default();
  // return module.compute_as_string; use that one instead to create the heap from javascript
  return module.html_to_rdfa;
}
function toggleForm(form, toggle) {
  const elements = form.elements;
  for (let i = 0, len = elements.length; i < len; ++i) {
    elements[i].readOnly = toggle;
  }
  const submitButton = form.querySelector('button[type="submit"]');
  submitButton.disabled = toggle;
}
async function run() {
  const form = document.querySelector("form");
  toggleForm(form, true);
  const html_to_rdfa = await loadWasmContext();
  toggleForm(form, false);
  const text_area = document.querySelector("#html");
  text_area.value = `
            <!DOCTYPE html>
            <html prefix="foaf: http://xmlns.com/foaf/0.1/">
              <head>
              <title>Test 0083</title>
              </head>
              <body>
             <div about="http://www.example.org/#somebody" rel="foaf:knows">
                 <p property="foaf:name">Ivan Herman</p>
              <p rel="foaf:mailbox" resource="mailto:ivan@w3.org">mailto:ivan@w3.org</p>
              <p about="http://danbri.org/foaf.rdf#danbri" typeof="foaf:Person" property="foaf:name">Dan Brickley</p>
             </div>
              </body>
            </html>
        `;

  form.addEventListener("submit", (e) => {
    e.preventDefault();
    const data = new FormData(e.target);
    let res = html_to_rdfa(
      data.get("html") || "",
      data.get("base") || "",
      data.get("wellKnownPrefix") || "",
    );
    const out = document.querySelector("pre");
    out.innerText = res;
  });
  const issueLink = document.querySelector("#issueLink");
  issueLink.onclick = (e) => {
    e.preventDefault();
    const a = document.createElement("a");
    let params = new URLSearchParams();
    const data = new FormData(form);
    params.append("title", "RDFa processing bug");

    params.append(
      "body",
      `
### Base: 
  
  \`${data.get("base") || ""}\`

### Well Known Prefix:

  \`${data.get("base") || ""}\`

### Html:
  
   \`\`\`html
   ${data.get("html") || ""}
   \`\`\`

          `,
    );
    a.href = `https://github.com/nbittich/graph-rdfa-processor/issues/new?${params.toString()}`;
    a.target = "_blank";
    a.click();
  };
}
const copyToClipboardBtn = document.querySelector("#copyToClipboard");
copyToClipboardBtn.onclick = function (e) {
  e.preventDefault();
  const out = document.querySelector("pre");

  navigator.clipboard.writeText(out.innerText);

  alert("Copied to clipboard");
};

run();

