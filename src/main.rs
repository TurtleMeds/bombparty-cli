use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct Syllable {
    syll: String,
    count: i32,
}

fn main() -> io::Result<()> {
    let mut rng = rand::rng();
    let words = parse_words("full.txt");
    println!("Done parsing words.");
    let syllables = parse_syllables("syllables.csv");
    println!("Done parsing syllables.");
    let mut cur_guess: String = String::new();
    // loop {
    //     println!("an");
    //     io::stdin()
    //         .read_line(&mut cur_guess)
    //         .expect("Failed to read line!");
    //     if cur_guess.trim().to_string() == "antidisestablishmentarianism" {
    //         println!("you got it!");
    //         break;
    //     }
    // }
    Ok(())
}

fn parse_words(file_path: &str) -> io::Result<Vec<String>> {
    let mut word_reader = BufReader::new(File::open(file_path)?);
    let mut cur_word = String::new();
    let mut words: Vec<String> = Vec::new();
    while word_reader.read_line(&mut cur_word)? > 0 {
        words.push(
            cur_word
                .strip_suffix("\n")
                .expect("failed to strip suffix while parsing word: {cur_word}")
                .to_string(),
        );
    }
    Ok(words)
}

fn parse_syllables(file_path: &str) -> io::Result<Vec<Syllable>> {
    let mut syllable_reader = BufReader::new(File::open(file_path)?);
    let mut cur_syllable = String::new();
    let mut sylls: Vec<Syllable> = Vec::new();
    while syllable_reader.read_line(&mut cur_syllable)? > 0 {
        let arr: Vec<&str> = cur_syllable
            .strip_suffix("\n")
            .expect("Error while stripping newline!")
            .split(",")
            .collect();
        // println!("{arr:?}");
        sylls.push(Syllable {
            syll: arr
                .get(1)
                .expect("Error while getting syllable string!")
                .to_string(),
            count: arr
                .get(0)
                .expect("Error while getting count from syllable!")
                .parse()
                .expect("Syllables file contained unexpected syntax!"),
        });
        cur_syllable.clear();
    }
    Ok(sylls)
}
