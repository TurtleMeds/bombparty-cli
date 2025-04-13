use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let mut sylls: HashMap<String, i32> = HashMap::new();
    let words = File::open("full.txt")?;
    let sylls = File::open("syllables.csv")?;
    let mut cur_guess: String = String::new();
    loop {
        println!("an");
        io::stdin()
            .read_line(&mut cur_guess)
            .expect("Failed to read line!");
        if cur_guess.trim().to_string() == "antidisestablishmentarianism" {
            println!("you got it!");
            break;
        }
    }
    Ok(())
}
