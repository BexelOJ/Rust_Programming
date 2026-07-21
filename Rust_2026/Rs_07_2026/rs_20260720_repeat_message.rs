use std::io;

fn main() {
    let mut message = String::new();
    io::stdin().read_line(&mut message).unwrap();
    let message = message.trim();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    // Print the message n times
    for _ in 0..n{
    	println!("{}",message);
    }
}

