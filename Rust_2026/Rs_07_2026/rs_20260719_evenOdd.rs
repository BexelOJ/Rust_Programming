use std::io;

fn main(){

let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
let n:i32 = input.trim().parse().unwrap();

if n % 2 == 0 {
	println!("Entered Number {} is Even",input);
 } else {
	println!("Entered Number {} is Odd",input);
}
}
