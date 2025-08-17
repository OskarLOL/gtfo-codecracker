use std::io::{self, Write};
use gtfo_codecracker::{load_words, match_pattern};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let words = load_words("gtfo-possible-codes.csv")?;
    println!("Loaded {} words.", words.len());
    println!("Type '#' to exit.");

    loop {
        print!("Enter 4-letter pattern (use '-' as wildcard, e.g., a--e): ");
        io::stdout().flush()?;
        let mut pattern = String::new();
        io::stdin().read_line(&mut pattern)?;
        let pattern = pattern.trim().to_lowercase();

        if pattern == "#" {
            println!("Exiting program.");
            break;
        }

        if pattern.len() != 4 {
            println!("Pattern must be exactly 4 letters.");
            continue;
        }

        let invalid_chars: Vec<char> = pattern
            .chars()
            .filter(|c| c != &'-' && !('a'..='z').contains(c))
            .collect();

        if !invalid_chars.is_empty() {
            println!(
                "Invalid characters detected: {}. Only letters a-z or '-' are allowed.",
                invalid_chars.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", ")
            );
            continue;
        }

        let matches = match_pattern(&pattern, &words);
        if matches.is_empty() {
            println!("No matches found.");
        } else {
            println!("Matching words:");
            for word in matches {
                println!("{}", word);
            }
        }
    }

    Ok(())
}
