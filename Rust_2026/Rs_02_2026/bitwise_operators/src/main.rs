fn main() {

let a: i32 = 2; // Bit presentation 10
let b: i32 = 3; // Bit presentation 11

let mut result:i32;

result = a & b;
println!("(a & b) => {} ",result);

result = a | b;
println!("(a | b) => {} ",result) ;

result = a ^ b;
println!("(a ^ b) => {} ",result);

result = !b;
println!("(!b) => {} ",result);

result = a << 2;
println!("(a << 2) => {} ",result);

result = a >> 1;
println!("(a >> 1) => {} ",result);

}

