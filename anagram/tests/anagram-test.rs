extern crate anagram;

#[test]
fn test_should_detect_no_matches() {
    let candidates = ["hello", "world", "zombies", "pants"];
    let expected: Vec<&str> = vec![];
    assert_eq!(anagram::matches("diaper", &candidates), expected);
}

#[test]
fn test_should_detect_simple_anagram() {
    let candidates = ["tan", "stand", "at"];
    let expected: Vec<&str> = vec!["tan"];
    assert_eq!(anagram::matches("ant", &candidates), expected);
}

#[test]
fn test_should_detect_multiple_anagrams() {
    let candidates = ["stream", "pigeon", "maters"];
    let expected: Vec<&str> = vec!["stream", "maters"];
    assert_eq!(anagram::matches("master", &candidates), expected);
}

#[test]
fn test_should_not_confuse_different_duplicates() {
    let candidates = ["eagle"];
    let expected: Vec<&str> = vec![];
    assert_eq!(anagram::matches("galea", &candidates), expected);
}

#[test]
fn test_should_not_include_identical_words() {
    let candidates = ["corn", "dark", "Corn", "rank", "CORN", "cron", "park"];
    let expected: Vec<&str> = vec!["cron"];
    assert_eq!(anagram::matches("corn", &candidates), expected);
}

#[test]
fn test_should_eliminate_anagram_subsets() {
    let candidates = ["dog", "goody"];
    let expected: Vec<&str> = vec![];
    assert_eq!(anagram::matches("good", &candidates), expected);
}

#[test]
fn test_should_eliminate_anagram_with_same_checksum() {
    let candidates = ["last"];
    let expected: Vec<&str> = vec![];
    assert_eq!(anagram::matches("mass", &candidates), expected);
}

#[test]
fn test_should_detect_anagram() {
    let candidates = ["enlists", "google", "inlets", "banana"];
    let expected: Vec<&str> = vec!["inlets"];
    assert_eq!(anagram::matches("listen", &candidates), expected);
}

#[test]
fn test_should_detect_more_multiple_anagrams() {
    let candidates = ["gallery", "ballerina", "regally", "clergy", "largely", "leading"];
    let expected: Vec<&str> = vec!["gallery", "regally", "largely"];
    assert_eq!(anagram::matches("allergy", &candidates), expected);
}

#[test]
fn test_should_treat_subject_anagrams_as_case_insensitive() {
    let candidates = ["cashregister", "carthorse", "radishes"];
    let expected: Vec<&str> = vec!["carthorse"];
    assert_eq!(anagram::matches("Orchestra", &candidates), expected);
}

#[test]
fn test_should_treat_candidate_anagrams_as_case_insensitive() {
    let candidates = ["Cashregister", "Carthorse", "Radishes"];
    let expected: Vec<&str> = vec!["Carthorse"];
    assert_eq!(anagram::matches("orchestra", &candidates), expected);
}
