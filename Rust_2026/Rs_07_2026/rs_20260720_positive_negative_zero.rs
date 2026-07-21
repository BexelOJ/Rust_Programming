use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    // Check and print
    if n < 0 {
    	println!("Negative");
    } else if n == 0{
    	println!("Zero");
    } else {
    	println!("Positive");
    }
}

