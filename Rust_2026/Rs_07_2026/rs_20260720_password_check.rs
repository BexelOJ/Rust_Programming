use std::io;

fn main() {
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    // Check length and print
  	let len = password.len();
  	if len >= 8 {
    	println!("Valid");
    } else {
    	println!("Invalid");
    }
}

