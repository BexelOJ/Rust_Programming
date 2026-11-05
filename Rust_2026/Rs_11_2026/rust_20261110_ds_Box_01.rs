//---------------------------------------------------
// Box - Rust Data Structures
// Date: 20261110
//---------------------------------------------------

use std::mem::{size_of, size_of_val};

//====================================================
// 01. Structure for Box examples
//====================================================

#[derive(Debug, Clone)]
struct Employee {
    id: u32,
    name: String,
    salary: f64,
}

//====================================================
// 02. Recursive structure using Box
//====================================================

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

//====================================================
// 03. Binary tree structure using Box
//====================================================

#[derive(Debug)]
struct TreeNode {
    data: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(data: i32) -> Self {
        Self {
            data,
            left: None,
            right: None,
        }
    }
}

//====================================================
// 04. Function accepting Box by reference
//====================================================

fn display_box(value: &Box<i32>) {
    println!("Box value       : {}", value);
}

//====================================================
// 05. Function returning Box
//====================================================

fn create_box(value: i32) -> Box<i32> {
    Box::new(value)
}

//====================================================
// 06. Function accepting ownership
//====================================================

fn consume_box(value: Box<i32>) {
    println!("Consumed value  : {}", value);
}

//====================================================
// 07. Function modifying Box
//====================================================

fn modify_box(value: &mut Box<i32>) {
    **value += 100;
}

//====================================================
// 08. Recursive List Display
//====================================================

fn display_list(list: &List) {
    match list {
        List::Cons(value, next) => {
            print!("{} -> ", value);
            display_list(next);
        }
        List::Nil => {
            println!("Nil");
        }
    }
}

//====================================================
// 09. Binary Tree Inorder Traversal
//====================================================

fn inorder(root: &Option<Box<TreeNode>>) {
    if let Some(node) = root {
        inorder(&node.left);
        print!("{} ", node.data);
        inorder(&node.right);
    }
}

//====================================================
// Main
//====================================================

fn main() {

    println!("\n========================================");
    println!("          BOX SMART POINTER");
    println!("========================================");

    //------------------------------------------------
    // 10. Create Box
    //------------------------------------------------

    println!("\n10. Create Box");

    let number = Box::new(100);

    println!("Box value       : {}", number);
    println!("Dereferenced    : {}", *number);

    //------------------------------------------------
    // 11. Mutable Box
    //------------------------------------------------

    println!("\n11. Mutable Box");

    let mut value = Box::new(200);

    println!("Before          : {}", value);

    *value = 500;

    println!("After           : {}", value);

    //------------------------------------------------
    // 12. Box with String
    //------------------------------------------------

    println!("\n12. Box with String");

    let message = Box::new(String::from("Rust Programming"));

    println!("Message         : {}", message);
    println!("Length          : {}", message.len());

    //------------------------------------------------
    // 13. Box with Array
    //------------------------------------------------

    println!("\n13. Box with Array");

    let array = Box::new([10, 20, 30, 40, 50]);

    println!("Array           : {:?}", array);
    println!("Element 2       : {}", array[2]);

    //------------------------------------------------
    // 14. Box with Vector
    //------------------------------------------------

    println!("\n14. Box with Vector");

    let numbers = Box::new(vec![10, 20, 30, 40]);

    println!("Vector          : {:?}", numbers);
    println!("Length          : {}", numbers.len());

    //------------------------------------------------
    // 15. Box with Structure
    //------------------------------------------------

    println!("\n15. Box with Structure");

    let employee = Box::new(Employee {
        id: 101,
        name: String::from("Bexel"),
        salary: 75000.0,
    });

    println!("Employee ID     : {}", employee.id);
    println!("Employee Name   : {}", employee.name);
    println!("Salary          : {}", employee.salary);

    //------------------------------------------------
    // 16. Box Ownership and Move
    //------------------------------------------------

    println!("\n16. Box Ownership");

    let first = Box::new(1000);

    let second = first;

    println!("Second          : {}", second);

    // first is no longer valid.

    //------------------------------------------------
    // 17. Clone Box
    //------------------------------------------------

    println!("\n17. Clone Box");

    let first = Box::new(2000);

    let second = first.clone();

    println!("First           : {}", first);
    println!("Second          : {}", second);

    //------------------------------------------------
    // 18. Box passed by reference
    //------------------------------------------------

    println!("\n18. Box by Reference");

    let value = Box::new(4000);

    display_box(&value);

    println!("Original        : {}", value);

    //------------------------------------------------
    // 19. Return Box from Function
    //------------------------------------------------

    println!("\n19. Return Box");

    let result = create_box(5000);

    println!("Returned value  : {}", result);

    //------------------------------------------------
    // 20. Transfer Ownership
    //------------------------------------------------

    println!("\n20. Transfer Ownership");

    let value = Box::new(6000);

    consume_box(value);

    //------------------------------------------------
    // 21. Mutable Reference
    //------------------------------------------------

    println!("\n21. Mutable Reference");

    let mut value = Box::new(7000);

    modify_box(&mut value);

    println!("Modified value  : {}", value);

    //------------------------------------------------
    // 22. Dereferencing
    //------------------------------------------------

    println!("\n22. Dereferencing");

    let mut value = Box::new(8000);

    println!("Before          : {}", *value);

    *value = 9000;

    println!("After           : {}", *value);

    //------------------------------------------------
    // 23. Box inside Option
    //------------------------------------------------

    println!("\n23. Box inside Option");

    let value: Option<Box<i32>> = Some(Box::new(10000));

    match value {
        Some(number) => println!("Value           : {}", number),
        None => println!("No value"),
    }

    //------------------------------------------------
    // 24. Box inside Result
    //------------------------------------------------

    println!("\n24. Box inside Result");

    let result: Result<Box<i32>, &str> = Ok(Box::new(11000));

    match result {
        Ok(number) => println!("Value           : {}", number),
        Err(error) => println!("Error           : {}", error),
    }

    //------------------------------------------------
    // 25. Recursive List
    //------------------------------------------------

    println!("\n25. Recursive List");

    let list = List::Cons(
        10,
        Box::new(
            List::Cons(
                20,
                Box::new(
                    List::Cons(
                        30,
                        Box::new(List::Nil),
                    ),
                ),
            ),
        ),
    );

    display_list(&list);

    //------------------------------------------------
    // 26. Binary Tree
    //------------------------------------------------

    println!("\n26. Binary Tree");

    let mut root = Some(Box::new(TreeNode::new(10)));

    if let Some(node) = root.as_mut() {

        node.left = Some(Box::new(TreeNode::new(5)));

        node.right = Some(Box::new(TreeNode::new(20)));

        if let Some(left) = node.left.as_mut() {
            left.left = Some(Box::new(TreeNode::new(3)));
            left.right = Some(Box::new(TreeNode::new(7)));
        }
    }

    println!("Tree            : {:?}", root);

    print!("Inorder         : ");
    inorder(&root);
    println!();

    //------------------------------------------------
    // 27. Nested Box
    //------------------------------------------------

    println!("\n27. Nested Box");

    let nested = Box::new(Box::new(Box::new(12345)));

    println!("Nested value    : {}", nested);

    //------------------------------------------------
    // 28. Box with Trait Object
    //------------------------------------------------

    println!("\n28. Box with Trait Object");

    trait Device {
        fn name(&self) -> &str;
    }

    struct Sensor;

    impl Device for Sensor {
        fn name(&self) -> &str {
            "Temperature Sensor"
        }
    }

    let device: Box<dyn Device> = Box::new(Sensor);

    println!("Device          : {}", device.name());

    //------------------------------------------------
    // 29. Memory Size
    //------------------------------------------------

    println!("\n29. Memory Size");

    let normal = 100_i32;

    let boxed = Box::new(100_i32);

    println!("i32 size        : {} bytes", size_of::<i32>());

    println!(
        "Box size        : {} bytes",
        size_of::<Box<i32>>()
    );

    println!(
        "Box value size  : {} bytes",
        size_of_val(&*boxed)
    );

    println!("Normal value    : {}", normal);

    //------------------------------------------------
    // 30. Box with Large Array
    //------------------------------------------------

    println!("\n30. Box with Large Array");

    let large_array = Box::new([0_u8; 1024]);

    println!(
        "Array size      : {} bytes",
        size_of_val(&*large_array)
    );

    println!("First byte      : {}", large_array[0]);

    //------------------------------------------------
    // 31. Automatic Cleanup
    //------------------------------------------------

    println!("\n31. Automatic Cleanup");

    {
        let temporary = Box::new(99999);

        println!("Temporary       : {}", temporary);

    } // Box is dropped here.

    println!("Temporary Box dropped at scope exit");

    //------------------------------------------------
    // End
    //------------------------------------------------

    println!("\n========================================");
    println!("          PROGRAM COMPLETED");
    println!("========================================");
}


