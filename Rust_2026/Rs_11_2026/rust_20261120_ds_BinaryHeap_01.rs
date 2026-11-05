//---------------------------------------------------
// BinaryHeap - Rust Data Structures
// Date: 20261120
//---------------------------------------------------


use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::mem::size_of;

//--------------------------------------------------
// 01. Structure for custom Employee example
//--------------------------------------------------

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Employee {
    id: u32,
    name: String,
}

//--------------------------------------------------
// 02. Display a BinaryHeap
//--------------------------------------------------

fn display_heap<T: std::fmt::Debug + Ord + Clone>(heap: &BinaryHeap<T>) {
    let mut temp = heap.clone();

    print!("Heap elements (priority order): ");

    while let Some(value) = temp.pop() {
        print!("{:?} ", value);
    }

    println!();
}

//--------------------------------------------------
// 03. Display a MinHeap
//--------------------------------------------------

fn display_min_heap(heap: &BinaryHeap<Reverse<i32>>) {
    let mut temp = heap.clone();

    print!("MinHeap elements: ");

    while let Some(Reverse(value)) = temp.pop() {
        print!("{} ", value);
    }

    println!();
}

//==================================================
// MAIN FUNCTION
//==================================================

fn main() {

    //--------------------------------------------------
    // 04. Create an empty BinaryHeap
    //--------------------------------------------------

    println!("\n04. Create BinaryHeap");

    let mut heap: BinaryHeap<i32> = BinaryHeap::new();

    println!("Is empty: {}", heap.is_empty());
    println!("Length  : {}", heap.len());

    //--------------------------------------------------
    // 05. Insert elements using push()
    //--------------------------------------------------

    println!("\n05. Insert elements");

    heap.push(10);
    heap.push(40);
    heap.push(20);
    heap.push(70);
    heap.push(30);

    println!("Length: {}", heap.len());

    display_heap(&heap);

    //--------------------------------------------------
    // 06. Access maximum element using peek()
    //--------------------------------------------------

    println!("\n06. Peek maximum element");

    match heap.peek() {
        Some(value) => println!("Maximum element: {}", value),
        None => println!("Heap is empty"),
    }

    //--------------------------------------------------
    // 07. Remove maximum element using pop()
    //--------------------------------------------------

    println!("\n07. Pop maximum element");

    match heap.pop() {
        Some(value) => println!("Removed: {}", value),
        None => println!("Heap is empty"),
    }

    display_heap(&heap);

    //--------------------------------------------------
    // 08. Insert another element
    //--------------------------------------------------

    println!("\n08. Insert another element");

    heap.push(100);

    display_heap(&heap);

    //--------------------------------------------------
    // 09. Check length and empty status
    //--------------------------------------------------

    println!("\n09. Length and empty status");

    println!("Length  : {}", heap.len());
    println!("Is empty: {}", heap.is_empty());

    //--------------------------------------------------
    // 10. Check heap capacity
    //--------------------------------------------------

    println!("\n10. Heap capacity");

    println!("Capacity: {}", heap.capacity());

    //--------------------------------------------------
    // 11. Create heap with initial capacity
    //--------------------------------------------------

    println!("\n11. BinaryHeap with capacity");

    let mut capacity_heap: BinaryHeap<i32> =
        BinaryHeap::with_capacity(10);

    capacity_heap.push(25);
    capacity_heap.push(75);
    capacity_heap.push(50);

    println!("Capacity: {}", capacity_heap.capacity());

    display_heap(&capacity_heap);

    //--------------------------------------------------
    // 12. Reserve additional capacity
    //--------------------------------------------------

    println!("\n12. Reserve capacity");

    capacity_heap.reserve(20);

    println!("Capacity after reserve: {}", capacity_heap.capacity());

    //--------------------------------------------------
    // 13. Shrink capacity
    //--------------------------------------------------

    println!("\n13. Shrink capacity");

    capacity_heap.shrink_to_fit();

    println!("Capacity after shrink: {}", capacity_heap.capacity());

    //--------------------------------------------------
    // 14. Iterate over heap elements
    //--------------------------------------------------

    println!("\n14. Iterate over heap");

    for value in &heap {
        print!("{} ", value);
    }

    println!();

    // Note: Iteration order is NOT guaranteed to be sorted.

    //--------------------------------------------------
    // 15. Convert BinaryHeap into sorted vector
    //--------------------------------------------------

    println!("\n15. Convert into sorted vector");

    let sorted_values = heap.clone().into_sorted_vec();

    println!("Ascending order: {:?}", sorted_values);

    //--------------------------------------------------
    // 16. Build heap from an existing vector
    //--------------------------------------------------

    println!("\n16. Create heap from Vec");

    let values = vec![15, 90, 35, 60, 10];

    let vector_heap: BinaryHeap<i32> = values.into();

    display_heap(&vector_heap);

    //--------------------------------------------------
    // 17. Convert heap into Vec
    //--------------------------------------------------

    println!("\n17. Convert heap into Vec");

    let result: Vec<i32> = vector_heap.into_vec();

    println!("Vector: {:?}", result);

    //--------------------------------------------------
    // 18. Extend heap with multiple elements
    //--------------------------------------------------

    println!("\n18. Extend heap");

    let mut extend_heap = BinaryHeap::new();

    extend_heap.extend([10, 50, 20, 80, 30]);

    display_heap(&extend_heap);

    //--------------------------------------------------
    // 19. Clear the heap
    //--------------------------------------------------

    println!("\n19. Clear heap");

    extend_heap.clear();

    println!("Length after clear: {}", extend_heap.len());
    println!("Is empty: {}", extend_heap.is_empty());

    //--------------------------------------------------
    // 20. BinaryHeap with duplicate values
    //--------------------------------------------------

    println!("\n20. Duplicate values");

    let mut duplicate_heap = BinaryHeap::new();

    duplicate_heap.extend([10, 30, 10, 50, 30, 50]);

    display_heap(&duplicate_heap);

    // BinaryHeap allows duplicate elements.

    //--------------------------------------------------
    // 21. BinaryHeap with negative numbers
    //--------------------------------------------------

    println!("\n21. Negative numbers");

    let mut negative_heap = BinaryHeap::new();

    negative_heap.extend([-10, -50, -20, -5, -100]);

    display_heap(&negative_heap);

    //--------------------------------------------------
    // 22. Create a MinHeap using Reverse
    //--------------------------------------------------

    println!("\n22. MinHeap using Reverse");

    let mut min_heap: BinaryHeap<Reverse<i32>> =
        BinaryHeap::new();

    min_heap.push(Reverse(40));
    min_heap.push(Reverse(10));
    min_heap.push(Reverse(70));
    min_heap.push(Reverse(20));
    min_heap.push(Reverse(50));

    display_min_heap(&min_heap);

    //--------------------------------------------------
    // 23. Peek minimum element
    //--------------------------------------------------

    println!("\n23. Peek minimum element");

    match min_heap.peek() {
        Some(Reverse(value)) => println!("Minimum: {}", value),
        None => println!("MinHeap is empty"),
    }

    //--------------------------------------------------
    // 24. Pop minimum element
    //--------------------------------------------------

    println!("\n24. Pop minimum element");

    match min_heap.pop() {
        Some(Reverse(value)) => println!("Removed: {}", value),
        None => println!("MinHeap is empty"),
    }

    display_min_heap(&min_heap);

    //--------------------------------------------------
    // 25. BinaryHeap with String
    //--------------------------------------------------

    println!("\n25. BinaryHeap with String");

    let mut string_heap: BinaryHeap<String> = BinaryHeap::new();

    string_heap.push(String::from("Apple"));
    string_heap.push(String::from("Orange"));
    string_heap.push(String::from("Banana"));
    string_heap.push(String::from("Mango"));

    display_heap(&string_heap);

    //--------------------------------------------------
    // 26. BinaryHeap with Employee
    //--------------------------------------------------

    println!("\n26. BinaryHeap with Employee");

    let mut employee_heap: BinaryHeap<Employee> = BinaryHeap::new();

    employee_heap.push(Employee {
        id: 101,
        name: String::from("Arun"),
    });

    employee_heap.push(Employee {
        id: 103,
        name: String::from("Bala"),
    });

    employee_heap.push(Employee {
        id: 102,
        name: String::from("Charan"),
    });

    // Employee ordering follows derived field ordering:
    // id first, then name.

    while let Some(employee) = employee_heap.pop() {
        println!(
            "Employee ID: {}, Name: {}",
            employee.id,
            employee.name
        );
    }

    //--------------------------------------------------
    // 27. BinaryHeap with tuples
    //--------------------------------------------------

    println!("\n27. BinaryHeap with tuples");

    let mut tuple_heap: BinaryHeap<(i32, &str)> = BinaryHeap::new();

    tuple_heap.push((2, "Medium"));
    tuple_heap.push((5, "High"));
    tuple_heap.push((1, "Low"));
    tuple_heap.push((5, "Critical"));

    while let Some((priority, name)) = tuple_heap.pop() {
        println!("Priority: {}, Task: {}", priority, name);
    }

    // Tuples are ordered lexicographically:
    // first field, then second field.

    //--------------------------------------------------
    // 28. BinaryHeap for task scheduling
    //--------------------------------------------------

    println!("\n28. Task scheduling");

    let mut task_heap: BinaryHeap<(u8, &str)> = BinaryHeap::new();

    task_heap.push((1, "Background task"));
    task_heap.push((5, "Critical task"));
    task_heap.push((3, "Normal task"));
    task_heap.push((4, "Important task"));

    while let Some((priority, task)) = task_heap.pop() {
        println!("Priority {}: {}", priority, task);
    }

    //--------------------------------------------------
    // 29. BinaryHeap for emergency handling
    //--------------------------------------------------

    println!("\n29. Emergency handling");

    let mut emergency_heap: BinaryHeap<(u8, &str)> =
        BinaryHeap::new();

    emergency_heap.push((2, "Minor injury"));
    emergency_heap.push((5, "Cardiac emergency"));
    emergency_heap.push((3, "Fracture"));
    emergency_heap.push((4, "Severe bleeding"));

    while let Some((severity, case_name)) = emergency_heap.pop() {
        println!("Severity {}: {}", severity, case_name);
    }

    //--------------------------------------------------
    // 30. Check memory size
    //--------------------------------------------------

    println!("\n30. Memory size");

    println!(
        "BinaryHeap<i32> descriptor size: {} bytes",
        size_of::<BinaryHeap<i32>>()
    );

    println!(
        "i32 element size: {} bytes",
        size_of::<i32>()
    );

    // Descriptor size does not include heap allocation.

    //--------------------------------------------------
    // 31. Process elements until heap is empty
    //--------------------------------------------------

    println!("\n31. Process entire heap");

    let mut process_heap = BinaryHeap::new();

    process_heap.extend([45, 15, 85, 25, 65]);

    while let Some(value) = process_heap.pop() {
        println!("Processing: {}", value);
    }

    println!("All elements processed.");

    //--------------------------------------------------
    // 32. BinaryHeap vs MinHeap
    //--------------------------------------------------

    println!("\n32. MaxHeap and MinHeap comparison");

    let mut max_heap = BinaryHeap::new();

    let mut min_heap = BinaryHeap::new();

    for value in [40, 10, 70, 20, 50] {
        max_heap.push(value);
        min_heap.push(Reverse(value));
    }

    println!("MaxHeap top: {:?}", max_heap.peek());
    println!("MinHeap top: {:?}", min_heap.peek());

    //--------------------------------------------------
    // 33. Final status
    //--------------------------------------------------

    println!("\n33. Program completed successfully.");
}


