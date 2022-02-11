use crate::interpreter::ByteCode;
use rstest::*;
#[rstest]
#[case(
    "LOAD_VAL 1
        LOAD_VAL 2
        ADD
        LOAD_VAL 1
        WRITE_VAR \'a\'
        READ_VAR \'a\'
        MULTIPLY
        RETURN_VALUE",
    3
)]
#[case(
    "LOAD_VAL 1
        WRITE_VAR 'x'
        LOAD_VAL 2
        WRITE_VAR 'y'
        READ_VAR 'x'
        LOAD_VAL 1
        ADD
        READ_VAR 'y'
        MULTIPLY
        RETURN_VALUE
",
    4
)]
#[case(
    "LOAD_VAL 1
        WRITE_VAR 'x'
        LOAD_VAL 2
        WRITE_VAR 'y'
        READ_VAR 'x'
        LOAD_VAL 1
        ADD
        LOAD_VAL 1
        ADD
        READ_VAR 'y'
        MULTIPLY
        RETURN_VALUE
",
    6
)]
fn test_bytecode_evaluate(#[case] bytecode: &str, #[case] expected: i64) {
    assert_eq!(ByteCode::evaluate_byte_code(bytecode), expected);
}
