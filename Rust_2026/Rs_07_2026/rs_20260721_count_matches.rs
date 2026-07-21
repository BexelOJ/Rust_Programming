use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    
    let mut numbers = Vec::new();
  
    for _ in 0..n {
        input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        numbers.push(input.trim().parse::<i32>().unwrap());
    }
    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let target: i32 = input.trim().parse().unwrap();

    // Count and print
}

