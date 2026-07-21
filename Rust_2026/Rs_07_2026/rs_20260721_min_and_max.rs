use std::io;
use std::io::BufRead;

fn main() {
	let mut input = String::new();
  	io::stdin().read_line(&mut input).unwrap();
    	let count: i32 = input.trim().parse().unwrap();

  	let mut v = Vec::new();
  
    	for _ in 0..count {
	let mut input = String::new();
  	io::stdin().read_line(&mut input).unwrap();
   	let val: i32 = input.trim().parse().unwrap();
  	v.push(val);  	
    }
    
    // Find and print min and max
    println!("Min: {:?}", v.iter().min().unwrap());
    println!("Max: {:?}", v.iter().max().unwrap());
}