/*
//---------------------------------------------------
Important distinction:

Data structure - Ordering

BinaryHeap - Max-heap by default
BinaryHeap with Reverse<T> - Min-heap
PriorityQueue (our previous implementation) - Custom priority
Queue - FIFO
Stack - LIFO

//---------------------------------------------------
Important operations and complexity:

Operation - Time complexity

new() - O(1)
peek() - O(1)
push() - O(log n)
pop() - O(log n)
len() - O(1)
is_empty() - O(1)
into_sorted_vec() - O(n log n)
from() / heapify - O(n)

//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261120_ds_BinaryHeap_01.rs -o binaryheap_example

./binaryheap_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

04. Create BinaryHeap
Is empty: true
Length  : 0

05. Insert elements
Length: 5
Heap elements (priority order): 70 40 30 20 10

06. Peek maximum element
Maximum element: 70

07. Pop maximum element
Removed: 70
Heap elements (priority order): 40 30 20 10

08. Insert another element
Heap elements (priority order): 100 40 30 20 10

09. Length and empty status
Length  : 5
Is empty: false

10. Heap capacity
Capacity: 8

11. BinaryHeap with capacity
Capacity: 10
Heap elements (priority order): 75 50 25

12. Reserve capacity
Capacity after reserve: 23

13. Shrink capacity
Capacity after shrink: 3

14. Iterate over heap
100 40 20 10 30

15. Convert into sorted vector
Ascending order: [10, 20, 30, 40, 100]

16. Create heap from Vec
Heap elements (priority order): 90 60 35 15 10

17. Convert heap into Vec
Vector: [90, 60, 35, 15, 10]

18. Extend heap
Heap elements (priority order): 80 50 30 20 10

19. Clear heap
Length after clear: 0
Is empty: true

20. Duplicate values
Heap elements (priority order): 50 50 30 30 10 10

21. Negative numbers
Heap elements (priority order): -5 -10 -20 -50 -100

22. MinHeap using Reverse
MinHeap elements: 10 20 40 50 70

23. Peek minimum element
Minimum: 10

24. Pop minimum element
Removed: 10
MinHeap elements: 20 40 50 70

25. BinaryHeap with String
Heap elements (priority order): "Orange" "Mango" "Banana" "Apple"

26. BinaryHeap with Employee
Employee ID: 103, Name: Bala
Employee ID: 102, Name: Charan
Employee ID: 101, Name: Arun

27. BinaryHeap with tuples
Priority: 5, Task: High
Priority: 5, Task: Critical
Priority: 2, Task: Medium
Priority: 1, Task: Low

28. Task scheduling
Priority 5: Critical task
Priority 4: Important task
Priority 3: Normal task
Priority 1: Background task

29. Emergency handling
Severity 5: Cardiac emergency
Severity 4: Severe bleeding
Severity 3: Fracture
Severity 2: Minor injury

30. Memory size
BinaryHeap<i32> descriptor size: 24 bytes
i32 element size: 4 bytes

31. Process entire heap
Processing: 85
Processing: 65
Processing: 45
Processing: 25
Processing: 15
All elements processed.

32. MaxHeap and MinHeap comparison
MaxHeap top: Some(70)
MinHeap top: Some(Reverse(10))

33. Program completed successfully.

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/
