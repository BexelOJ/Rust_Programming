use std::io;
use std::f64::consts::PI;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let radius: f64 = input.trim().parse().unwrap();

    // Calculate and print
    let area = PI * radius * radius;  
    let circumference: f64 = 2.00 * PI * radius;

    println!("Area: {:.2}",area);
    println!("Circumference: {:.2}", circumference);
}

