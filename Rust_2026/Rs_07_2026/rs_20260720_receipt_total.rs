use std::io;

fn main(){
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  let item: String = input.trim().parse().unwrap();

  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  let price: f64 = input.trim().parse().unwrap();

  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  let quantity: i32 = input.trim().parse().unwrap();

  let total: f64 = quantity as f64 * price;
  
  println!("Item: {}",item);
  println!("Price: ${:.2}",price);
  println!("Quantity: {}",quantity);
  println!("Total: ${:.2}",total);

}




