pub fn is_surrogate_pair(code_lead: u32, code_tail: u32) -> bool {
    code_lead >= 0xD800 && code_lead < 0xDC00 && code_tail >= 0xDC00 && code_tail <= 0xE000
}
