use serial_test::serial;
use test_case::test_case;

use super::cmp_files;
const INPUT_OUTPUT_DIR: &str = "examples/rdfa_primer";

#[test_case("example02" ; "2.1.1.1  Hints on Social Networking Sites     : rdfa_primer_02")]
#[test_case("example04" ; "2.1.1.2  Links with Flavor                    : rdfa_primer_04")]
#[test_case("example06" ; "2.1.1.3  Setting a Default Vocabulary         : rdfa_primer_06")]
#[test_case("example07" ; "2.1.1.3  Setting a Default Vocabulary         : rdfa_primer_07")]
#[test_case("example08" ; "2.1.1.3  Setting a Default Vocabulary         : rdfa_primer_08")]
#[test_case("example09" ; "2.1.1.3  Setting a Default Vocabulary         : rdfa_primer_09")]
#[test_case("example10" ; "2.1.1.4  Multiple Items per Page              : rdfa_primer_10")]
#[test_case("example11" ; "2.1.1.4  Multiple Items per Page              : rdfa_primer_11")]
#[test_case("example15" ; "2.1.2.1  Contact Information                  : rdfa_primer_15")]
#[test_case("example17" ; "2.1.2.2  Describing Social Networks           : rdfa_primer_17")]
#[test_case("example18" ; "2.1.2.2  Describing Social Networks           : rdfa_primer_18")]
#[test_case("example19" ; "2.1.2.2  Describing Social Networks           : rdfa_primer_19")]
#[test_case("example20" ; "2.1.2.2  Describing Social Networks           : rdfa_primer_20")]
#[test_case("example22" ; "2.1.3    Repeated Patterns                    : rdfa_primer_22")]
#[test_case("example23" ; "2.1.4    Internal References                  : rdfa_primer_23")]
#[test_case("example24" ; "2.1.4    Internal References                  : rdfa_primer_24")]
#[test_case("example25" ; "2.1.4    Internal References                  : rdfa_primer_25")]
#[test_case("example26" ; "2.1.4    Internal References                  : rdfa_primer_26")]
#[test_case("example27" ; "2.1.5    Using Multiple Vocabularies          : rdfa_primer_27")]
#[test_case("example28" ; "2.1.5    Using Multiple Vocabularies          : rdfa_primer_28")]
#[test_case("example29" ; "2.1.5    Using Multiple Vocabularies          : rdfa_primer_29")]
#[test_case("example30" ; "2.1.5.1  Repeating properties                 : rdfa_primer_30")]
#[test_case("example31" ; "2.1.5.1  Repeating properties                 : rdfa_primer_31")]
#[test_case("example32" ; "2.1.5.1  Repeating properties                 : rdfa_primer_32")]
#[test_case("example33" ; "2.1.5.2  Default Prefixes (Initial Context)   : rdfa_primer_33")]
#[test_case("example35" ; "2.2.1    Using the content attribute          : rdfa_primer_35")]
#[test_case("example36" ; "2.2.1    Using the content attribute          : rdfa_primer_36")]
#[test_case("example39" ; "2.2.2    Datatypes                            : rdfa_primer_39")]
#[test_case("example40" ; "2.2.3    Alternative for setting the context  : rdfa_primer_40")]
#[test_case("example41" ; "2.2.3    Alternative for setting the context  : rdfa_primer_41")]
#[test_case("example42" ; "2.2.3    Alternative for setting the context  : rdfa_primer_42")]
#[test_case("example43" ; "2.2.3    Alternative for setting the context  : rdfa_primer_43")]
#[test_case("example44" ; "2.2.3    Alternative for setting the context  : rdfa_primer_44")]
#[test_case("example45" ; "2.2.4    Alternative for setting the property : rdfa_primer_45")]
#[test_case("example46" ; "2.2.4    Alternative for setting the property : rdfa_primer_46")]
#[serial]
fn test(test_name: &str) {
    cmp_files(test_name, INPUT_OUTPUT_DIR, "http://test.org")
}
