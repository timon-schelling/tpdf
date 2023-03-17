use timpl_internal::*;

#[test]
fn indent_n() {
    (1..=10000).for_each(|n| {
        assert_eq!(
            indent(n, "\nstring".to_string()),
            format!("\n{}string", " ".repeat(n))
        )
    });
}

#[test]
fn multi_line_indent_n() {
    (1..=10000).for_each(|n| {
        let indent_str = " ".repeat(n);
        assert_eq!(
            indent(n, "\n1\n    2\n        3\n    2\n1\n".to_string()),
            format!(
                "\n{}1\n{}    2\n{}        3\n{}    2\n{}1\n",
                indent_str, indent_str, indent_str, indent_str, indent_str,
            )
        )
    });
}

#[test]
fn irgnore_first_line_indent_n() {
    (1..=10000).for_each(|n| assert_eq!(indent(n, "string".to_string()), "string"));
}