/*
//---------------------------------------------------
Key concepts to understand:

Concept	Meaning

Box::new(value)	
- Allocates a value on the heap

*boxed
- Dereferences the Box

Box<T>
- Single ownership

Box::clone()
- Clones the underlying value when T: Clone

Option<Box<T>>	
- Optional heap-owned value

Box<dyn Trait>	
- Heap-allocated trait object

//---------------------------------------------------


//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261110_ds_Box_01.rs -o box_example

./box_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

========================================
          BOX SMART POINTER
========================================

10. Create Box
Box value       : 100
Dereferenced    : 100

11. Mutable Box
Before          : 200
After           : 500

12. Box with String
Message         : Rust Programming
Length          : 16

13. Box with Array
Array           : [10, 20, 30, 40, 50]
Element 2       : 30

14. Box with Vector
Vector          : [10, 20, 30, 40]
Length          : 4

15. Box with Structure
Employee ID     : 101
Employee Name   : Bexel
Salary          : 75000

16. Box Ownership
Second          : 1000

17. Clone Box
First           : 2000
Second          : 2000

18. Box by Reference
Box value       : 4000
Original        : 4000

19. Return Box
Returned value  : 5000

20. Transfer Ownership
Consumed value  : 6000

21. Mutable Reference
Modified value  : 7100

22. Dereferencing
Before          : 8000
After           : 9000

23. Box inside Option
Value           : 10000

24. Box inside Result
Value           : 11000

25. Recursive List
10 -> 20 -> 30 -> Nil

26. Binary Tree
Tree            : Some(TreeNode { data: 10, left: Some(TreeNode { data: 5, left: Some(TreeNode { data: 3, left: None, right: None }), right: Some(TreeNode { data: 7, left: None, right: None }) }), right: Some(TreeNode { data: 20, left: None, right: None }) })
Inorder         : 3 5 7 10 20

27. Nested Box
Nested value    : 12345

28. Box with Trait Object
Device          : Temperature Sensor

29. Memory Size
i32 size        : 4 bytes
Box size        : 8 bytes
Box value size  : 4 bytes
Normal value    : 100

30. Box with Large Array
Array size      : 1024 bytes
First byte      : 0

31. Automatic Cleanup
Temporary       : 99999
Temporary Box dropped at scope exit

========================================
          PROGRAM COMPLETED
========================================

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/


