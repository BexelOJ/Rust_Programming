//---------------------------------------------------
// Vector - Rust Data Structures
// Date: 20261105
//---------------------------------------------------

use std::convert::TryInto;
use std::mem;

//==============================================================
// 01. VECTOR DECLARATION
//==============================================================

fn vector_declaration()
{
    let numbers: Vec<i32> = Vec::new();

    println!("Empty vector: {:?}", numbers);
}

//==============================================================
// 02. VECTOR INITIALIZATION USING vec! MACRO
//==============================================================

fn vector_initialization()
{
    let numbers = vec![10, 20, 30, 40, 50];

    println!("Vector:       {:?}", numbers);
}

//==============================================================
// 03. MUTABLE VECTOR
//==============================================================

fn mutable_vector()
{
    let mut numbers = vec![10, 20, 30];

    numbers.push(40);

    println!("After push:   {:?}", numbers);
}

//==============================================================
// 04. VECTOR WITH INITIAL CAPACITY
//==============================================================

fn vector_with_capacity()
{
    let mut numbers: Vec<i32> = Vec::with_capacity(10);

    numbers.push(100);
    numbers.push(200);

    println!("Vector:       {:?}", numbers);
    println!("Length:       {}", numbers.len());
    println!("Capacity:     {}", numbers.capacity());
}

//==============================================================
// 05. PUSH ELEMENT
//==============================================================

fn push_element()
{
    let mut numbers = vec![10, 20];

    numbers.push(30);
    numbers.push(40);

    println!("After push:   {:?}", numbers);
}

//==============================================================
// 06. POP ELEMENT
//==============================================================

fn pop_element()
{
    let mut numbers = vec![10, 20, 30];

    let removed = numbers.pop();

    println!("Removed:      {:?}", removed);
    println!("Remaining:    {:?}", numbers);
}

//==============================================================
// 07. INSERT ELEMENT
//==============================================================

fn insert_element()
{
    let mut numbers = vec![10, 20, 40];

    numbers.insert(2, 30);

    println!("After insert: {:?}", numbers);
}

//==============================================================
// 08. REMOVE ELEMENT
//==============================================================

fn remove_element()
{
    let mut numbers = vec![10, 20, 30, 40];

    let removed = numbers.remove(1);

    println!("Removed:      {}", removed);
    println!("Remaining:    {:?}", numbers);
}

//==============================================================
// 09. SWAP REMOVE
//==============================================================

fn swap_remove_element()
{
    let mut numbers = vec![10, 20, 30, 40];

    let removed = numbers.swap_remove(1);

    println!("Removed:      {}", removed);
    println!("Remaining:    {:?}", numbers);
}

//==============================================================
// 10. VECTOR LENGTH AND CAPACITY
//==============================================================

fn vector_length_capacity()
{
    let mut numbers = Vec::with_capacity(8);

    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    println!("Length:       {}", numbers.len());
    println!("Capacity:     {}", numbers.capacity());
    println!("Is empty:     {}", numbers.is_empty());
}

//==============================================================
// 11. ACCESS ELEMENT USING INDEX
//==============================================================

fn access_using_index()
{
    let numbers = vec![10, 20, 30, 40];

    println!("Element:      {}", numbers[2]);
}

//==============================================================
// 12. SAFE ACCESS USING GET
//==============================================================

fn safe_access()
{
    let numbers = vec![10, 20, 30];

    match numbers.get(5)
    {
        Some(value) => println!("Value:        {}", value),
        None => println!("Index not found"),
    }
}

//==============================================================
// 13. MODIFY ELEMENT USING INDEX
//==============================================================

fn modify_element()
{
    let mut numbers = vec![10, 20, 30];

    numbers[1] = 200;

    println!("Modified:     {:?}", numbers);
}

//==============================================================
// 14. MODIFY ELEMENT USING GET_MUT
//==============================================================

fn modify_using_get_mut()
{
    let mut numbers = vec![10, 20, 30];

    if let Some(value) = numbers.get_mut(1)
    {
        *value = 200;
    }

    println!("Modified:     {:?}", numbers);
}

//==============================================================
// 15. ITERATE USING FOR
//==============================================================

fn iterate_vector()
{
    let numbers = vec![10, 20, 30, 40];

    for value in &numbers
    {
        println!("Value:        {}", value);
    }
}

//==============================================================
// 16. ITERATE USING ITER
//==============================================================

