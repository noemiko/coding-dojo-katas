fn find_all_two_words_anagrams(anagram_letters: String, words_db: Vec<String>) -> Vec<String> {
    vec![String::from("yolo")]
}

fn main() {
    let anagram_letters = String::from("documenting");
    let words_db: Vec<String> = vec![String::from("lo"), String::from("yo")];
    find_all_two_words_anagrams(anagram_letters, words_db);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify() {
        let anagram_letters = String::from("yolo");
        let words_db: Vec<String> = vec![String::from("lo"), String::from("yo")];

        let result = find_all_two_words_anagrams(anagram_letters, words_db);
        assert_eq!(result, vec![String::from("lo"), String::from("yo")]);
    }
}
