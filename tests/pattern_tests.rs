use gtfo_codecracker::{load_words_from_str, match_pattern};

// Use the embedded CSV for tests
const CSV_DATA: &str = include_str!("../data/gtfo-possible-codes.csv");

#[test]
fn test_all_words_match_themselves() {
    let words = load_words_from_str(CSV_DATA).expect("Failed to load CSV");

    for word in &words {
        let result = match_pattern(&word, &words);
        assert!(result.contains(word), "Word '{}' should match itself", word);
    }
}

#[test]
fn test_wildcard_matching() {
    let words = load_words_from_str(CSV_DATA).expect("Failed to load CSV");

    for word in words.iter().take(10) {
        let mut pattern = String::from("----");
        pattern.replace_range(0..1, &word[0..1]);
        let result = match_pattern(&pattern, &words);
        assert!(
            result.contains(word),
            "Pattern '{}' should match '{}'",
            pattern,
            word
        );
    }
}

#[test]
fn test_no_match_for_invalid_pattern() {
    let words = load_words_from_str(CSV_DATA).expect("Failed to load CSV");
    let pattern = "zzzz"; // unlikely to exist
    let result = match_pattern(pattern, &words);
    assert!(
        result.is_empty(),
        "Pattern '{}' should not match any word",
        pattern
    );
}

#[test]
fn test_partial_wildcard_matching() {
    let words = load_words_from_str(CSV_DATA).expect("Failed to load CSV");

    for word in words.iter().take(10) {
        if word.len() < 4 {
            continue;
        }
        let mut pattern = String::from("----");
        pattern.replace_range(1..2, &word[1..2]);
        let result = match_pattern(&pattern, &words);
        assert!(
            result.contains(word),
            "Pattern '{}' should match '{}'",
            pattern,
            word
        );
    }
}

#[test]
fn test_full_wildcard_matches_all() {
    let words = load_words_from_str(CSV_DATA).expect("Failed to load CSV");
    let pattern = "----";
    let result = match_pattern(pattern, &words);
    assert_eq!(
        result.len(),
        words.len(),
        "Full wildcard should match all words"
    );
}
