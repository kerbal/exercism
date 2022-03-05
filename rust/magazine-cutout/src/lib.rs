// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut hm: HashMap<String, i32> = magazine.iter().fold(HashMap::new(), |mut hm, s| {
        *hm.entry(s.to_string()).or_insert(0) += 1;
        return hm;
    });
    hm = note.iter().fold(hm, |mut hm, s| {
        *hm.entry(s.to_string()).or_insert(0) -= 1;
        return hm;
    });
    hm.values().all(|&x| x >= 0)
}
