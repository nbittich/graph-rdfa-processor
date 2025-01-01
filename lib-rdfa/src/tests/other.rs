use serial_test::serial;
use test_case::test_case;

use super::cmp_files;
const INPUT_OUTPUT_DIR: &str = "examples/other";

#[test_case("example0001"  ; "Case prefix not scoped properly                                                              : other_0001 ")]
#[test_case("example0002"  ; "inlist doesn't behave with base ending with /                                                : other_0002 ")]
#[test_case("example0003"  ; "inlist override base                                                                         : other_0003 ")]
#[test_case("example0004"  ; "whitespace in uri                                                                            : other_0004 ")]
#[test_case("example0005"  ; "other random example from izegem                                                             : other_0005 ")]
#[test_case("example0006"  ; "test with html content                                                                       : other_0006 ")]
#[test_case("example0007"  ; "test with plain literal content                                                              : other_0007 ")]
#[test_case("example0008"  ; "test with plain literal content and href                                                     : other_0008 ")]
#[test_case("example0009"  ; "test with html content and href                                                              : other_0009 ")]
#[test_case("example0010"  ; "Generated by chatgpt #01 (Rel)                                                               : other_0010 ")]
#[test_case("example0011"  ; "Generated by chatgpt #02 (Rel)                                                               : other_0011 ")]
#[test_case("example0012"  ; "Generated by chatgpt #03 (Rel)                                                               : other_0012 ")]
#[test_case("example0013"  ; "Generated by chatgpt #04 (Rel)                                                               : other_0013 ")]
#[test_case("example0014"  ; "Generated by chatgpt #05 (Rel)                                                               : other_0014 ")]
#[test_case("example0015"  ; "Generated by chatgpt #06 (Rel)                                                               : other_0015 ")]
#[test_case("example0016"  ; "Generated by chatgpt #07 (Rev)                                                               : other_0016 ")]
#[test_case("example0017"  ; "Generated by chatgpt #08 (Rev)                                                               : other_0017 ")]
#[test_case("example0018"  ; "Generated by chatgpt #09 (Rel)                                                               : other_0018 ")]
#[test_case("example0019"  ; "Generated by chatgpt #10                                                                     : other_0019 ")]
#[test_case("example0020"  ; "From rdfa.info/play                                                                          : other_0020 ")]
#[test_case("example0021"  ; "From rdfa.info/play with about                                                               : other_0021 ")]
#[test_case("example0022"  ; "From rdfa.info/play product                                                                  : other_0022 ")]
#[test_case("example0023"  ; "From rdfa.info/play places                                                                   : other_0023 ")]
#[test_case("example0024"  ; "From rdfa.info/play event                                                                    : other_0024 ")]
#[test_case("example0025"  ; "From rdfa.info/play social                                                                   : other_0025 ")]
#[test_case("example0026"  ; "From rdfa.info/play person                                                                   : other_0026 ")]
#[test_case("example0027"  ; "Missing link to publication                                                                  : other_0027 ")]
#[test_case("example0028"  ; "Trying our bests to parse the text                                                           : other_0028 ")]
#[serial]
fn test(test_name: &str) {
    cmp_files(
        test_name,
        INPUT_OUTPUT_DIR,
        "http://rdfa.info/test-suite/test-cases/rdfa1.1/html5/",
    )
}
