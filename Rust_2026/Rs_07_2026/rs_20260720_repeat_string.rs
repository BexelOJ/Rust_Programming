use std::io;

fn main() {
    let mut word = String::new();
    io::stdin().read_line(&mut word).unwrap();
    let word = word.trim();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Print the word repeated n times
    for _ in 0..n{
    	print!("{}",word);
    }

    println!("");
}

