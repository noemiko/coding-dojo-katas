use rstest::rstest;

fn find_all_two_words_anagrams(anagram_letters: String, words_db: Vec<String>) -> Vec<String> {
    vec![String::from("yolo")]
}

fn main() {
    let anagram_letters = String::from("documenting");
    let words_db: Vec<String> = vec![String::from("lo"), String::from("yo")];
    find_all_two_words_anagrams(anagram_letters, words_db);
}

#[rstest]
#[case(String::from("yolo"), vec![String::from("lo"), String::from("yo")], vec![String::from("lo"), String::from("yo")])]
#[case(String::from("yolo"), vec![String::from("yo"), String::from("lo")], vec![String::from("yo"), String::from("lo")])]
fn anagram_test(
    #[case] anagram_letters: String,
    #[case] words_db: Vec<String>,
    #[case] expected_result: Vec<String>,
) {
    assert_eq!(
        expected_result,
        find_all_two_words_anagrams(anagram_letters, words_db)
    )
}
