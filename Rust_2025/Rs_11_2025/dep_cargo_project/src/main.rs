use std::io;

mod calculator;
mod printer;
mod greetings;
mod utils;

fn main() {
    // Change greeting message here
    greetings::say_hello("Exin Innvo Labs");

    let a = read_input("Enter first number: ");
    let b = read_input("Enter second number: ");

    let sum = calculator::add(a, b);
    printer::print_sum(sum);

    // Output doubled values for each input
    let double_a = utils::double(a);
    let double_b = utils::double(b);
    println!("The doubled of first value is: {}", double_a);
    println!("The doubled of second value is: {}", double_b);
}

fn read_input(prompt: &str) -> i32 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid integer!"),
        }
    }
}


