use serial_test::serial;
use test_case::test_case;

use super::cmp_files;
const INPUT_OUTPUT_DIR: &str = "examples/earl_html5";

// https://rdfa.info/earl-reports/

#[test_case("example0001"  ; "Predicate establishment with @property                                                : earl_reports_html5_0001 ")]
#[test_case("example0006"  ; "@rel and @rev                                                                         : earl_reports_html5_0006 ")]
#[test_case("example0007"  ; "@rel, @rev, @property, @content                                                       : earl_reports_html5_0007 ")]
#[test_case("example0008"  ; "empty string @about                                                                   : earl_reports_html5_0008 ")]
#[test_case("example0009"  ; "@rev                                                                                  : earl_reports_html5_0009 ")]
#[test_case("example0010"  ; "@rel, @rev, @href                                                                     : earl_reports_html5_0010 ")]
#[test_case("example0014"  ; "@datatype, xsd:integer                                                                : earl_reports_html5_0014 ")]
#[test_case("example0015"  ; "meta and link                                                                         : earl_reports_html5_0015 ")]
#[test_case("example0017"  ; "Related blanknodes                                                                    : earl_reports_html5_0017 ")]
#[test_case("example0018"  ; "@rel for predicate                                                                    : earl_reports_html5_0018 ")]
#[test_case("example0020"  ; "Inheriting @about for subject                                                         : earl_reports_html5_0020 ")]
#[test_case("example0021"  ; "Subject inheritance with no @about                                                    : earl_reports_html5_0021 ")]
#[test_case("example0023"  ; "@id does not generate subjects                                                        : earl_reports_html5_0023 ")]
#[test_case("example0025"  ; "simple chaining test                                                                  : earl_reports_html5_0025 ")]
#[test_case("example0026"  ; "@content                                                                              : earl_reports_html5_0026 ")]
#[test_case("example0027"  ; "@content, ignore element content                                                      : earl_reports_html5_0027 ")]
#[test_case("example0029"  ; "markup stripping with @datatype                                                       : earl_reports_html5_0029 ")]
#[test_case("example0030"  ; "omitted @about                                                                        : earl_reports_html5_0030 ")]
#[test_case("example0031"  ; "simple @resource                                                                      : earl_reports_html5_0031 ")]
#[test_case("example0032"  ; "@resource overrides @href                                                             : earl_reports_html5_0032 ")]
#[test_case("example0033"  ; "simple chaining test with bNode                                                       : earl_reports_html5_0033 ")]
#[test_case("example0034"  ; "simple img[@src] test                                                                 : earl_reports_html5_0034 ")]
#[test_case("example0035"  ; "@src/@href test                                                                       : earl_reports_html5_0035 ")]
#[test_case("example0036"  ; "@src/@resource test                                                                   : earl_reports_html5_0036 ")]
#[test_case("example0037"  ; "@src/@resource test                                                                   : earl_reports_html5_0037 ")]
#[test_case("example0038"  ; "@rev - img[@src] test                                                                 : earl_reports_html5_0038 ")]
#[test_case("example0039"  ; "@rev - img[@src] test                                                                 : earl_reports_html5_0039 ")]
#[test_case("example0048"  ; "@typeof with @about and @rel present, no @resource                                    : earl_reports_html5_0048 ")]
#[test_case("example0049"  ; "@typeof with @about, no @rel or @resource                                             : earl_reports_html5_0049 ")]
#[test_case("example0050"  ; "@typeof without anything else                                                         : earl_reports_html5_0050 ")]
#[test_case("example0051"  ; "@typeof with a single @property                                                       : earl_reports_html5_0051 ")]
#[test_case("example0052"  ; "@typeof with @resource and nothing else                                               : earl_reports_html5_0052 ")]
#[test_case("example0053"  ; "@typeof with @resource and nothing else, with a subelement                            : earl_reports_html5_0053 ")]
#[test_case("example0054"  ; "multiple @property                                                                    : earl_reports_html5_0054 ")]
#[test_case("example0055"  ; "multiple @rel                                                                         : earl_reports_html5_0055 ")]
#[test_case("example0056"  ; "@typeof applies to @about on same element with hanging rel                            : earl_reports_html5_0056 ")]
#[test_case("example0057"  ; "hanging @rel creates multiple triples                                                 : earl_reports_html5_0057 ")]
#[test_case("example0059"  ; "multiple hanging @rels with multiple children                                         : earl_reports_html5_0059 ")]
#[test_case("example0060"  ; "UTF-8 conformance                                                                     : earl_reports_html5_0060 ")]
#[test_case("example0063"  ; "@rel in head using reserved XHTML value and empty-prefix CURIE syntax                 : earl_reports_html5_0063 ")]
#[test_case("example0064"  ; "@about with safe CURIE                                                                : earl_reports_html5_0064 ")]
#[test_case("example0065"  ; "@rel with safe CURIE                                                                  : earl_reports_html5_0065 ")]
#[test_case("example0066"  ; "@about with @typeof in the head                                                       : earl_reports_html5_0066 ")]
#[test_case("example0067"  ; "@property in the head                                                                 : earl_reports_html5_0067 ")]
#[test_case("example0068"  ; "Relative URI in @about                                                                : earl_reports_html5_0068 ")]
#[test_case("example0069"  ; "Relative URI in @href                                                                 : earl_reports_html5_0069 ")]
#[test_case("example0070"  ; "Relative URI in @resource                                                             : earl_reports_html5_0070 ")]
#[test_case("example0071"  ; "No explicit @about                                                                    : earl_reports_html5_0071 ")]
#[test_case("example0072"  ; "Relative URI in @about (with XHTML base in head)                                      : earl_reports_html5_0072 ")]
#[test_case("example0073"  ; "Relative URI in @resource (with XHTML base in head)                                   : earl_reports_html5_0073 ")]
#[test_case("example0074"  ; "Relative URI in @href (with XHTML base in head)                                       : earl_reports_html5_0074 ")]
#[test_case("example0075"  ; "Reserved word 'license' in @rel with no explicit @about                               : earl_reports_html5_0075 ")]
#[test_case("example0080"  ; "@about overrides @resource in incomplete triples                                      : earl_reports_html5_0080 ")]
#[test_case("example0083"  ; "multiple ways of handling incomplete triples                                          : earl_reports_html5_0083 ")]
#[test_case("example0084"  ; "handling incomplete triples, this time with both @rel and @rev                        : earl_reports_html5_0084 ")]
#[test_case("example0088"  ; "Interpretation of the CURIE (_:)                                                      : earl_reports_html5_0088 ")]
#[test_case("example0089"  ; "@src sets a new subject (@typeof)                                                     : earl_reports_html5_0089 ")]
#[test_case("example0091"  ; "Non-reserved, un-prefixed CURIE in @property                                          : earl_reports_html5_0091 ")]
#[test_case("example0093"  ; "XMLLiteral content with explicit @datatype (user-data-typed literal)                  : earl_reports_html5_0093 ")]
#[test_case("example0099"  ; "Preservation of white space in literals                                               : earl_reports_html5_0099 ")]
#[test_case("example0104"  ; "rdf:value                                                                             : earl_reports_html5_0104 ")]
#[test_case("example0106"  ; "chaining with empty value in inner @rel                                               : earl_reports_html5_0106 ")]
#[test_case("example0107"  ; "no garbage collecting bnodes                                                          : earl_reports_html5_0107 ")]
#[test_case("example0110"  ; "bNode generated even though no nested @about exists                                   : earl_reports_html5_0110 ")]
#[test_case("example0111"  ; "two bNodes generated after three levels of nesting                                    : earl_reports_html5_0111 ")]
#[test_case("example0112"  ; "plain literal with datatype=\"\"                                                      : earl_reports_html5_0112 ")]
#[test_case("example0115"  ; "XML Entities must be supported by RDFa parser                                         : earl_reports_html5_0115 ")]
#[test_case("example0117"  ; "Fragment identifiers stripped from BASE                                               : earl_reports_html5_0117 ")]
#[test_case("example0118"  ; "empty string is not equivalent to NULL - @about                                       : earl_reports_html5_0118 ")]
#[test_case("example0119"  ; "[prefix:] CURIE format is valid                                                       : earl_reports_html5_0119 ")]
#[test_case("example0120"  ; "[:] CURIE format is valid                                                             : earl_reports_html5_0120 ")]
#[test_case("example0122"  ; "[] does not set the object                                                            : earl_reports_html5_0122 ")]
#[test_case("example0126"  ; "Multiple @typeof values                                                               : earl_reports_html5_0126 ")]
#[test_case("example0134"  ; "Uppercase reserved words                                                              : earl_reports_html5_0134 ")]
#[test_case("example0140"  ; "Blank nodes identifiers are not allowed as predicates                                 : earl_reports_html5_0140 ")]
#[test_case("example0174"  ; "Support single character prefix in CURIEs                                             : earl_reports_html5_0174 ")]
#[test_case("example0175"  ; "IRI for @property is allowed                                                          : earl_reports_html5_0175 ")]
#[test_case("example0176"  ; "IRI for @rel and @rev is allowed                                                      : earl_reports_html5_0176 ")]
#[test_case("example0176b" ; "IRI for @rel and @rev is allowed                                                      : earl_reports_html5_0176b")]
#[test_case("example0177"  ; "@prefix                                                                               : earl_reports_html5_0177 ")]
#[test_case("example0178"  ; "@prefix with multiple mappings                                                        : earl_reports_html5_0178 ")]
#[test_case("example0182"  ; "prefix locality                                                                       : earl_reports_html5_0182 ")]
#[test_case("example0186"  ; "@vocab after subject declaration                                                      : earl_reports_html5_0186 ")]
#[test_case("example0187"  ; "@vocab redefinition                                                                   : earl_reports_html5_0187 ")]
#[test_case("example0188"  ; "@vocab only affects predicates                                                        : earl_reports_html5_0188 ")]
#[test_case("example0189"  ; "@vocab overrides default term                                                         : earl_reports_html5_0189 ")]
#[test_case("example0190"  ; "term case insensitivity                                                               : earl_reports_html5_0190 ")]
#[test_case("example0196"  ; "process explicit XMLLiteral                                                           : earl_reports_html5_0196 ")]
#[test_case("example0197"  ; "TERMorCURIEorAbsURI requires an absolute URI                                          : earl_reports_html5_0197 ")]
#[test_case("example0206"  ; "Usage of Initial Context                                                              : earl_reports_html5_0206 ")]
#[test_case("example0207"  ; "Vevent using @typeof                                                                  : earl_reports_html5_0207 ")]
#[test_case("example0213"  ; "Datatype generation for a literal with XML content, version 1.1                       : earl_reports_html5_0213 ")]
#[test_case("example0214"  ; "Root element has implicit @about=\"\"                                                 : earl_reports_html5_0214 ")]
#[test_case("example0216"  ; "Proper character encoding detection in spite of large headers                         : earl_reports_html5_0216 ")]
#[test_case("example0217"  ; "@vocab causes rdfa:usesVocabulary triple to be added                                  : earl_reports_html5_0217 ")]
#[test_case("example0218"  ; "@inlist to create empty list                                                          : earl_reports_html5_0218 ")]
#[test_case("example0219"  ; "@inlist with literal                                                                  : earl_reports_html5_0219 ")]
#[test_case("example0220"  ; "@inlist with IRI                                                                      : earl_reports_html5_0220 ")]
#[test_case("example0221"  ; "@inlist with hetrogenious membership                                                  : earl_reports_html5_0221 ")]
#[test_case("example0224"  ; "@inlist hanging @rel                                                                  : earl_reports_html5_0224 ")]
#[test_case("example0225"  ; "@inlist on different elements with same subject                                       : earl_reports_html5_0225 ")]
#[test_case("example0228"  ; "alternate for test 0040: @rev - @src/@resource test                                   : earl_reports_html5_0228 ")]
#[test_case("example0229"  ; "img[@src] test with omitted @about                                                    : earl_reports_html5_0229 ")]
#[test_case("example0231"  ; "Set image license information                                                         : earl_reports_html5_0231 ")]
#[test_case("example0232"  ; "@typeof with @rel present, no @href, @resource, or @about                             : earl_reports_html5_0232 ")]
#[test_case("example0233"  ; "@typeof with @rel and @resource present, no @about                                    : earl_reports_html5_0233 ")]
#[test_case("example0246"  ; "hanging @rel creates multiple triples, @typeof permutation                            : earl_reports_html5_0246 ")]
#[test_case("example0247"  ; "Multiple incomplete triples                                                           : earl_reports_html5_0247 ")]
#[test_case("example0248"  ; "multiple ways of handling incomplete triples (with @rev)                              : earl_reports_html5_0248 ")]
#[test_case("example0249"  ; "multiple ways of handling incomplete triples (with @rel and @rev);                    : earl_reports_html5_0249 ")]
#[test_case("example0250"  ; "Checking the right behaviour of @typeof with @about, in presence of @property         : earl_reports_html5_0250 ")]
#[test_case("example0251"  ; "lang                                                                                  : earl_reports_html5_0251 ")]
#[test_case("example0252"  ; "lang inheritance                                                                      : earl_reports_html5_0252 ")]
#[test_case("example0253"  ; "plain literal with datatype=\"\" and lang preservation                                : earl_reports_html5_0253 ")]
#[test_case("example0254"  ; "@datatype=\"\" generates plain literal in presence of child nodes                     : earl_reports_html5_0254 ")]
#[test_case("example0255"  ; "lang=\"\" clears language setting                                                     : earl_reports_html5_0255 ")]
#[test_case("example0257"  ; "element with @property and no child nodes generates empty plain literal               : earl_reports_html5_0257 ")]
#[test_case("example0259"  ; "XML+RDFa Initial Context                                                              : earl_reports_html5_0259 ")]
#[test_case("example0261"  ; "White space preservation in XMLLiteral                                                : earl_reports_html5_0261 ")]
#[test_case("example0262"  ; "Predicate with @property, with white spaces before and after the attribute value      : earl_reports_html5_0262 ")]
#[test_case("example0263"  ; "@property appearing on the html element yields the base as the subjects               : earl_reports_html5_0263 ")]
#[test_case("example0264"  ; "@property appearing on the head element gets the subject from parent                  : earl_reports_html5_0264 ")]
#[test_case("example0265"  ; "@property appearing on the head element gets the subject from parent                  : earl_reports_html5_0265 ")]
#[test_case("example0266"  ; "@property without @content or @datatype, typed object set by @href and @typeof        : earl_reports_html5_0266 ")]
#[test_case("example0267"  ; "@property without @content or @datatype, typed object set by @resource and @typeof    : earl_reports_html5_0267 ")]
#[test_case("example0268"  ; "@property without @content or @datatype, typed object set by @src and @typeof         : earl_reports_html5_0268 ")]
#[test_case("example0269"  ; "Use of @property in HEAD without explicit subject                                     : earl_reports_html5_0269 ")]
#[test_case("example0271"  ; "Use of @property in HEAD with explicit parent subject via @about                      : earl_reports_html5_0271 ")]
#[test_case("example0272"  ; "time element with @datetime an xsd:date                                               : earl_reports_html5_0272 ")]
#[test_case("example0273"  ; "time element with @datetime an xsd:time                                               : earl_reports_html5_0273 ")]
#[test_case("example0274"  ; "time element with @datetime an xsd:dateTime                                           : earl_reports_html5_0274 ")]
#[test_case("example0275"  ; "time element with value an xsd:date                                                   : earl_reports_html5_0275 ")]
#[test_case("example0276"  ; "time element with value an xsd:time                                                   : earl_reports_html5_0276 ")]
#[test_case("example0277"  ; "time element with value an xsd:dateTime                                               : earl_reports_html5_0277 ")]
#[test_case("example0278"  ; "@content overrides @datetime                                                          : earl_reports_html5_0278 ")]
#[test_case("example0279"  ; "@datatype used with @datetime overrides default datatype                              : earl_reports_html5_0279 ")]
#[test_case("example0281"  ; "time element with @datetime an xsd:gYear                                              : earl_reports_html5_0281 ")]
#[test_case("example0282"  ; "time element with @datetime an xsd:gYearMonth                                         : earl_reports_html5_0282 ")]
#[test_case("example0283"  ; "time element with @datetime an invalid datatype generates plain literal               : earl_reports_html5_0283 ")]
#[test_case("example0284"  ; "time element not matching datatype but with explicit @datatype                        : earl_reports_html5_0284 ")]
#[test_case("example0287"  ; "time element with @datetime an xsd:dateTime with TZ offset                            : earl_reports_html5_0287 ")]
#[test_case("example0287b" ; "time element with @datetime an xsd:dateTime with TZ offset                            : earl_reports_html5_0287b")]
#[test_case("example0289"  ; "@href becomes subject when @property and @content are present                         : earl_reports_html5_0289 ")]
#[test_case("example0290"  ; "@href becomes subject when @property and @datatype are present                        : earl_reports_html5_0290 ")]
#[test_case("example0291"  ; "@href as subject overridden by @about                                                 : earl_reports_html5_0291 ")]
#[test_case("example0292"  ; "@about overriding @href as subject is used as parent resource                         : earl_reports_html5_0292 ")]
#[test_case("example0293"  ; "Testing the ':' character usage in a CURIE                                            : earl_reports_html5_0293 ")]
#[test_case("example0296"  ; "@property does set parent object without @typeof                                      : earl_reports_html5_0296 ")]
#[test_case("example0297"  ; "@about=[] with @typeof does not create a new subject                                  : earl_reports_html5_0297 ")]
#[test_case("example0298"  ; "@about=[] with @typeof does not create a new object                                   : earl_reports_html5_0298 ")]
#[test_case("example0299"  ; "@resource=[] with @href or @src uses @href or @src (@rel)                             : earl_reports_html5_0299 ")]
#[test_case("example0300"  ; "@resource=[] with @href or @src uses @href or @src (@property                         : earl_reports_html5_0300 ")]
#[test_case("example0301"  ; "@property with @typeof creates a typed_resource for chaining                          : earl_reports_html5_0301 ")]
#[test_case("example0302"  ; "@typeof with different content types                                                  : earl_reports_html5_0302 ")]
#[test_case("example0303"  ; "For HTML+RDFa, remove term elements of @rel/@rev when on same element as @property    : earl_reports_html5_0303 ")]
#[test_case("example0311"  ; "Ensure no triples are generated when @property is empty                               : earl_reports_html5_0311 ")]
#[test_case("example0312"  ; "Mute plain @rel if @property is present                                               : earl_reports_html5_0312 ")]
#[test_case("example0315"  ; "@property and @typeof with incomplete triples                                         : earl_reports_html5_0315 ")]
#[test_case("example0316"  ; "@property and @typeof with incomplete triples (href variant)                          : earl_reports_html5_0316 ")]
#[test_case("example0317"  ; "@datatype inhibits new @property behavior                                             : earl_reports_html5_0317 ")]
#[test_case("example0318"  ; "Setting @vocab to empty strings removes default vocabulary                            : earl_reports_html5_0318 ")]
#[test_case("example0321"  ; "rdfa:copy to rdfa:Pattern                                                             : earl_reports_html5_0321 ")]
#[test_case("example0322"  ; "rdfa:copy for additional property value                                               : earl_reports_html5_0322 ")]
#[test_case("example0323"  ; "Multiple references to rdfa:Pattern                                                   : earl_reports_html5_0323 ")]
#[test_case("example0324"  ; "Multiple references to rdfa:Pattern                                                   : earl_reports_html5_0324 ")]
#[test_case("example0325"  ; "Multiple references to rdfa:Pattern creating a resource                               : earl_reports_html5_0325 ")]
#[test_case("example0326"  ; "rdfa:Pattern removed only if referenced                                               : earl_reports_html5_0326 ")]
#[test_case("example0327"  ; "rdfa:Pattern chaining                                                                 : earl_reports_html5_0327 ")]
#[test_case("example0328"  ; "@content overrides the content of the time element.                                   : earl_reports_html5_0328 ")]
#[test_case("example0329"  ; "Recursive triple generation                                                           : earl_reports_html5_0329 ")]
#[test_case("example0330"  ; "@datatype overrides inherited @lang                                                   : earl_reports_html5_0330 ")]
#[test_case("example0331"  ; "@datatype overrides inherited @lang, with @content                                    : earl_reports_html5_0331 ")]
#[test_case("example0332"  ; "Empty @datatype doesn't override inherited @lang, with @content                       : earl_reports_html5_0332 ")]
#[test_case("example0333"  ; "@content overrides @datetime (with @datatype specified)                               : earl_reports_html5_0333 ")]
#[test_case("example0334"  ; "@resource changes the current subject for the nested elements                         : earl_reports_html5_0334 ")]
#[serial]
fn test(test_name: &str) {
    cmp_files(
        test_name,
        INPUT_OUTPUT_DIR,
        "http://rdfa.info/test-suite/test-cases/rdfa1.1/html5",
    )
}
