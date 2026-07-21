use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let a: i32 = input.trim().parse().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let b: i32 = input.trim().parse().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let c: i32 = input.trim().parse().unwrap();

    // Find and print the smallest
  	let small: i32;  
  	if a < b && a < c {
    	   small = a;
        } else if b < a && b < c {
    	   small = b;
        } else {
    	   small = c;
        }
  println!("Smallest: {}", small);
}

