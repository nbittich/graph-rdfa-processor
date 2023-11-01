use serial_test::serial;
use test_case::test_case;

use super::cmp_files;
const INPUT_OUTPUT_DIR: &str = "examples/earl_html5";

// https://rdfa.info/earl-reports/

#[test_case("example0001"  ; "Predicate establishment with @property                                 : earl_reports_html5_0001")]
#[test_case("example0006"  ; "@rel and @rev                                                          : earl_reports_html5_0006")]
#[test_case("example0007"  ; "@rel, @rev, @property, @content                                        : earl_reports_html5_0007")]
#[test_case("example0008"  ; "empty string @about                                                    : earl_reports_html5_0008")]
#[test_case("example0009"  ; "@rev                                                                   : earl_reports_html5_0009")]
#[test_case("example0010"  ; "@rel, @rev, @href                                                      : earl_reports_html5_0010")]
#[test_case("example0014"  ; "@datatype, xsd:integer                                                 : earl_reports_html5_0014")]
#[test_case("example0015"  ; "meta and link                                                          : earl_reports_html5_0015")]
#[test_case("example0017"  ; "Related blanknodes                                                     : earl_reports_html5_0017")]
#[test_case("example0018"  ; "@rel for predicate                                                     : earl_reports_html5_0018")]
#[test_case("example0020"  ; "Inheriting @about for subject                                          : earl_reports_html5_0020")]
#[test_case("example0021"  ; "Subject inheritance with no @about                                     : earl_reports_html5_0021")]
#[test_case("example0023"  ; "@id does not generate subjects                                         : earl_reports_html5_0023")]
#[test_case("example0025"  ; "simple chaining test                                                   : earl_reports_html5_0025")]
#[test_case("example0026"  ; "@content                                                               : earl_reports_html5_0026")]
#[test_case("example0027"  ; "@content, ignore element content                                       : earl_reports_html5_0027")]
#[test_case("example0029"  ; "markup stripping with @datatype                                        : earl_reports_html5_0029")]
#[test_case("example0030"  ; "omitted @about                                                         : earl_reports_html5_0030")]
#[test_case("example0031"  ; "simple @resource                                                       : earl_reports_html5_0031")]
#[test_case("example0032"  ; "@resource overrides @href                                              : earl_reports_html5_0032")]
#[test_case("example0033"  ; "simple chaining test with bNode                                        : earl_reports_html5_0033")]
#[test_case("example0034"  ; "simple img[@src] test                                                  : earl_reports_html5_0034")]
#[test_case("example0036"  ; "@src/@resource test                                                    : earl_reports_html5_0036")]
#[test_case("example0038"  ; "@rev - img[@src] test                                                  : earl_reports_html5_0038")]
#[test_case("example0048"  ; "@typeof with @about and @rel present, no @resource                     : earl_reports_html5_0048")]
#[test_case("example0049"  ; "@typeof with @about, no @rel or @resource                              : earl_reports_html5_0049")]
#[test_case("example0050"  ; "@typeof without anything else                                          : earl_reports_html5_0050")]
#[test_case("example0051"  ; "@typeof with a single @property                                        : earl_reports_html5_0051")]
#[test_case("example0052"  ; "@typeof with @resource and nothing else                                : earl_reports_html5_0052")]
#[test_case("example0053"  ; "@typeof with @resource and nothing else, with a subelement             : earl_reports_html5_0053")]
#[test_case("example0054"  ; "multiple @property                                                     : earl_reports_html5_0054")]
#[test_case("example0055"  ; "multiple @rel                                                          : earl_reports_html5_0055")]
#[test_case("example0056"  ; "@typeof applies to @about on same element with hanging rel             : earl_reports_html5_0056")]
#[test_case("example0057"  ; "hanging @rel creates multiple triples                                  : earl_reports_html5_0057")]
#[test_case("example0059"  ; "multiple hanging @rels with multiple children                          : earl_reports_html5_0059")]
#[test_case("example0060"  ; "UTF-8 conformance                                                      : earl_reports_html5_0060")]
#[test_case("example0063"  ; "@rel in head using reserved XHTML value and empty-prefix CURIE syntax  : earl_reports_html5_0063")]
#[test_case("example0064"  ; "@about with safe CURIE                                                 : earl_reports_html5_0064")]
#[test_case("example0065"  ; "@rel with safe CURIE                                                   : earl_reports_html5_0065")]
#[test_case("example0066"  ; "@about with @typeof in the head                                        : earl_reports_html5_0066")]
#[test_case("example0067"  ; "@property in the head                                                  : earl_reports_html5_0067")]
#[test_case("example0068"  ; "Relative URI in @about                                                 : earl_reports_html5_0068")]
#[test_case("example0069"  ; "Relative URI in @href                                                  : earl_reports_html5_0069")]
#[test_case("example0070"  ; "Relative URI in @resource                                              : earl_reports_html5_0070")]
#[test_case("example0071"  ; "No explicit @about                                                     : earl_reports_html5_0071")]
#[test_case("example0072"  ; "Relative URI in @about (with XHTML base in head)                       : earl_reports_html5_0072")]
#[test_case("example0073"  ; "Relative URI in @resource (with XHTML base in head)                    : earl_reports_html5_0073")]
#[test_case("example0074"  ; "Relative URI in @href (with XHTML base in head)                        : earl_reports_html5_0074")]
#[test_case("example0075"  ; "Reserved word 'license' in @rel with no explicit @about                : earl_reports_html5_0075")]
#[test_case("example0080"  ; "@about overrides @resource in incomplete triples                       : earl_reports_html5_0080")]
#[test_case("example0083"  ; "@about overrides @resource in incomplete triples                       : earl_reports_html5_0083")]
#[serial]
fn test(test_name: &str) {
    cmp_files(
        test_name,
        INPUT_OUTPUT_DIR,
        "http://rdfa.info/test-suite/test-cases/rdfa1.1/html5",
    )
}
