use serial_test::serial;
use test_case::test_case;

use super::cmp_files;
const INPUT_OUTPUT_DIR: &str = "examples/other";

// https://rdfa.info/earl-reports/

#[test_case("example0001"  ; "Case prefix not scoped properly                                                : other_0001 ")]
#[serial]
fn test(test_name: &str) {
    cmp_files(
        test_name,
        INPUT_OUTPUT_DIR,
        "http://rdfa.info/test-suite/test-cases/rdfa1.1/html5",
    )
}
