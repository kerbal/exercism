pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![];

    (2..).filter(|&x| {
        let is_prime = !primes.iter().any(|&prime| x % prime == 0);
        if is_prime {
            primes.push(x);
            return true
        }
        false
    }).nth(n as usize).unwrap()
}
