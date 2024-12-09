use serial_test::serial;
use test_case::test_case;

use super::cmp_files;
const INPUT_OUTPUT_DIR: &str = "examples/other";

#[test_case("example0001"  ; "Case prefix not scoped properly                                                              : other_0001 ")]
#[test_case("example0002"  ; "inlist doesn't behave with base ending with /                                                : other_0002 ")]
#[test_case("example0003"  ; "inlist override base                                                                         : other_0003 ")]
#[test_case("example0004"  ; "whitespace in uri                                                                            : other_0004 ")]
#[test_case("example0005"  ; "other random example from izegem                                                             : other_0005 ")]
#[serial]
fn test(test_name: &str) {
    cmp_files(
        test_name,
        INPUT_OUTPUT_DIR,
        "http://rdfa.info/test-suite/test-cases/rdfa1.1/html5/",
    )
}
