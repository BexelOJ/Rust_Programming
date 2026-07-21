use std::io;

fn calculate_tip(bill: f64, tip_percent: i32) {
    // Calculate tip and total
  	let tip: f64 = bill * (tip_percent as f64 / 100.00);
		let total: f64 =  bill + tip;  
    // Print formatted results
		println!("Bill: ${:.2}", bill);
    println!("Tip: ${:.2}", tip);
    println!("Total: ${:.2}", total);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let bill: f64 = input.trim().parse().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let tip_percent: i32 = input.trim().parse().unwrap();

    calculate_tip(bill, tip_percent);
}







