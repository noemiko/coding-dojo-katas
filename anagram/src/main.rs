use itertools::Itertools;
use rstest::rstest;
use std::fs;

fn sort_letters(word: &str) -> String {
    let mut vector_of_letters = word.chars().collect::<Vec<_>>();
    vector_of_letters.sort_unstable();
    vector_of_letters.iter().collect()
}

fn find_all_two_words_anagrams<'a>(anagram_letters: &str, words_db: Vec<&'a str>) -> Vec<Vec<&'a str>> {
    let anagram_letters = sort_letters(anagram_letters);

    let mut anagram_words: Vec<Vec<&str>> = vec![];

    for vpair in words_db.into_iter().permutations(2) {
        let first_word = vpair.first().unwrap();
        let second_word = vpair.last().unwrap();

        let merged_words = format!("{}{}", first_word, second_word);
        let sorted_letters_from_merged_words = sort_letters(&merged_words);
        if anagram_letters == sorted_letters_from_merged_words {
            anagram_words.push(vec![
                first_word,
                second_word,
            ]);
        }
    }
    anagram_words
}

fn main() {
    let anagram_letters = "documenting";
    let file_path = "./word_list.txt";

    let content =
        fs::read_to_string(file_path).expect("File with words should be provided with project.");
    let words_db = content.split_whitespace().collect::<Vec<&str>>();
    let result = find_all_two_words_anagrams(anagram_letters, words_db);

    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[rstest]
    #[case("yolo",
     vec!["lo", "yo"], 
     vec![vec!["lo", "yo"], vec!["yo", "lo"]])]
    #[case("ffff", vec!["ff","ff"], vec![vec!["ff", "ff"], vec!["ff", "ff"]])]
    #[case("yolo", vec!["to","fo"], vec![])]
    fn anagram_test(
        #[case] anagram_letters: &str,
        #[case] words_db: Vec<&str>,
        #[case] expected_result: Vec<Vec<&str>>,
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
