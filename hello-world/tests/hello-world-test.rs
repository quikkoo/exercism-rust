extern crate hello_world;

#[test]
fn test_with_empty_name() {
    assert_eq!("Hello, World!", hello_world::greet(""));
}

#[test]
fn test_name_rust() {
    assert_eq!("Hello, Rust!", hello_world::greet("Rust"));
}

#[test]
fn test_name_exercism() {
    assert_eq!("Hello, Exercism!", hello_world::greet("Exercism"));
}
