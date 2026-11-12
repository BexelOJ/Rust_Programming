fn main() {
    let mut loop_count = 0;
    let mut while_count = 0;

// loop in Rust:
    loop {
        loop_count += 1;
        println!("loop_count: {}", loop_count);
        if loop_count == 5 {
            break;
        }
    }
    println!("");

// while loop in Rust:
    while while_count < 5 {
        println!("while_count: {}", while_count);
        while_count += 1;
    }
    println!("");

// for loop in Rust:
    println!("iter over a Range, using for loop");
    for i in 0..5 {
        println!("{}", i);
    }
    println!("");

// Iterating over an array:
    let numbers = [10, 20, 30, 40, 50];
    println!("Array variants:");
    for num in numbers {
        println!("{}", num);
    }
    println!("");

// Iterating over a String:
    let name = String::from("Bexel");

    for ch in name.chars() {
        println!("{}", ch);
    }


}



/*
Syntax - Meaning:
0..5	- 0 to 4 (5 excluded)
0..=5	- 0 to 5 (5 included)
1..10	- 1 to 9
1..=10	- 1 to 10

Keyword - Purpose:
break - Terminates the loop
continue - Skips the current iteration


*/





