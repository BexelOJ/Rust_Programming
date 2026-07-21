use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    // Count down from n to 1
		for x in (1..n+1).rev() {
    		  println!("{}",x);
	
    }
    // Print Go!
    println!("Go");
}

