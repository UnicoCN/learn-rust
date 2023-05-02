// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 26;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn vec2str(v :&Vec<char>) -> String {
    let mut str = String::new();
    for i in v.iter() {
        str.push(*i);
    }
    str
}

fn hashset2str(s :&HashSet<char>) -> String {
    let mut str = String::new();
    for i in s.iter() {
        str.push(*i);
    }
    str
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    let mut my_guess: Vec<char> = Vec::new();
    let mut have_guessed: HashSet<char> = HashSet::new();
    let mut res = secret_word.len();
    let mut cnt = 0;
    for _ in secret_word_chars.iter() {
        my_guess.push('-');
    }
    while res > 0 && cnt < NUM_INCORRECT_GUESSES {
        println!("The word so far is {0}", vec2str(&my_guess));
        println!("You have guessed the following letters: {0}", hashset2str(&have_guessed));
        println!("You have {0} guesses left", res);
        print!("Please guess a letter: ");

        io::stdout()
        .flush()
        .expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Error reading line.");
        
        if let Some(ch) = guess.chars().next() {
            if !have_guessed.contains(&ch) {
                have_guessed.insert(ch);
            }
            let mut i = 0;
            let mut f = false;
            while i < secret_word_chars.len() {
                if secret_word_chars[i] == ch {
                    my_guess[i] = ch;
                    res -= 1;
                    f = true;
                }
                i += 1;
            }
            cnt += 1;
            if !f {
                println!("Sorry, that letter is not in the word");
            }
        }
        println!("");
    }
    if res == 0 {
        println!("Congratulations you guessed the secret word: {0}!", secret_word);
    } else {
        println!("Sorry, you ran out of guesses!");
    }
}
