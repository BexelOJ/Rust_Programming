use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    // Extract and print each digit
      let mut dup: i32 = n;
      let ones = dup % 10;
      dup = dup/10;
      let tens = dup % 10;
      dup = dup/10;
      let hun = dup %10;
    println!("Hundreds: {}", hun);
    println!("Tens: {}", tens);
    println!("Ones: {}", ones);   
}

