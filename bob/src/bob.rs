extern crate regex;

use regex::Regex;

pub fn hey(question: &str) -> &str {
    let fine = Regex::new(r"^[\s\v\x{00a0}\x{2002}]*$").unwrap();
    let whoa = Regex::new(r"^[^\p{Ll}]*[A-Z][^\p{Ll}]*$").unwrap();
    let sure = Regex::new(r"^.*\?[\s\v]*$").unwrap();

    if fine.is_match(question) {
        return "Fine. Be that way!"
    }
    if whoa.is_match(question) {
        return "Whoa, chill out!"
    }
    if sure.is_match(question) {
        return "Sure."
    }
    return "Whatever."
}
