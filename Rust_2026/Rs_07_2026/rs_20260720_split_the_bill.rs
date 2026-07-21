use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let total: f64 = input.trim().parse().unwrap();
    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num: f64 = input.trim().parse().unwrap();

    // Calculate and print
    println!("Each pays: {:.2}",total/num);
}

