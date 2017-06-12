extern crate bob;

#[test]
fn test_should_respond_to_a_statement() {
    assert_eq!("Whatever.", bob::hey("Tom-ay-to, tom-aaaah-to."));
}

#[test]
fn test_should_respond_to_shouting() {
    assert_eq!("Whoa, chill out!", bob::hey("WATCH OUT!"));
}

#[test]
fn test_should_respond_to_questions() {
    assert_eq!("Sure.", bob::hey("Does this cryogenic chamber make me look fat?"));
}

#[test]
fn test_should_respond_to_questions_ending_with_numbers() {
    assert_eq!("Sure.", bob::hey("You are what, like 15?"));
}

#[test]
fn test_should_respond_to_forceful_talking() {
    assert_eq!("Whatever.", bob::hey("Let's go make out behind the gym!"));
}

#[test]
fn test_should_respond_to_acronyms_in_regular_speech() {
    assert_eq!("Whatever.", bob::hey("It's OK if you don't want to go to the DMV."));
}

#[test]
fn test_should_respond_to_forceful_questions_as_shouting() {
    assert_eq!("Whoa, chill out!", bob::hey("WHAT THE HELL WERE YOU THINKING?"));
}

#[test]
fn test_should_respond_to_shouting_with_special_characters() {
    assert_eq!("Whoa, chill out!", bob::hey("ZOMG THE %^*@#$(*^ ZOMBIES ARE COMING!!11!!1!"));
}

#[test]
fn test_should_respond_to_numbers_when_shouting() {
    assert_eq!("Whoa, chill out!", bob::hey("1, 2, 3 GO!"));
}

#[test]
fn test_should_respond_to_numbers_as_speech() {
    assert_eq!("Whatever.", bob::hey("1, 2, 3"));
}

#[test]
fn test_should_respond_to_questions_with_only_numbers() {
    assert_eq!("Sure.", bob::hey("4?"));
}

#[test]
fn test_should_respond_to_shouting_with_no_exclamation_mark() {
    assert_eq!("Whoa, chill out!", bob::hey("I HATE YOU"));
}

#[test]
fn test_should_respond_to_statement_containing_question_mark() {
    assert_eq!("Whatever.", bob::hey("Ending with ? means a question."));
}

#[test]
fn test_should_respond_to_prattling_on() {
    assert_eq!("Sure.", bob::hey("Wait! Hang on. Are you going to be OK?"));
}

#[test]
fn test_should_respond_to_silence() {
    assert_eq!("Fine. Be that way!", bob::hey(""));
}

#[test]
fn test_should_respond_to_prolonged_silence() {
    assert_eq!("Fine. Be that way!", bob::hey("          "));
}

#[test]
fn test_should_respond_to_others_blank_characters() {
    assert_eq!("Fine. Be that way!", bob::hey("\n\r \t\u{000B}\u{00A0}\u{2002}"));
}

#[test]
fn test_should_respond_to_multiple_line_questions() {
    assert_eq!("Whatever.", bob::hey("\nDoes this cryogenic chamber make me look fat?\nno"));
}

#[test]
fn test_should_respond_to_non_letters_with_question() {
    assert_eq!("Sure.", bob::hey(":) ?"));
}
