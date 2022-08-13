pub fn number_of_digit(num: u32) -> u32 {
  return (num as f64).log10().floor() as u32 + 1;
}

pub fn is_armstrong_number(num: u32) -> bool {
  let n = number_of_digit(num);
  let s = format!("{}", num);
  let arr: Vec<char> = s.chars().collect();
  let sum = arr.iter().fold(0u32, |sum, &s| { sum + u32::pow(s.to_digit(10).unwrap(), n) });
  return sum == num;
}
