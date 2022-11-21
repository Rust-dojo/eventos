/*

https://exercism.org/tracks/rust/exercises/anagram

Instructions
An anagram is a rearrangement of letters to form a new word. Given a word and a list of
 candidates, select the sublist of anagrams of the given word.

Given "listen" and a list of candidates like "enlists" "google" "inlets" "banana" the 
program should return a list containing "inlets".

The solution is case insensitive, which means "WOrd" is the same as "word" or "woRd". 
It may help to take a peek at the std library for functions that can convert between them.

The solution cannot contain the input word. A word is always an anagram of itself,
 which means it is not an interesting result. Given "hello" and the list 
 ["hello", "olleh"] the answer is ["olleh"].

You are going to have to adjust the function signature provided in the stub in order 
for the lifetimes to work out properly. This is intentional: what's there demonstrates the basics of lifetime syntax, and what's missing teaches how to interpret lifetime-related compiler errors.

Try to limit case changes. Case changes are expensive in terms of time, so it's faster to minimize them.

If sorting, consider sort_unstable which is typically faster than stable sorting. When applicable, unstable sorting is preferred because it is generally faster than stable sorting and it doesn't allocate auxiliary memory.

*/

pub fn is_anagram(word: &String, candidate: &String) -> bool {
    // candidate.len() == word.len()

    let mut first_word: Vec<char> = word.chars().collect();
    first_word.sort();

    let first_word_str:String = first_word.into_iter().collect();

    let mut candidate_word = candidate.chars().collect::<Vec<char>>();
    candidate_word.sort_unstable();

    let candidate_word_str:String = candidate_word.into_iter().collect();

    candidate_word_str == first_word_str
}

pub fn find_anagrams( word: &String, anagram_candidates: Vec<String>) -> Vec<String>{
    let mut result = vec![];
    for c in anagram_candidates.into_iter() {
        if is_anagram(word, &c) {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_match() {
        let word = "foo".to_string();
        let expected: Vec<String> = vec![];
        let result = find_anagrams(&word, vec!["huebr".to_string()]);
        assert_eq!(result, expected);
    }

    #[test]
    fn one_match() {
        let word = "foo".to_string();
        let candidates = vec!["foo".to_string(), "hue".to_string()];
        let expected: Vec<String> = vec!["foo".to_string()];
        let result = find_anagrams(&word, candidates);
        assert_eq!(result, expected);
    }

}
