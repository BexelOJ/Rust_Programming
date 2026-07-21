use std::io;

fn main() {
    let mut word = String::new();
    io::stdin().read_line(&mut word).unwrap();
    let word = word.trim();

    // Print first and last character
    let first_char = word.chars().next().unwrap();
    let last_char = word.chars().last().unwrap();
  	println!("First: {}", first_char);
    println!("Last: {}", last_char);
}

