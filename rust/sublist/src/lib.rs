// use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    use Comparison::*;

    if a == b { return Equal };
    match (a.len(), b.len()) {
        (0, _) => { return Sublist }
        (_, 0) => { return Superlist },
        (m, n) if (m > n) => if a.windows(n).any(|x| x == b) { return Superlist } else { return Unequal }
        (m, n) if (m < n) => if b.windows(m).any(|x| x == a) { return Sublist } else { return Unequal }
        (_, _) => {
            if a == b { return Equal }
            return Unequal;
        }
    }
}