fn iterate_using_iter()
{
    let numbers = vec![10, 20, 30];

    for value in numbers.iter()
    {
        println!("Value:        {}", value);
    }
}

//==============================================================
// 17. ITERATE USING ITER_MUT
//==============================================================

fn iterate_using_iter_mut()
{
    let mut numbers = vec![10, 20, 30];

    for value in numbers.iter_mut()
    {
        *value *= 2;
    }

    println!("Modified:     {:?}", numbers);
}

//==============================================================
// 18. ITERATE WITH INDEX
//==============================================================

fn iterate_with_index()
{
    let numbers = vec![100, 200, 300];

    for (index, value) in numbers.iter().enumerate()
    {
        println!("Index: {}     Value: {}", index, value);
    }
}

//==============================================================
// 19. EXTEND VECTOR
//==============================================================

fn extend_vector()
{
    let mut numbers = vec![10, 20];

    numbers.extend([30, 40, 50]);

    println!("Extended:     {:?}", numbers);
}

//==============================================================
// 20. APPEND TWO VECTORS
//==============================================================

fn append_vectors()
{
    let mut first = vec![10, 20];
    let mut second = vec![30, 40];

    first.append(&mut second);

    println!("First:        {:?}", first);
    println!("Second:       {:?}", second);
}

//==============================================================
// 21. CLONE VECTOR
//==============================================================

fn clone_vector()
{
    let original = vec![10, 20, 30];

    let copied = original.clone();

    println!("Original:     {:?}", original);
    println!("Cloned:       {:?}", copied);
}

//==============================================================
// 22. CLEAR VECTOR
//==============================================================

fn clear_vector()
{
    let mut numbers = vec![10, 20, 30];

    numbers.clear();

    println!("After clear:  {:?}", numbers);
    println!("Length:       {}", numbers.len());
    println!("Capacity:     {}", numbers.capacity());
}

//==============================================================
// 23. TRUNCATE VECTOR
//==============================================================

fn truncate_vector()
{
    let mut numbers = vec![10, 20, 30, 40, 50];

    numbers.truncate(3);

    println!("Truncated:    {:?}", numbers);
}

//==============================================================
// 24. RESIZE VECTOR
//==============================================================

fn resize_vector()
{
    let mut numbers = vec![10, 20, 30];

    numbers.resize(5, 100);

    println!("Resized:      {:?}", numbers);
}

//==============================================================
// 25. FILL VECTOR
//==============================================================

fn fill_vector()
{
    let mut numbers = vec![0; 5];

    numbers.fill(100);

    println!("Filled:       {:?}", numbers);
}

//==============================================================
// 26. RETAIN SELECTED ELEMENTS
//==============================================================

fn retain_elements()
{
    let mut numbers = vec![10, 15, 20, 25, 30];

    numbers.retain(|value| value % 2 == 0);

    println!("Retained:     {:?}", numbers);
}

//==============================================================
// 27. SORT VECTOR
//==============================================================

fn sort_vector()
{
    let mut numbers = vec![50, 10, 40, 20, 30];

    numbers.sort();

    println!("Sorted:       {:?}", numbers);
}

//==============================================================
// 28. UNSTABLE SORT
//==============================================================

fn unstable_sort()
{
    let mut numbers = vec![50, 10, 40, 20, 30];

    numbers.sort_unstable();

    println!("Sorted:       {:?}", numbers);
}

//==============================================================
// 29. REVERSE VECTOR
//==============================================================

fn reverse_vector()
{
    let mut numbers = vec![10, 20, 30, 40];

    numbers.reverse();

    println!("Reversed:     {:?}", numbers);
}

//==============================================================
// 30. SEARCH ELEMENT
//==============================================================

fn search_element()
{
    let numbers = vec![10, 20, 30, 40];

    println!("Contains 30:  {}", numbers.contains(&30));
    println!("Contains 90:  {}", numbers.contains(&90));
}

//==============================================================
// 31. BINARY SEARCH
//==============================================================

fn binary_search()
{
    let numbers = vec![10, 20, 30, 40, 50];

    match numbers.binary_search(&30)
    {
        Ok(index) => println!("Found at:     {}", index),
        Err(index) => println!("Insert at:    {}", index),
    }
}

//==============================================================
// 32. SPLIT VECTOR
//==============================================================

fn split_vector()
{
    let numbers = vec![10, 20, 30, 40, 50];

    let (first, second) = numbers.split_at(2);

    println!("First part:   {:?}", first);
    println!("Second part:  {:?}", second);
}

