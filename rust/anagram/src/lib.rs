use std::collections::HashSet;
use std::collections::HashMap;

pub fn count_char(word: &str) -> HashMap<char, u32> {
    let mut char_count: HashMap<char, u32> = HashMap::new();

    word.to_lowercase().chars().enumerate().for_each(|(_, c)| {
        char_count.entry(c).or_insert(0);
        char_count.insert(c, char_count[&c] + 1);
    });

    char_count
}

pub fn eq_case_insensitive (a: &str, b: &str) -> bool {
    a.to_lowercase().eq(&b.to_lowercase())
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_char_count = count_char(word);

    possible_anagrams
    .iter()
    .filter(|possible_anagram| {
        let char_count = count_char(possible_anagram.to_lowercase().as_str());
        !eq_case_insensitive(word, possible_anagram) && word_char_count.eq(&char_count)
    })
    .cloned()
    .collect()
}
