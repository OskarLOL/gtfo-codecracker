use std::error::Error;

pub fn load_words_from_str(csv_data: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut words = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(csv_data.as_bytes());

    for result in rdr.records() {
        let record = result?;
        words.push(record[0].to_lowercase());
    }

    Ok(words)
}

pub fn match_pattern(pattern: &str, words: &[String]) -> Vec<String> {
    words
        .iter()
        .filter(|word| {
            word.chars()
                .zip(pattern.chars())
                .all(|(w_c, p_c)| p_c == '-' || w_c == p_c)
        })
        .cloned()
        .collect()
}
