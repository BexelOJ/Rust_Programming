use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let width: i32 = input.trim().parse().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let height: i32 = input.trim().parse().unwrap();

    // Print the rectangle border
    for i in 0..height {
        for j in 0..width {
            if i == 0 || i == height - 1 || j == 0 || j == width - 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }   
 
}

