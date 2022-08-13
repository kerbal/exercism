pub fn factors(n: u64) -> Vec<u64> {
    let mut num = 2;
    let mut m = n;
    let mut factors: Vec<u64> = vec![];

    while m > 1 {
        while m % num == 0 {
            factors.push(num);
            m /= num;
        }
        num += 1;
    }

    factors
}
