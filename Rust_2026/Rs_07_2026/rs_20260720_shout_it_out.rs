use std::io;

fn main() {
    let mut word = String::new();
    io::stdin().read_line(&mut word).unwrap();
    let word = word.trim();

    // Print uppercase and length
  	println!("{}\nLength: {}",word.to_uppercase(),word.len());
	// println!("{}",word.len());
}

