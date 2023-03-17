#[path ="../../../examples/html.rs"]
#[allow(dead_code)]
mod example;

#[test]
fn template() {
    let mut out = example::template();
    out.push('\n');
    assert_eq!(out, include_str!("expected.html"));
}
