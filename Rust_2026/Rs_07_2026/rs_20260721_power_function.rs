use std::io;

fn power(base: i64, exp: i64) -> i64 {
   // Calculate base^exp using a loop
   
  let mut res: i64 = base;  
  
  if exp == 0 {
  	return 1;
  }
  
  for _ in 1..exp {
      res *= base;
   }	
  res
}

fn main(){
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  let base: i64 = input.trim().parse().unwrap();
  
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  let exp: i64 = input.trim().parse().unwrap();
  
  let final_res: i64 = power(base, exp);
  println!("Result: {}",final_res);
  
}

