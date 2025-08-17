use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

/// Returns words from CSV file
pub fn load_words(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

    let words: Vec<String> = rdr
        .records()
        .filter_map(|r| r.ok())
        .map(|record| record[0].to_lowercase())
        .collect();

    Ok(words)
}

/// Returns words matching the 4-letter pattern (with '-' as wildcard)
pub fn match_pattern<'a>(pattern: &str, words: &'a [String]) -> Vec<&'a String> {
    words
        .iter()
        .filter(|word| word.chars().zip(pattern.chars()).all(|(w_c, p_c)| p_c == '-' || w_c == p_c))
        .collect()
}
