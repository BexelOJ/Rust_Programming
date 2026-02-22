use std::io;

fn main() {

let mut input = String::new();

println!("Enter a number: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    println!("Hello, {}!", input.trim());  // trim() removes newline
    
    let number: i32 = input
        .trim()           // Remove "\n"
        .parse()          // String → i32
        .expect("Not a valid number");  // Handle parse error
    
    println!("You entered: {}", number);

    /*
if input % 2==0 {
    println!("Even");
} else {
    println!("Odd");
}
*/
}




