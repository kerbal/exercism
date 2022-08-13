use std::collections::HashSet;

fn get_factors (n: u32) -> Vec<u32> {
    (1..=n).filter(|x| n % x == 0).collect()
}

pub fn raindrops(n: u32) -> String {
    let factors = get_factors(n);
    let factors_set: HashSet<u32> = HashSet::from_iter(factors);

    let mut result = String::from("");

    if factors_set.contains(&3) { result.push_str("Pling") }
    if factors_set.contains(&5) { result.push_str("Plang") }
    if factors_set.contains(&7) { result.push_str("Plong") }

    if result.len() > 0 { return result }

    n.to_string()
}
