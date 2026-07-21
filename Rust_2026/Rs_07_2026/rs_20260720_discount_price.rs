use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let price: f64 = input.trim().parse().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let discount_percent: i32 = input.trim().parse().unwrap();

    // Calculate discount and final price
    let discount_price: f64 = price * (discount_percent as f64/ 100.00);
    let final_price: f64 = price - discount_price;
  
    // Print results
    println!("Discount: {:.2}",discount_price);
    println!("Final price: {:.2}",final_price);
}



