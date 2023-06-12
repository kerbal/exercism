/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.chars().any(|c| !(c.is_ascii_digit() || c.is_whitespace())) { return false }
    if code.chars().into_iter().filter(|c| { c.is_ascii_digit() }).count() < 2 { return false }

    code
    .chars()
    .filter(|c| c.is_ascii_digit())
    .rev()
    .map(|c| c.to_digit(10).unwrap())
    .enumerate()
    .map(|(index, digit)| {
        println!("{}", digit);
        if index % 2 == 1 {
            if digit * 2 > 9 {
                return digit * 2 - 9
            }
            return digit * 2;
        }
        digit
    }).sum::<u32>() % 10 == 0
}
