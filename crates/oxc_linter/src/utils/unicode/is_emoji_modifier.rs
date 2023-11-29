pub fn is_emoji_modifier(code_point: u32) -> bool {
    code_point >= 0x1F3FB && code_point <= 0x1F3FF
}

#[test]
pub fn test() {
    assert!(is_emoji_modifier(0x1F3FB));
    assert!(is_emoji_modifier(0x1F3FC));
    assert!(is_emoji_modifier(0x1F3FD));
    assert!(is_emoji_modifier(0x1F3FE));
    assert!(is_emoji_modifier(0x1F3FF));
    assert!(!is_emoji_modifier(0x1F3FA));
    assert!(!is_emoji_modifier(0x1F3F0));
    assert!(!is_emoji_modifier(0x1F3F1));
    assert!(!is_emoji_modifier(0x1F3F2));
    assert!(!is_emoji_modifier(0x1F3F3));
    assert!(!is_emoji_modifier(0x1F3F4));
    assert!(!is_emoji_modifier(0x1F3F5));
    assert!(!is_emoji_modifier(0x1F3F6));
    assert!(!is_emoji_modifier(0x1F3F7));
    assert!(!is_emoji_modifier(0x1F3F8));
    assert!(!is_emoji_modifier(0x1F3F9));
    assert!(!is_emoji_modifier(0x1F3FA));
    assert!(!is_emoji_modifier(0x1F3F0));
    assert!(!is_emoji_modifier(0x1F3F1));
    assert!(!is_emoji_modifier(0x1F3F2));
    assert!(!is_emoji_modifier(0x1F3F3));
    assert!(!is_emoji_modifier(0x1F3F4));
    assert!(!is_emoji_modifier(0x1F3F5));
    assert!(!is_emoji_modifier(0x1F3F6));
    assert!(!is_emoji_modifier(0x1F3F7));
    assert!(!is_emoji_modifier(0x1F3F8));
    assert!(!is_emoji_modifier(0x1F3F9));
    assert!(!is_emoji_modifier(0x1F3FA));
    assert!(!is_emoji_modifier(0x1F3F0));
    assert!(!is_emoji_modifier(0x1F3F1));
}
