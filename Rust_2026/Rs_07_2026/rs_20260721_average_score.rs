use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    // Read scores and calculate average
	  let mut sum: i32 = 0;
    for _ in 0..n {
          let mut input = String::new();
          io::stdin().read_line(&mut input).unwrap();
          let val: i32 = input.trim().parse().unwrap();
          sum += val;
    }
  
    // Print the average
  	println!("Average: {:.1}",sum as f64/n as f64); 
}

