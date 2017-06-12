extern crate word_count;

use std::collections::HashMap;

#[test]
fn test_should_count_one_word() {
    let phrase = "word";
    let counts = from_expected(&[("word", 1)]);

    assert_eq!(word_count::calculate(phrase), counts);
}

#[test]
fn test_count_one_of_each() {
    let phrase = "one of each";
    let counts = from_expected(&[
        ("one", 1),
        ("of", 1),
        ("each", 1),
    ]);

    assert_eq!(word_count::calculate(phrase), counts);
}

#[test]
fn test_should_count_multiple_occurrences() {
    let phrase = "one fish two fish red fish blue fish";
    let counts = from_expected(&[
        ("one", 1),
        ("fish", 4),
        ("two", 1),
        ("red", 1),
        ("blue", 1),
    ]);

    assert_eq!(word_count::calculate(phrase), counts);
}

#[test]
fn test_should_count_everything_just_once() {
    let phrase = "all the kings horses and all the kings men";
    let counts = from_expected(&[
        ("all",2),
        ("the", 2),
        ("kings", 2),
        ("horses", 1),
        ("and", 1),
        ("men", 1),
    ]);

    assert_eq!(word_count::calculate(phrase), counts);
}

#[test]
fn test_should_ignore_punctuation() {
    let phrase = "car : carpet as java : javascript!!&@$%^&";
    let counts = from_expected(&[
        ("car", 1),
        ("carpet", 1),
        ("as", 1),
        ("java", 1),
        ("javascript", 1),
    ]);

    assert_eq!(word_count::calculate(phrase), counts);
}

#[test]
fn test_should_handle_cramped_lists() {
    let phrase = "one,two,three";
    let counts = from_expected(&[
        ("one", 1),
        ("two", 1),
        ("three", 1),
    ]);

    assert_eq!(word_count::calculate(phrase), counts);
}

#[test]
fn test_should_include_numbers() {
    let phrase = "testing, 1, 2 testing";
    let counts = from_expected(&[
        ("testing", 2),
        ("1", 1),
        ("2", 1),
    ]);

    assert_eq!(word_count::calculate(phrase), counts);
}

#[test]
fn test_normalize_case() {
    let phrase = "go Go GO";
    let counts = from_expected(&[("go", 3)]);

    assert_eq!(word_count::calculate(phrase), counts);
}

#[test]
fn test_should_allow_apostrophes() {
    let phrase = "First: don't laugh. Then: don't cry.";
    let counts = from_expected(&[
        ("first", 1),
        ("don't", 2),
        ("laugh", 1),
        ("then", 1),
        ("cry", 1),
    ]);

    assert_eq!(word_count::calculate(phrase), counts);
}

#[test]
fn test_should_treat_symbols_as_separators() {
    let phrase = "hey,my_spacebar_is_broken.";
    let counts = from_expected(&[
        ("hey", 1),
        ("my", 1),
        ("spacebar", 1),
        ("is", 1),
        ("broken", 1),
    ]);

    assert_eq!(word_count::calculate(phrase), counts);
}

#[test]
fn test_should_counts_words_with_quotations() {
    let phrase = "Joe can't tell between 'large' and large.";
    let counts = from_expected(&[
        ("joe", 1),
        ("can't", 1),
        ("tell", 1),
        ("between", 1),
        ("large", 2),
        ("and", 1),
    ]);

    assert_eq!(word_count::calculate(phrase), counts);
}


fn from_expected(coll: &[(&str, i32)]) -> HashMap<String, i32> {
    coll.iter()
        .map(|&(key, val)| (key.to_string(), val))
        .collect()
}
