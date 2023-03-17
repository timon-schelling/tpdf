use timpl_proc::*;

#[test]
fn timpl_line() {
    assert_eq!(timpl! { string }, "string");
}

#[test]
fn timpl_lines() {
    assert_eq!(
        timpl! {
            first
            second
            third
        },
        "first\nsecond\nthird"
    );
}

#[test]
fn timpl_string_arg() {
    assert_eq!(timpl! { {"string"} }, "string");
}

#[test]
fn timpl_string_args() {
    assert_eq!(
        timpl! { {"first"}, {"second"}, {"third"} },
        "first, second, third"
    );
}

#[test]
fn timpl_arg_var() {
    let arg = "string";
    assert_eq!(timpl! { { arg } }, "string");
}

#[test]
fn timpl_args_var() {
    let arg = "string";
    assert_eq!(
        timpl! { { arg }, { arg }, { arg } },
        "string, string, string"
    );
}

#[test]
fn timpl_args_vars() {
    let arg1 = "first";
    let arg2 = "second";
    let arg3 = "third";
    assert_eq!(
        timpl! { { arg1 }, { arg2 }, { arg3 } },
        "first, second, third"
    );
}

#[test]
fn timpl_args_vars_lines() {
    let arg1 = "first";
    let arg2 = "second";
    let arg3 = "third";
    assert_eq!(
        timpl! {
            { arg1 }
            { arg2 }
            { arg3 }
        },
        "first\nsecond\nthird"
    );
}

#[test]
fn timpl_indentation() {
    assert_eq!(
        timpl! {
            first
                second
                    third
                second
            first
        },
        "first\n    second\n        third\n    second\nfirst"
    );
}

#[test]
fn timpl_indentation_string_args() {
    let expected = r"first
    second
    second
        third
            fourth
        third
    second
first
first";

    assert_eq!(
        timpl! {
            {
                "first"
            }
                {
                    "second\nsecond"
                }
                    {
                        "third\n    fourth"
                    }
                    {"third"}
                {"second\nfirst"}
            {"first"}
        },
        expected
    );
}
