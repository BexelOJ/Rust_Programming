fn greater_than(a: i32, b: i32)->bool{
if a > b {
    println!("{} greater than {}",a, b);
}
else{
    println!("{} <= {}",a, b);
}
return a > b;
}

fn less_than(a: i32, b: i32)->bool{
if a < b {
    println!("{} less than {}",a, b);
}
else{
    println!("{} >= {}",a, b);
}
return a < b;
}

fn equals_to(a: i32, b: i32)->bool{
if a == b {
    println!("{} equals to {}",a, b);
}
return a == b;
}

fn not_equals_to(a: i32, b: i32)->bool{
if a != b {
    println!("{} not equals to {}",a, b);
}
return a != b;
}


fn main() {

let a: i32 = 10;
let b: i32 = 20;

println!("Value of A:{} ",a);
println!("Value of B : {} ",b);

let res_1 = greater_than(a,b);
println!("{}",res_1);

let res_2 = less_than(a, b);
println!("{}",res_2);

let res_3 = equals_to(a, b);
println!("{}",res_3);

let res_4 = not_equals_to(a, b);
println!("{}",res_4);
}

/*
let mut res = A>B ;
println!("A greater than B: {} ",res);
res = A<B ;
println!("A lesser than B: {} ",res) ;
res = A>=B ;
println!("A greater than or equal to B: {} ",res);
res = A<=B;
println!("A lesser than or equal to B: {}",res) ;
res = A==B ;
println!("A is equal to B: {}",res) ;

*/

