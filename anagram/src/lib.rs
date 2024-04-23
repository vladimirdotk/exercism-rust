use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, posible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let mut hs = HashSet::new();

    let sorted_word = sort_word(word);

    for w in posible_anagrams {
        if word.to_lowercase() == w.to_lowercase() {
            continue;
        }
        if sort_word(w) == sorted_word {
            hs.insert(*w);
        }
    }

    hs
}

fn sort_word(word: &str) -> String {
    let mut word_chars = word.to_lowercase().chars().collect::<Vec<char>>();

    word_chars.sort_unstable();

    word_chars.iter().collect()
}
