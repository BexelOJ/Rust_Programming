use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let first_name = input.trim().to_string();

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let last_name = input.trim().to_string();

    // Create username (lowercase, no space)
    let user_name = first_name.to_lowercase()+&last_name.to_lowercase();
    
    // Create initials (uppercase first letters)
    let initial1 = first_name.chars().next().unwrap().to_ascii_uppercase();
    let initial2 = last_name.chars().next().unwrap().to_ascii_uppercase();

    // Print results
    println!("User Name : {}",user_name);
    println!("Initials: {}{}", initial1, initial2);

}



