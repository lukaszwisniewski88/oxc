pub fn is_regional_indicator_symbol(code_point: u32) -> bool {
    code_point >= 0x1F1E6 && code_point <= 0x1F1FF
}
