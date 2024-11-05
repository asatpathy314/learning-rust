extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) ->  HashSet<&'a str> {
    let mut anagram_set: HashSet<&'a str> = HashSet::new();
    let mut word_freq_map = HashMap::new(); 

    let lowercase_word = word.to_lowercase();

    let grapheme_word_vec= UnicodeSegmentation::graphemes(lowercase_word.as_str(), true).collect::<Vec<&str>>();

    for grapheme in grapheme_word_vec.iter() {
        *word_freq_map.entry(grapheme).or_insert(0) += 1;
    }

    for possible_anagram in possible_anagrams.iter() {
        let mut anagram_freq_map = HashMap::new(); 
        let lowercase_anagram= possible_anagram.to_lowercase();
        let grapheme_anagram_vec= UnicodeSegmentation::graphemes(lowercase_anagram.as_str(), true).collect::<Vec<&str>>();

        for grapheme in grapheme_anagram_vec.iter() {
            *anagram_freq_map.entry(grapheme).or_insert(0) += 1;
        }

        println!("anagram-map: {:?}", anagram_freq_map);
        println!("word-map: {:?}", word_freq_map);

        if word_freq_map == anagram_freq_map && lowercase_word != lowercase_anagram{
            println!("added");
            anagram_set.insert(possible_anagram);
        }
    }

    anagram_set
}
