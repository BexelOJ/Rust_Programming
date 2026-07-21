use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num: i32 = input.trim().parse().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let low: i32 = input.trim().parse().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let high: i32 = input.trim().parse().unwrap();

    // Check and print
    if num >= low && num <= high {
    	println!{"In range"};
    } else {
    	println!("Out of range");
    }
}

