use std::io;

fn main() {
    let mut user_name:String = String::new();
    println!("Hello User, Please enter your name: ");
    io::stdin().read_line(&mut user_name);

    println!("Name of User : {}",user_name);
    println!("String length : {}", user_name.len());
}
