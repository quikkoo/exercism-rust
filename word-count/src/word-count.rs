extern crate regex;

use regex::Regex;
use std::collections::HashMap;

pub fn calculate(phrase: &str) -> HashMap<String, i32> {
    let pattern = Regex::new("[']?[^'a-zA-Z0-9]+[']?").unwrap();

    pattern.split(&phrase.to_lowercase())
        .map(|word| word.to_string())
        .fold(HashMap::new(), |mut acc, word| {
            let count = acc.get(&word)
                .unwrap_or(&0) + 1;
            acc.insert(word, count);
            acc
        })
}
