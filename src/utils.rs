pub fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

pub fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || c.is_digit(10)
}
