#[cfg(feature = "grapheme")]
pub fn reverse(input: &str) -> String {
    unicode_segmentation::UnicodeSegmentation::graphemes(input, true)
        .rev()
        .collect()
}

#[cfg(not(feature = "grapheme"))]
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
