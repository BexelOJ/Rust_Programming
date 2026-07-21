use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Print the star triangle
  	for i in 1..n+1 {
            for j in 0..i {
    		print!("*");
            }
            println!();
        }
//     println!();
}

