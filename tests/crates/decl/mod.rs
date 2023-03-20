use std::fmt::Display;

use timpl::*;

#[test]
fn timpl_if() {
    fn template(expr: bool) -> String {
        timpl_if!(expr, {
            body
        })
    }

    assert_eq!(template(true), "body");
    assert_eq!(template(false), "");
}

#[test]
fn timpl_if_else() {
    fn template(expr: bool) -> String {
        timpl_if_else!(expr, {
            body
        }, {
            else
        })
    }

    assert_eq!(template(true), "body");
    assert_eq!(template(false), "else");
}

#[test]
fn timpl_map() {
    fn template(items: impl std::iter::Iterator<Item = impl Display>) -> String {
        timpl_map!(items.into_iter(), item, {
            { item },
        })
    }

    assert_eq!(template(vec![1, 2, 3, 4, 5].iter()), "1,2,3,4,5,");
    assert_eq!(template(vec!['a', 'b', 'c', 'd', 'e'].iter()), "a,b,c,d,e,");
}

#[test]
fn timpl_map_ln() {
    fn template(items: impl std::iter::Iterator<Item = impl Display>) -> String {
        timpl_map_ln!(items.into_iter(), item, {
            { item }
        })
    }

    assert_eq!(template(vec![1, 2, 3, 4, 5].iter()), "1\n2\n3\n4\n5");
    assert_eq!(template(vec!['a', 'b', 'c', 'd', 'e'].iter()), "a\nb\nc\nd\ne");
}
