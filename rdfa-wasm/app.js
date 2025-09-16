async function loadWasmContext() {
  const module = await import("./pkg/rdfa_wasm.js");
  await module.default();
  // return module.compute_as_string; use that one instead to create the heap from javascript
  return { htmlToRdfa: module.html_to_rdfa, rdfaToTurtle :module.rdfa_to_turtle };
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
  const {htmlToRdfa,rdfaToTurtle} = await loadWasmContext();
  toggleForm(form, false);

  // initLoadFromUrl();
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
    let res = htmlToRdfa(
      data.get("html") || "",
      data.get("base") || "",
      data.get("wellKnownPrefix") || "",
    );
    // transform to turtle for better reading. could be just a button.
    let turtle = rdfaToTurtle(res)
    const out = document.querySelector("pre");
    out.innerText = turtle;
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

function initLoadFromUrl() {
  const loadBtn = document.querySelector("#loadFromUrlBtn");
  loadBtn.addEventListener('click', async (e) => {
    e.stopPropagation();
    e.preventDefault();
    const url = document.querySelector('#fromUrl');
    const urlContent = url.value;
    if (urlContent?.length) {
      try {
        const _ = new URL(urlContent);// just to make sure url is valid

        const response = await fetch(urlContent);
        if (response.status === 200) {
          // doesn't work on gh pages because of CORS
          alert(await response.text());
        }
      } catch (e) {
        alert("url seems invalid, check logs");
        console.log(e);
      }

    }
  });
}
const copyToClipboardBtn = document.querySelector("#copyToClipboard");
copyToClipboardBtn.onclick = function (e) {
  e.preventDefault();
  const out = document.querySelector("pre");

  navigator.clipboard.writeText(out.innerText);

  alert("Copied to clipboard");
};

run();

