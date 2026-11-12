use std::io;

fn add(a: i32, b:i32)->i32{
    a + b
}

fn sub(a: i32, b:i32)->i32{
    b - a
}

fn mul(a: i32, b:i32)->i32{
    a * b
}

fn div(a:i32, b:i32)->i32{
    b/a
}

fn main() {
    let a = 10;
    let b = 20;
    let c = 30;
    let d = 40;
    
    println!("Hello, world!");
    println!("Addition Result : {}", add(a,b));
    println!("Subtraction Result : {}", sub(a,b));
    println!("Multiplication Result : {}", mul(a,b));
    println!("Division Result: {}", div(a,b));

    println!("All Done");
}
