use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let a: i32 = input.trim().parse().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let b: i32 = input.trim().parse().unwrap();

    // Print the four operations
   println!("{} + {} = {}", a, b, a + b);
   println!("{} - {} = {}", a, b, a - b);
   println!("{} * {} = {}", a, b, a * b);
   println!("{} / {} = {}", a, b, a / b);
  
}

