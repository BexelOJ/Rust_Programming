use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let sentence = input.trim();

    // Count and print the number of words
    println!("{} words", sentence.split_whitespace().count());
}


