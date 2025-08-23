use std::error::Error;
use eframe::egui::Color32;

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

fn lerp(a: u8, b: u8, t: f32) -> u8 {
    ((a as f32) + (b as f32 - a as f32) * t) as u8
}

/// Blend between yellow and green depending on `num`
/// `num = 1` → pure green
/// `num = 20` → pure yellow
pub fn blend_color(num: usize) -> Color32 {
    let t = ((num.saturating_sub(1)) as f32 / 10.0).clamp(0.0, 1.0);
    // t = 0 → green, t = 1 → yellow

    let green = Color32::from_rgb(0, 255, 0);
    let yellow = Color32::from_rgb(255, 255, 0);

    let r = lerp(green.r(), yellow.r(), t);
    let g = lerp(green.g(), yellow.g(), t);
    let b = lerp(green.b(), yellow.b(), t);

    Color32::from_rgb(r, g, b)
}