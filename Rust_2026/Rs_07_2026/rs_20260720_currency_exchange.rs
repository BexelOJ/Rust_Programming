use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let amount: f64 = input.trim().parse().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let rate: f64 = input.trim().parse().unwrap();

    // Calculate and print result
    println!("Result: {:.2}",amount * rate);
}