//==============================================================
// 33. CHUNKS
//==============================================================

fn vector_chunks()
{
    let numbers = vec![10, 20, 30, 40, 50, 60];

    for chunk in numbers.chunks(2)
    {
        println!("Chunk:        {:?}", chunk);
    }
}

//==============================================================
// 34. WINDOWS
//==============================================================

fn vector_windows()
{
    let numbers = vec![10, 20, 30, 40];

    for window in numbers.windows(2)
    {
        println!("Window:       {:?}", window);
    }
}

//==============================================================
// 35. DRAIN ELEMENTS
//==============================================================

fn drain_vector()
{
    let mut numbers = vec![10, 20, 30, 40, 50];

    let removed: Vec<i32> = numbers.drain(1..3).collect();

    println!("Drained:      {:?}", removed);
    println!("Remaining:    {:?}", numbers);
}

//==============================================================
// 36. DEDUPLICATE CONSECUTIVE ELEMENTS
//==============================================================

fn deduplicate_vector()
{
    let mut numbers = vec![10, 10, 20, 20, 30, 10];

    numbers.dedup();

    println!("Deduplicated: {:?}", numbers);
}

//==============================================================
// 37. CONVERT ARRAY INTO VECTOR
//==============================================================

fn array_to_vector()
{
    let array = [10, 20, 30];

    let numbers = array.to_vec();

    println!("Vector:       {:?}", numbers);
}

//==============================================================
// 38. CONVERT VECTOR INTO ARRAY
//==============================================================

fn vector_to_array()
{
    let numbers = vec![10, 20, 30];

    match <Vec<i32> as TryInto<[i32; 3]>>::try_into(numbers)
    {
        Ok(array) => println!("Array:        {:?}", array),
        Err(_) => println!("Conversion failed"),
    }
}

//==============================================================
// 39. PASS VECTOR AS SLICE TO FUNCTION
//==============================================================

fn print_slice(numbers: &[i32])
{
    println!("Slice:        {:?}", numbers);
}

fn vector_as_slice()
{
    let numbers = vec![10, 20, 30];

    print_slice(&numbers);
}

//==============================================================
// 40. PASS MUTABLE VECTOR TO FUNCTION
//==============================================================

fn modify_vector(numbers: &mut Vec<i32>)
{
    numbers.push(400);
}

fn mutable_vector_function()
{
    let mut numbers = vec![100, 200, 300];

    modify_vector(&mut numbers);

    println!("Modified:     {:?}", numbers);
}

//==============================================================
// 41. RETURN VECTOR FROM FUNCTION
//==============================================================

fn create_vector() -> Vec<i32>
{
    let numbers = vec![10, 20, 30];

    numbers
}

fn return_vector()
{
    let numbers = create_vector();

    println!("Returned:     {:?}", numbers);
}

//==============================================================
// 42. VECTOR OF STRINGS
//==============================================================

fn vector_of_strings()
{
    let names = vec![
        String::from("Linux"),
        String::from("QNX"),
        String::from("FreeRTOS"),
    ];

    println!("Names:        {:?}", names);
}

//==============================================================
// 43. VECTOR OF STRUCTURES
//==============================================================

#[derive(Debug)]
struct Device
{
    id: u32,
    name: String,
}

fn vector_of_structures()
{
    let devices = vec![
        Device
        {
            id: 1,
            name: String::from("Sensor"),
        },
        Device
        {
            id: 2,
            name: String::from("Controller"),
        },
    ];

    for device in &devices
    {
        println!("ID: {}        Name: {}", device.id, device.name);
    }
}

//==============================================================
// 44. VECTOR INTO_ITER OWNERSHIP
//==============================================================

fn vector_into_iter()
{
    let numbers = vec![10, 20, 30];

    for value in numbers
    {
        println!("Value:        {}", value);
    }

    // numbers is moved into the loop.
    // It cannot be used after this point.
}

//==============================================================
// 45. MEMORY SIZE OF VECTOR
//==============================================================

fn vector_memory_size()
{
    let numbers = vec![10, 20, 30, 40];

    println!("Vector size:  {} bytes", mem::size_of_val(&numbers));
    println!("Element size: {} bytes", mem::size_of::<i32>());
    println!("Length:       {}", numbers.len());
    println!("Capacity:     {}", numbers.capacity());
}

//==============================================================
// 46. VECTOR SUM, MINIMUM AND MAXIMUM
//==============================================================

