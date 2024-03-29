use core::num;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

fn to_decimal_base (number: &[u32], from_base: u32) -> u32 {
    number
    .into_iter()
    .rev()
    .enumerate()
    .map(|(index, digit)| {
        digit * (from_base.pow(index as u32))
    })
    .sum()
}

fn decimal_to_any_base (number: u32, to_base: u32) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
    let mut temp = number;

    while temp > 0 {
        result.push(temp % to_base);
        temp /= to_base;
    }

    result.reverse();

    result
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 { return Err(Error::InvalidInputBase) }
    if to_base < 2{ return Err(Error::InvalidOutputBase) }
    if let Some(&invalid_digit) = number.into_iter().find(|&&digit| { digit >= from_base }) {
        return Err(Error::InvalidDigit(invalid_digit));
    }
    let number_in_decimal_base = to_decimal_base(number, from_base);
    if number_in_decimal_base == 0 {
        return Ok(vec![0; 1])
    }
    Ok(decimal_to_any_base(number_in_decimal_base, to_base))
}
