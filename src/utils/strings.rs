pub fn slice_at_char_boundary(s: &str, byte_index: usize) -> &str {
    let mut last_valid_index = 0;

    for (i, _) in s.char_indices() {
        if i > byte_index {
            break;
        }
        last_valid_index = i;
    }

    &s[..last_valid_index]
}