fn vector_statistics()
{
    let numbers = vec![10, 20, 30, 40, 50];

    let sum: i32 = numbers.iter().sum();

    let minimum = numbers.iter().min();
    let maximum = numbers.iter().max();

    println!("Sum:          {}", sum);
    println!("Minimum:      {:?}", minimum);
    println!("Maximum:      {:?}", maximum);
}

//==============================================================
// 47. MAIN FUNCTION
//==============================================================

fn main()
{
    vector_declaration();
    vector_initialization();
    mutable_vector();
    vector_with_capacity();
    push_element();
    pop_element();
    insert_element();
    remove_element();
    swap_remove_element();
    vector_length_capacity();
    access_using_index();
    safe_access();
    modify_element();
    modify_using_get_mut();
    iterate_vector();
    iterate_using_iter();
    iterate_using_iter_mut();
    iterate_with_index();
    extend_vector();
    append_vectors();
    clone_vector();
    clear_vector();
    truncate_vector();
    resize_vector();
    fill_vector();
    retain_elements();
    sort_vector();
    unstable_sort();
    reverse_vector();
    search_element();
    binary_search();
    split_vector();
    vector_chunks();
    vector_windows();
    drain_vector();
    deduplicate_vector();
    array_to_vector();
    vector_to_array();
    vector_as_slice();
    mutable_vector_function();
    return_vector();
    vector_of_strings();
    vector_of_structures();
    vector_into_iter();
    vector_memory_size();
    vector_statistics();
}


/*
//---------------------------------------------------
Important concepts:

Array vs Vector

Feature	Array [T; N]	Vector Vec<T>
Size	Fixed	Dynamic
Memory	Inline storage	Elements stored in heap allocation
Length	Compile-time fixed	Runtime changeable
Add elements	Not directly	push()
Remove elements	Not directly	pop()
Capacity	Fixed	Can grow
Ownership	Owns elements	Owns elements

//---------------------------------------------------


//---------------------------------------------------
Compile and execute:

rustc rust_20261105_ds_Vector_01.rs -o vector

./vector

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

Empty vector: []
Vector:       [10, 20, 30, 40, 50]
After push:   [10, 20, 30, 40]
Vector:       [100, 200]
Length:       2
Capacity:     10
After push:   [10, 20, 30, 40]
Removed:      Some(30)
Remaining:    [10, 20]
After insert: [10, 20, 30, 40]
Removed:      20
Remaining:    [10, 30, 40]
Removed:      20
Remaining:    [10, 40, 30]
Length:       3
Capacity:     8
Is empty:     false
Element:      30
Index not found
Modified:     [10, 200, 30]
Modified:     [10, 200, 30]
Value:        10
Value:        20
Value:        30
Value:        40
Value:        10
Value:        20
Value:        30
Modified:     [20, 40, 60]
Index: 0     Value: 100
Index: 1     Value: 200
Index: 2     Value: 300
Extended:     [10, 20, 30, 40, 50]
First:        [10, 20, 30, 40]
Second:       []
Original:     [10, 20, 30]
Cloned:       [10, 20, 30]
After clear:  []
Length:       0
Capacity:     3
Truncated:    [10, 20, 30]
Resized:      [10, 20, 30, 100, 100]
Filled:       [100, 100, 100, 100, 100]
Retained:     [10, 20, 30]
Sorted:       [10, 20, 30, 40, 50]
Sorted:       [10, 20, 30, 40, 50]
Reversed:     [40, 30, 20, 10]
Contains 30:  true
Contains 90:  false
Found at:     2
First part:   [10, 20]
Second part:  [30, 40, 50]
Chunk:        [10, 20]
Chunk:        [30, 40]
Chunk:        [50, 60]
Window:       [10, 20]
Window:       [20, 30]
Window:       [30, 40]
Drained:      [20, 30]
Remaining:    [10, 40, 50]
Deduplicated: [10, 20, 30, 10]
Vector:       [10, 20, 30]
Array:        [10, 20, 30]
Slice:        [10, 20, 30]
Modified:     [100, 200, 300, 400]
Returned:     [10, 20, 30]
Names:        ["Linux", "QNX", "FreeRTOS"]
ID: 1        Name: Sensor
ID: 2        Name: Controller
Value:        10
Value:        20
Value:        30
Vector size:  24 bytes
Element size: 4 bytes
Length:       4
Capacity:     4
Sum:          150
Minimum:      Some(10)
Maximum:      Some(50)

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/


