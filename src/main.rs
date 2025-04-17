use colored::Colorize;
use crossterm::style::Stylize;
use crossterm::{self, QueueableCommand};
use rand::seq::IndexedRandom;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::{cmp, str};

struct Syllable {
    syll: String,
    count: i32,
}

#[derive(Debug)]
struct Letter {
    letter: char,
    count: i32,
}

fn main() -> io::Result<()> {
    println!("Parsing words...");
    let words = parse_words("full.txt").expect("Error while parsing words!");
    let mut used_words: Vec<String> = Vec::new();
    println!("Done parsing words.");
    println!("Parsing syllables...");
    let syllables = parse_syllables("syllables.csv").expect("Error while parsing syllables!");
    println!("Done parsing syllables.");
    const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
    let mut alphabet_board: Vec<Letter> = Vec::new();
    for char in ALPHABET.chars().into_iter() {
        alphabet_board.push(Letter {
            letter: (char),
            count: (1),
        });
    }
    let wpp = 500;
    let mut lives = 3;
    let mut cur_guess: String = String::new();
    loop {
        let cur_syllable = choose_syllable(wpp, &syllables);
        println!("{syllable}", syllable = cur_syllable.syll);
        io::stdin()
            .read_line(&mut cur_guess)
            .expect("Failed to read line!");
        cur_guess = cur_guess.trim().to_lowercase();
        if is_guess_valid(&cur_guess, &cur_syllable, &used_words, &words) {
            println!("you got it!");
            used_words.push(cur_guess.to_string());
            update_alphabet(&mut alphabet_board, &cur_guess);
            print_alphabet(&alphabet_board);
        } else {
            lives -= 1;
            println!("KABOOM! you lost a life. now at {lives} lives.");
        }
        if lives <= 0 {
            break;
        }
        cur_guess.clear();
    }
    Ok(())
}

fn is_guess_valid(
    guess: &String,
    syllable: &Syllable,
    used_words: &Vec<String>,
    words: &Vec<String>,
) -> bool {
    words.contains(&guess) && guess.contains(&syllable.syll) && !used_words.contains(&guess)
}

fn update_alphabet(alphabet_board: &mut Vec<Letter>, guess: &String) {
    for char in guess.chars() {
        for letter in &mut *alphabet_board {
            if char == letter.letter {
                letter.count = cmp::max(letter.count - 1, 0);
            }
        }
    }
}

fn print_alphabet(alphabet_board: &Vec<Letter>) {
    let mut to_print = String::new();
    for letter in alphabet_board {
        to_print.push_str(
            format!(
                "{char}",
                char = {
                    if letter.count == 0 {
                        letter.letter.on_black()
                    } else {
                        letter.letter.on_red()
                    }
                }
            )
            .as_str(),
        );
    }
    println!("{to_print}");
}

fn choose_syllable(wpp: i32, syllables: &Vec<Syllable>) -> &Syllable {
    let mut syllable = random_syllable(syllables);
    while syllable.count < wpp {
        syllable = random_syllable(&syllables);
    }
    syllable
}

fn random_syllable(syllables: &Vec<Syllable>) -> &Syllable {
    let mut rng = rand::rng();
    syllables
        .choose(&mut rng)
        .expect("Error while choosing random syllable!")
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
        cur_word.clear();
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
