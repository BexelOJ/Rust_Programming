use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    // Loop from 1 to n
  	for i in 1..(n+1) {
       	   if i % 3 == 0 && i % 5 == 0 {
		println!("FizzBuzz");
	   } else if i % 3 == 0 {
		println!("Fizz");
	   } else if i % 5 == 0 {
		println!("Buzz");
	   } else {
		println!("{}",i);
	   }
	   
    }
}

