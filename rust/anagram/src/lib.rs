use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let mut word_key = word.chars().collect::<Vec<char>>();
    word_key.sort_unstable();

    possible_anagrams
        .iter()
        .filter(|a| a.len() == word.len()) // filter out words with non-matching lengths as they cannot be anagrams
        .map(|a| (a, a.to_lowercase())) // create a (word, key) pair, the key will be used to detect the anagrams
        .filter(|(_, k)| *k != word) // filter out words that are the same as the input (case insensitive)
        .map(|(a, k)| (a, k.chars().collect::<Vec<char>>())) // Transform the key for this possible anagram into a vec
        .map(|(a, mut k)| {
            k.sort_unstable(); // and sort it
            (a, k)
        })
        .filter(|(_, k)| word_key == *k) // compare the keys, anagrams will end up as the same sorted vec
        .map(|(a, _)| *a) // we only want the input words, we can toss the key
        .collect()
}
