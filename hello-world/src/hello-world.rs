pub fn greet(name: &str) -> String {
    match name {
        "" => "Hello, World!".to_string(),
        _  => format!("Hello, {}!", name),
    }
}
