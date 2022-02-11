use crate::interpreter_with_loops::ByteCodeWithLoops;
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
LOOP_START 10
        LOAD_VAL 1
        ADD
LOOP_END
        RETURN_VALUE
",
    16
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
LOOP_START 8
        LOAD_VAL 1
        ADD
LOOP_END
LOOP_START 2
        LOAD_VAL 2
        ADD
LOOP_END

        RETURN_VALUE
",
    18
)]
fn test_bytecode_evaluate(#[case] bytecode: &str, #[case] expected: i64) {
    assert_eq!(ByteCodeWithLoops::evaluate_byte_code(bytecode), expected);
}
