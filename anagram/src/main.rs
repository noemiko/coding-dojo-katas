use itertools::Itertools;
use rstest::rstest;

fn sort_letters(word: &str) -> String {
    let mut vector_of_letters = word.chars().collect::<Vec<_>>();
    vector_of_letters.sort_unstable();
    vector_of_letters.iter().collect()
}

fn find_all_two_words_anagrams(anagram_letters: &str, words_db: Vec<String>) -> Vec<Vec<String>> {
    let expected_letters = sort_letters(anagram_letters);

    let mut anagram_words: Vec<Vec<String>> = vec![];

    for vpair in words_db.into_iter().permutations(2) {
        let first_word = vpair.first().unwrap();
        let second_word = vpair.last().unwrap();

        let merged_words = format!("{}{}", first_word, second_word);
        let sorted_letters_from_merged_words = sort_letters(&merged_words);
        if expected_letters == sorted_letters_from_merged_words {
            anagram_words.push(vec![first_word.clone(), second_word.clone()]);
        }
    }
    anagram_words
}

fn main() {
    let anagram_letters = "documenting";
    let words_db: Vec<String> = vec![String::from("gint"), String::from("nemucod")];
    let result = find_all_two_words_anagrams(anagram_letters, words_db);

    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[rstest]
    #[case("yolo",
     vec![String::from("lo"), String::from("yo")], 
     vec![vec![String::from("lo"), String::from("yo")], vec![String::from("yo"), String::from("lo")]])]
    #[case("yolo", vec![String::from("to"), String::from("to")], vec![])]
    fn anagram_test(
        #[case] anagram_letters: &str,
        #[case] words_db: Vec<String>,
        #[case] expected_result: Vec<Vec<String>>,
    ) {
        assert_eq!(
            expected_result,
            find_all_two_words_anagrams(anagram_letters, words_db)
        )
    }
    #[rstest]
    #[case("yolo", String::from("looy"))]
    #[case("documenting", String::from("cdegimnnotu"))]
    fn sorting_words_test(#[case] input: &str, #[case] expected: String) {
        let result = sort_letters(input);
        assert_eq!(expected, result)
    }
}
