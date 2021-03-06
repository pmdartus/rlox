extern crate rlox;

use std::{fs, str};

fn load_fixture(path: &str) -> Option<String> {
    fs::read_to_string(path).ok()
}

fn run(in_file: &str) {
    let input = load_fixture(in_file)
        .unwrap_or_else(|| panic!("Expected fixture {} to be present.", in_file));

    let out_file = &format!("{}.out", in_file);
    let expected_out = load_fixture(out_file);
    let mut actual_out = vec![];

    rlox::evaluate(&input, &mut actual_out).unwrap();

    // Generate actual output file is not present.
    if expected_out.is_none() {
        fs::write(out_file, actual_out).unwrap();
        return;
    }

    let actual_out_str = str::from_utf8(&actual_out).unwrap();
    let expected_out_str = expected_out.unwrap();
    assert_eq!(
        actual_out_str, expected_out_str,
        "Actual and expected input don't match"
    );
}

macro_rules! test_fixture {
    ( $($name: ident => $file: expr),* ) => {
        $(
            #[test]
            fn $name() {
                run(&format!("tests/fixtures/{}", $file))
            }
        )*
    }
}

test_fixture! {
    arithmetic => "arithmetic.lox",
    comparison => "comparison.lox",
    variables => "variables.lox",
    scope => "scope.lox"
}
