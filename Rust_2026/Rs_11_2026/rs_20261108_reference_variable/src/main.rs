fn main() {
    let x = 10;
    let y = &x;

    println!("x = {}", x);
    println!("y = {}", y);

    println!("Reference: {:p}", y);
    println!("Value: {}", *y);

// Rust supports mutable references
    let mut a = 10;
    let b = &mut a;

    *b = 20;
    println!("value after change by ref variable = {}", a);


}


/*
Operation - C - C++ - Rust
Address of - &x - &x - &x
Pointer declaration - int *p - int *p - let p: *const i32
Dereference - *p - *p - *p


Rust has two different concepts:

References
&T — immutable reference.
&mut T — mutable reference.

Raw pointers
*const T — immutable raw pointer.
*mut T — mutable raw pointer.


*/
