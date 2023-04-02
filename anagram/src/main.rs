use itertools::Itertools;
use rstest::rstest;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn sort_letters(word: &str) -> String {
    let mut vector_of_letters: Vec<char> = word.chars().collect();
    vector_of_letters.sort_unstable();
    vector_of_letters.iter().collect()
}

fn get_anagrams_by_sorting<'a>(anagram_letters: &str, words_db: Vec<&'a str>) -> Vec<Vec<&'a str>> {
    let anagram_letters = sort_letters(anagram_letters);

    let mut anagram_words: Vec<Vec<&str>> = vec![];

    for vpair in words_db.into_iter().unique().permutations(2) {
        let first_word = vpair.first().unwrap();
        let second_word = vpair.last().unwrap();

        let merged_words = format!("{}{}", first_word, second_word);
        if anagram_letters.len() != merged_words.len() {
            continue;
        }
        let sorted_letters_from_merged_words = sort_letters(&merged_words);
        if anagram_letters == sorted_letters_from_merged_words {
            anagram_words.push(vec![first_word, second_word]);
        }
    }
    anagram_words
}

fn get_letters_mapped_to_primes() -> HashMap<char, u128> {
    let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let primes = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101,
    ];
    let mut map = HashMap::new();

    for (i, letter) in letters.iter().enumerate() {
        map.insert(*letter, primes[i]);
    }
    map
}

fn map_word_to_primes(word: &str, mapped_letters_to_primes: &HashMap<char, u128>) -> Vec<u128> {
    word.chars()
        .filter_map(|c| mapped_letters_to_primes.get(&c).copied())
        .collect()
}

fn get_anagrams_by_primes<'a>(anagram_letters: &str, words_db: Vec<&'a str>) -> Vec<Vec<&'a str>> {
    let mapped_letters_to_primes = get_letters_mapped_to_primes();
    let expected_letters_as_primes = map_word_to_primes(anagram_letters, &mapped_letters_to_primes);
    let expected_multiplication: u128 = expected_letters_as_primes.iter().product();

    let mut anagram_words: Vec<Vec<&str>> = vec![];

    for first_word in words_db.iter().unique().cloned() {
        let first_word_as_primes = map_word_to_primes(first_word, &mapped_letters_to_primes);

        for second_word in words_db.iter().unique().cloned() {
            let merged_words = first_word.to_owned() + second_word;
            if anagram_letters.len() != merged_words.len() {
                continue;
            }
            let second_word_as_primes = map_word_to_primes(&second_word, &mapped_letters_to_primes);
            let two_words_as_primes = first_word_as_primes
                .iter()
                .chain(second_word_as_primes.iter())
                .cloned()
                .collect::<Vec<u128>>();
            let word_sum: u128 = two_words_as_primes.iter().product();

            if expected_multiplication == word_sum {
                anagram_words.push(vec![first_word, second_word]);
            }
        }
    }
    anagram_words
}

fn find_anagrams<'a>(
    word: &str,
    words_db: Vec<&'a str>,
    algorithm_name: &str,
) -> Vec<Vec<&'a str>> {
    if algorithm_name == "by_primes" {
        get_anagrams_by_primes(word, words_db)
    } else {
        get_anagrams_by_sorting(word, words_db)
    }
}

fn main() {
    let anagram_letters = "documenting";
    let file_path = "./word_list.txt";

    let content =
        fs::read_to_string(file_path).expect("File with words should be provided with project.");
    let words_db = content.split_whitespace().collect::<Vec<&str>>();

    for algorithm_name in vec!["by_sorting", "by_primes"] {
        let time_of_sorting_algorithm = Instant::now();
        let result = find_anagrams(anagram_letters, words_db.clone(), algorithm_name);
        let elapsed = time_of_sorting_algorithm.elapsed();
        println!("{} elapsed: {:.2?}", algorithm_name, elapsed);
        println!("{:?}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[rstest]
    // BY SORTING
    #[case("yolo",
           vec!["lo", "yo"], 
           vec![vec!["lo", "yo"], vec!["yo", "lo"]])]
    // not matching words
    #[case("anything", vec!["sport","suzuku"], vec![])]
    // matching with not mathching
    #[case("anything", vec!["thing","sport","suzuku", "any"], vec![vec!["thing", "any"], vec!["any", "thing"],])]
    // duplications of words
    #[case("oyol", vec!["yo","lo", "lo"], vec![vec!["yo", "lo"], vec!["lo", "yo"],]) ]
    #[case("lollol", vec!["lol","lol"], vec![])]
    
    fn anagram_test(
        #[case] anagram_letters: &str,
        #[case] words_db: Vec<&str>,
        #[case] expected_result: Vec<Vec<&str>>,
    ) {
        for algorithm_name in vec!["by_sorting", "by_primes"] {
            println!("{}", algorithm_name);
            assert_eq!(
                expected_result,
                find_anagrams(anagram_letters, words_db.clone(), algorithm_name)
            )
        }
    }
    #[rstest]
    #[case("yolo", String::from("looy"))]
    #[case("documenting", String::from("cdegimnnotu"))]
    fn sorting_words_test(#[case] input: &str, #[case] expected: String) {
        let result = sort_letters(input);
        assert_eq!(expected, result)
    }
}
