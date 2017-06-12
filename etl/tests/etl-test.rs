extern crate etl;

use std::collections::HashMap;

#[test]
fn test_should_transform_one_value() {
    let actual = from_actual(&[
        (1, vec!("WORLD")),
    ]);
    let expected = from_expected(&[("world", 1)]);

    assert_eq!(etl::transform(&actual), expected);
}

#[test]
fn test_should_transform_more_values() {
    let actual = from_actual(&[
        (1, vec!("WORLD", "GSCHOOLERS")),
    ]);
    let expected = from_expected(&[
        ("world", 1),
        ("gschoolers", 1),
    ]);

    assert_eq!(etl::transform(&actual), expected);
}

#[test]
fn test_should_transform_more_keys() {
    let actual = from_actual(&[
        (1, vec!("APPLE", "ARTICHOKE")),
        (2, vec!("BOAT", "BALLERINA")),
    ]);
    let expected = from_expected(&[
        ("apple", 1),
        ("artichoke", 1),
        ("boat",  2),
        ("ballerina", 2),
    ]);

    assert_eq!(etl::transform(&actual), expected);
}

#[test]
fn test_should_transform_full_dataset() {
    let actual = from_actual(&[
        (1,  vec!("A", "E", "I", "O", "U", "L", "N", "R", "S", "T")),
        (2,  vec!("D", "G")),
        (3,  vec!("B", "C", "M", "P")),
        (4,  vec!("F", "H", "V", "W", "Y")),
        (5,  vec!("K")),
        (8,  vec!("J", "X")),
        (10, vec!("Q", "Z")),
    ]);
    let expected = from_expected(&[
        ("a", 1), ("b",  3), ("c", 3), ("d", 2), ("e", 1),
        ("f", 4), ("g",  2), ("h", 4), ("i", 1), ("j", 8),
        ("k", 5), ("l",  1), ("m", 3), ("n", 1), ("o", 1),
        ("p", 3), ("q", 10), ("r", 1), ("s", 1), ("t", 1),
        ("u", 1), ("v",  4), ("w", 4), ("x", 8), ("y", 4),
        ("z", 10),
    ]);

    assert_eq!(etl::transform(&actual), expected);
}

fn from_actual<'s>(coll: &[(i32, Vec<&'s str>)]) -> HashMap<i32, Vec<&'s str>> {
    coll.iter()
        .cloned()
        .collect()
}

fn from_expected(coll: &[(&str, i32)]) -> HashMap<String, i32> {
    coll.iter()
        .map(|&(key, val)| (key.to_string(), val))
        .collect()
}
