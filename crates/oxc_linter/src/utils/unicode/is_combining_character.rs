use regex::Regex;

pub fn is_combining_character(character: char) -> bool {
    let is : Regex = Regex::new(r"^[\p{Mc}\p{Me}\p{Mn}]$").unwrap();
    is.is_match(&character.to_string())
}
#[test]
fn test(){
    assert!(!is_combining_character('A'));
    assert!(is_combining_character('\u{0300}'));
    assert!(is_combining_character('\u{0301}'));
    assert!(is_combining_character('\u{0331}'));
    assert!(is_combining_character('\u{0345}'));
    assert!(is_combining_character('\u{0364}'));
}