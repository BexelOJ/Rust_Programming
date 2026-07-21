use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let weight: f64 = input.trim().parse().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let height: f64 = input.trim().parse().unwrap();

    // Calculate and print BMI
    let bmi: f64 = weight / (height * height);
    println!("BMI: {:.1}",bmi);
}

