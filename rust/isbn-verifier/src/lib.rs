/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = isbn
        .chars()
        .filter(|c| c.is_numeric() || *c == 'X')
        .collect::<Vec<char>>();
    if isbn.len() != 10 {
        return false;
    }

    isbn.iter()
        .filter(|c| c.is_numeric())
        .take(9)
        .chain(isbn.iter().filter(|c| c.is_numeric() || **c == 'X').nth(9))
        .map(|c| match c.is_digit(10) {
            true => c.to_digit(10).expect("infallible"),
            false => 10,
        })
        .enumerate()
        .map(|(i, d)| d * (10 - i as u32))
        .sum::<u32>()
        % 11
        == 0
}
