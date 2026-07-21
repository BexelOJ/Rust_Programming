use std::io;

/*
--------------------------------------------------
1.) Empty String (most common for user input)

let mut s = String::new();

// Creates an empty string.
// s : String

--------------------------------------------------
2.) From a string literal

let s = String::from("Hello");

// or let s: String = String::from("Hello");

--------------------------------------------------
3. Using .to_string()

let s = "Hello".to_string();

--------------------------------------------------
4. String literal (&str)

let s = "Hello";


--------------------------------------------------

let s = "Hello";  
// is a string literal &str,
// is in .rodata memory
// cannot modify

let str = s.to_string(); 
// hello will be in Heap memory
// str is a pointer in stack
// can modify

--------------------------------------------------
let s = "Hello";	.rodata (read-only memory)
let s = input.trim();	Heap memory owned by input
let s = &my_string;	Heap memory owned by my_string
let s = &vec[..];	Heap memory owned by the Vec

--------------------------------------------------

*/


