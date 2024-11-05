extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) ->  HashSet<&'a str> {
    let anagram_set: HashSet<&'a str> = possible_anagrams.iter()
                                                             .filter(|anagram| is_anagram(word.to_lowercase(), anagram.to_lowercase()))
                                                             .map(|anagram| *anagram)
                                                             .collect();

    anagram_set
}

fn is_anagram(word: String, possible_anagram: String) -> bool {
    if word == possible_anagram {return false;}
    get_char_count(&word) == get_char_count(&possible_anagram)
}

fn get_char_count<'a>(word: &str) -> HashMap<String, i32> {
    let mut word_freq_map = HashMap::new(); 
    let word_vec: Vec<&str> = UnicodeSegmentation::graphemes(word, true).collect();

    for grapheme in word_vec.iter() {
        *word_freq_map.entry(grapheme.to_string()).or_insert(0) += 1;
    }

    word_freq_map
}