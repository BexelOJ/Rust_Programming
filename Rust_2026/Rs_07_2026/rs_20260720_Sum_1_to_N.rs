use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    // Calculate and print the sum
    let mut sum = 0;
    for i in 0..n+1 {
	 sum += i;
    }
    println!("Sum: {}", sum);
}

