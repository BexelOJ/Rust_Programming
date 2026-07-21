use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let age: i32 = input.trim().parse().unwrap();

    // Determine ticket type and price

    // Print results
    if age < 12 {
    	println!("Child");
      println!("$5");
    } else if age >=12 && age <= 64 {
    	println!("Adult");
      println!("$15");    
    } else {
    	println!("Senior");
      println!("$8");    
    }
  
}

