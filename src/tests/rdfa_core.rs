use serial_test::serial;
use test_case::test_case;

use super::cmp_files;
const INPUT_OUTPUT_DIR: &str = "examples/rdfa_core";

#[test_case("example002"  ; "2.1 The RDFa Attributes                            : rdfa_core_002 ")]
#[test_case("example081"  ; "8.2 Completing incomplete triples                  : rdfa_core_081 ")]
#[test_case("example082"  ; "8.2 Completing incomplete triples                  : rdfa_core_082 ")]
#[test_case("example083"  ; "8.2 Completing incomplete triples                  : rdfa_core_083 ")]
#[test_case("example084"  ; "8.2 Completing incomplete triples                  : rdfa_core_084 ")]
#[test_case("example088"  ; "8.2 Completing incomplete triples                  : rdfa_core_088 ")]
#[test_case("example091"  ; "8.2 Completing incomplete triples                  : rdfa_core_091 ")]
#[test_case("example094"  ; "8.2 Completing incomplete triples                  : rdfa_core_094 ")]
#[test_case("example094b" ; "8.2 Completing incomplete triples                  : rdfa_core_094b")]
#[test_case("example106"  ; "8.3.1.1.1 Language Tags                            : rdfa_core_106 ")]
#[test_case("example107"  ; "8.3.1.1.1 Language Tags                            : rdfa_core_107 ")]
#[test_case("example107b" ; "8.3.1.1.1 Language Tags                            : rdfa_core_107b")]
#[test_case("example108"  ; "8.3.1.2 Typed Literals                             : rdfa_core_108 ")]
#[test_case("example111"  ; "8.3.1.3 XML Literals                               : rdfa_core_111 ")]
#[test_case("example113"  ; "8.3.1.3 XML Literals                               : rdfa_core_113 ")]
#[test_case("example118"  ; "8.3.2.2 Using @href or @src to set the object      : rdfa_core_118 ")]
#[test_case("example117"  ; "8.3.2.2 Using @href or @src to set the object      : rdfa_core_117 ")]
#[test_case("example123"  ; "8.4 List Generation                                : rdfa_core_123 ")]
#[test_case("example124"  ; "8.4 List Generation                                : rdfa_core_124 ")]
#[test_case("example126"  ; "8.4 List Generation                                : rdfa_core_126 ")]
#[test_case("example126b" ; "8.4 List Generation                                : rdfa_core_126b")]
#[test_case("example127"  ; "8.4 List Generation                                : rdfa_core_127 ")]
#[serial]
fn test(test_name: &str) {
    cmp_files(test_name, INPUT_OUTPUT_DIR, "http://test.org")
}
