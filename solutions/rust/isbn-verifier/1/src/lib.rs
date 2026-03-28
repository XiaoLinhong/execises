/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.contains('X') && !isbn.ends_with('X') {
        return false;
    }

    let mut total = 0;
    let mut count = 0;
    for (i, c) in isbn.chars().filter(|&c| c != '-').enumerate() {
        count += 1; 
            match c {
                '0'..='9' => total += (10 - i) * c.to_digit(10).unwrap() as usize,
                'X'       => total += (10 - i) * 10,
                 _        => return false
            }
    }
    total % 11 == 0 && count == 10

}
