extern crate rlox;

use std::{fs, str};

fn load_fixture(path: &str) -> Option<String> {
    fs::read_to_string(path).ok()
}

fn run(in_file: &str) {
    let input =
        load_fixture(in_file).expect(&format!("Expected fixture {} to be present.", in_file));

    let out_file = &format!("{}.out", in_file);
    let expected_out = load_fixture(out_file);
    let mut actual_out = vec![];

    let res = rlox::evaluate(&input, &mut actual_out);

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
    ($name: ident, $file: expr) => {
        #[test]
        fn $name() {
            run($file)
        }
    };
}

test_fixture!(arithmetic, "tests/fixtures/arithmetic.lox");
test_fixture!(comparison, "tests/fixtures/comparison.lox");
