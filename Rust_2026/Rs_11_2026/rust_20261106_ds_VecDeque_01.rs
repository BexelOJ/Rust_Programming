//---------------------------------------------------
// VecDeque - Rust Data Structures
// Date: 20261106
//---------------------------------------------------

use std::collections::VecDeque;
use std::mem;

//==============================================================
// 01. VECDEQUE DECLARATION
//==============================================================

fn deque_declaration()
{
    let numbers: VecDeque<i32> = VecDeque::new();

    println!("Empty deque:  {:?}", numbers);
}

//==============================================================
// 02. INITIALIZATION USING MACRO
//==============================================================

fn deque_initialization()
{
    let numbers: VecDeque<i32> = VecDeque::from([10, 20, 30, 40]);

    println!("Deque:        {:?}", numbers);
}

//==============================================================
// 03. MUTABLE VECDEQUE
//==============================================================

fn mutable_deque()
{
    let mut numbers = VecDeque::from([10, 20, 30]);

    numbers.push_back(40);

    println!("After push:   {:?}", numbers);
}

//==============================================================
// 04. PUSH ELEMENT AT BACK
//==============================================================

fn push_back_element()
{
    let mut numbers = VecDeque::new();

    numbers.push_back(10);
    numbers.push_back(20);
    numbers.push_back(30);

    println!("Deque:        {:?}", numbers);
}

//==============================================================
// 05. PUSH ELEMENT AT FRONT
//==============================================================

fn push_front_element()
{
    let mut numbers = VecDeque::from([20, 30]);

    numbers.push_front(10);

    println!("Deque:        {:?}", numbers);
}

//==============================================================
// 06. POP ELEMENT FROM BACK
//==============================================================

fn pop_back_element()
{
    let mut numbers = VecDeque::from([10, 20, 30]);

    let removed = numbers.pop_back();

    println!("Removed:      {:?}", removed);
    println!("Remaining:    {:?}", numbers);
}

//==============================================================
// 07. POP ELEMENT FROM FRONT
//==============================================================

fn pop_front_element()
{
    let mut numbers = VecDeque::from([10, 20, 30]);

    let removed = numbers.pop_front();

    println!("Removed:      {:?}", removed);
    println!("Remaining:    {:?}", numbers);
}

//==============================================================
// 08. INSERT ELEMENT AT INDEX
//==============================================================

fn insert_element()
{
    let mut numbers = VecDeque::from([10, 20, 40]);

    numbers.insert(2, 30);

    println!("After insert: {:?}", numbers);
}

//==============================================================
// 09. REMOVE ELEMENT AT INDEX
//==============================================================

fn remove_element()
{
    let mut numbers = VecDeque::from([10, 20, 30, 40]);

    let removed = numbers.remove(1);

    println!("Removed:      {:?}", removed);
    println!("Remaining:    {:?}", numbers);
}

//==============================================================
// 10. GET ELEMENT USING INDEX
//==============================================================

fn access_element()
{
    let numbers = VecDeque::from([10, 20, 30, 40]);

    println!("Element:      {}", numbers[2]);
}

//==============================================================
// 11. SAFE ACCESS USING GET
//==============================================================

fn safe_access()
{
    let numbers = VecDeque::from([10, 20, 30]);

    match numbers.get(5)
    {
        Some(value) => println!("Value:        {}", value),
        None => println!("Index not found"),
    }
}

//==============================================================
// 12. MODIFY ELEMENT
//==============================================================

fn modify_element()
{
    let mut numbers = VecDeque::from([10, 20, 30]);

    numbers[1] = 200;

    println!("Modified:     {:?}", numbers);
}

//==============================================================
// 13. GET FRONT ELEMENT
//==============================================================

fn get_front()
{
    let numbers = VecDeque::from([10, 20, 30]);

    println!("Front:        {:?}", numbers.front());
}

//==============================================================
// 14. GET BACK ELEMENT
//==============================================================

fn get_back()
{
    let numbers = VecDeque::from([10, 20, 30]);

    println!("Back:         {:?}", numbers.back());
}

//==============================================================
// 15. FRONT MUTABLE REFERENCE
//==============================================================

fn front_mut()
{
    let mut numbers = VecDeque::from([10, 20, 30]);

    if let Some(value) = numbers.front_mut()
    {
        *value = 100;
    }

    println!("Modified:     {:?}", numbers);
}

//==============================================================
// 16. BACK MUTABLE REFERENCE
//==============================================================

fn back_mut()
{
    let mut numbers = VecDeque::from([10, 20, 30]);

    if let Some(value) = numbers.back_mut()
    {
        *value = 300;
    }

    println!("Modified:     {:?}", numbers);
}

//==============================================================
// 17. LENGTH AND CAPACITY
//==============================================================

fn deque_length_capacity()
{
    let mut numbers = VecDeque::with_capacity(10);

    numbers.push_back(10);
    numbers.push_back(20);

    println!("Length:       {}", numbers.len());
    println!("Capacity:     {}", numbers.capacity());
    println!("Is empty:     {}", numbers.is_empty());
}

//==============================================================
// 18. RESERVE CAPACITY
//==============================================================

fn reserve_capacity()
{
    let mut numbers: VecDeque<i32> = VecDeque::new();

    numbers.reserve(10);

    println!("Capacity:     {}", numbers.capacity());
}

//==============================================================
// 19. SHRINK TO FIT
//==============================================================

fn shrink_capacity()
{
    let mut numbers = VecDeque::with_capacity(100);

    numbers.push_back(10);
    numbers.push_back(20);

    numbers.shrink_to_fit();

    println!("Length:       {}", numbers.len());
    println!("Capacity:     {}", numbers.capacity());
}

//==============================================================
// 20. ITERATE USING FOR
//==============================================================

fn iterate_deque()
{
    let numbers = VecDeque::from([10, 20, 30]);

    for value in &numbers
    {
        println!("Value:        {}", value);
    }
}

//==============================================================
// 21. ITERATE USING ITER_MUT
//==============================================================

fn iterate_mut()
{
    let mut numbers = VecDeque::from([10, 20, 30]);

    for value in numbers.iter_mut()
    {
        *value *= 2;
    }

    println!("Modified:     {:?}", numbers);
}

//==============================================================
// 22. ITERATE WITH INDEX
//==============================================================

fn iterate_with_index()
{
    let numbers = VecDeque::from([100, 200, 300]);

    for (index, value) in numbers.iter().enumerate()
    {
        println!("Index: {}     Value: {}", index, value);
    }
}

//==============================================================
// 23. CLEAR DEQUE
//==============================================================

fn clear_deque()
{
    let mut numbers = VecDeque::from([10, 20, 30]);

    numbers.clear();

    println!("After clear:  {:?}", numbers);
    println!("Length:       {}", numbers.len());
    println!("Capacity:     {}", numbers.capacity());
}

//==============================================================
// 24. EXTEND DEQUE
//==============================================================

fn extend_deque()
{
    let mut numbers = VecDeque::from([10, 20]);

    numbers.extend([30, 40, 50]);

    println!("Extended:     {:?}", numbers);
}

//==============================================================
// 25. APPEND TWO DEQUES
//==============================================================

fn append_deques()
{
    let mut first = VecDeque::from([10, 20]);
    let mut second = VecDeque::from([30, 40]);

    first.append(&mut second);

    println!("First:        {:?}", first);
    println!("Second:       {:?}", second);
}

//==============================================================
// 26. ROTATE LEFT
//==============================================================

fn rotate_left()
{
    let mut numbers = VecDeque::from([10, 20, 30, 40, 50]);

    numbers.rotate_left(2);

    println!("Rotated left: {:?}", numbers);
}

//==============================================================
// 27. ROTATE RIGHT
//==============================================================

fn rotate_right()
{
    let mut numbers = VecDeque::from([10, 20, 30, 40, 50]);

    numbers.rotate_right(2);

    println!("Rotated right:{:?}", numbers);
}

//==============================================================
// 28. MAKE CONTIGUOUS
//==============================================================

fn make_contiguous()
{
    let mut numbers = VecDeque::from([10, 20, 30, 40]);

    numbers.pop_front();
    numbers.push_back(50);

    let slice = numbers.make_contiguous();

    println!("Contiguous:   {:?}", slice);
}

//==============================================================
// 29. AS SLICES
//==============================================================

fn deque_as_slices()
{
    let mut numbers = VecDeque::from([10, 20, 30, 40, 50]);

    numbers.pop_front();
    numbers.pop_front();
    numbers.push_back(60);
    numbers.push_back(70);

    let (first, second) = numbers.as_slices();

    println!("First slice:  {:?}", first);
    println!("Second slice: {:?}", second);
}

//==============================================================
// 30. CONTAINS ELEMENT
//==============================================================

fn contains_element()
{
    let numbers = VecDeque::from([10, 20, 30]);

    println!("Contains 20:  {}", numbers.contains(&20));
    println!("Contains 90:  {}", numbers.contains(&90));
}

//==============================================================
// 31. SPLIT OFF
//==============================================================

fn split_off_deque()
{
    let mut numbers = VecDeque::from([10, 20, 30, 40, 50]);

    let second = numbers.split_off(2);

    println!("First deque:  {:?}", numbers);
    println!("Second deque: {:?}", second);
}

//==============================================================
// 32. DRAIN ELEMENTS
//==============================================================

fn drain_deque()
{
    let mut numbers = VecDeque::from([10, 20, 30, 40, 50]);

    let removed: Vec<i32> = numbers.drain(1..3).collect();

    println!("Drained:      {:?}", removed);
    println!("Remaining:    {:?}", numbers);
}

//==============================================================
// 33. RETAIN ELEMENTS
//==============================================================

fn retain_elements()
{
    let mut numbers = VecDeque::from([10, 15, 20, 25, 30]);

    numbers.retain(|value| value % 2 == 0);

    println!("Retained:     {:?}", numbers);
}

//==============================================================
// 34. SORT DEQUE
//==============================================================

fn sort_deque()
{
    let mut numbers = VecDeque::from([50, 10, 40, 20, 30]);

    numbers.make_contiguous().sort();

    println!("Sorted:       {:?}", numbers);
}

//==============================================================
// 35. BINARY SEARCH
//==============================================================

fn binary_search()
{
    let mut numbers = VecDeque::from([50, 10, 40, 20, 30]);

    numbers.make_contiguous().sort();

    match numbers.binary_search(&30)
    {
        Ok(index) => println!("Found at:     {}", index),
        Err(index) => println!("Insert at:    {}", index),
    }
}

//==============================================================
// 36. CONVERT VECDEQUE INTO VECTOR
//==============================================================

fn deque_to_vector()
{
    let deque = VecDeque::from([10, 20, 30]);

    let numbers: Vec<i32> = deque.into();

    println!("Vector:       {:?}", numbers);
}

//==============================================================
// 37. CONVERT VECTOR INTO VECDEQUE
//==============================================================

fn vector_to_deque()
{
    let numbers = vec![10, 20, 30];

    let deque: VecDeque<i32> = numbers.into();

    println!("Deque:        {:?}", deque);
}

//==============================================================
// 38. PASS DEQUE AS SLICE
//==============================================================

fn print_slice(numbers: &[i32])
{
    println!("Slice:        {:?}", numbers);
}

fn deque_as_slice()
{
    let mut numbers = VecDeque::from([10, 20, 30]);

    let slice = numbers.make_contiguous();

    print_slice(slice);
}

//==============================================================
// 39. VECTOR OF STRINGS
//==============================================================

fn deque_of_strings()
{
    let mut messages: VecDeque<String> = VecDeque::new();

    messages.push_back(String::from("Message 1"));
    messages.push_front(String::from("Urgent Message"));

    println!("Messages:     {:?}", messages);
}

//==============================================================
// 40. MEMORY SIZE
//==============================================================

fn deque_memory_size()
{
    let numbers = VecDeque::from([10, 20, 30, 40]);

    println!("Deque size:   {} bytes", mem::size_of_val(&numbers));
    println!("Element size: {} bytes", mem::size_of::<i32>());
    println!("Length:       {}", numbers.len());
    println!("Capacity:     {}", numbers.capacity());
}

//==============================================================
// 41. QUEUE BEHAVIOUR
//==============================================================

fn queue_behaviour()
{
    let mut queue = VecDeque::new();

    // Insert at rear.
    queue.push_back(10);
    queue.push_back(20);
    queue.push_back(30);

    // Remove from front.
    println!("Processed:    {:?}", queue.pop_front());
    println!("Processed:    {:?}", queue.pop_front());

    println!("Remaining:    {:?}", queue);
}

//==============================================================
// 42. STACK BEHAVIOUR
//==============================================================

fn stack_behaviour()
{
    let mut stack = VecDeque::new();

    // Push elements at back.
    stack.push_back(10);
    stack.push_back(20);
    stack.push_back(30);

    // Pop elements from back.
    println!("Popped:       {:?}", stack.pop_back());
    println!("Popped:       {:?}", stack.pop_back());

    println!("Remaining:    {:?}", stack);
}

//==============================================================
// 43. MAIN FUNCTION
//==============================================================

fn main()
{
    deque_declaration();
    deque_initialization();
    mutable_deque();
    push_back_element();
    push_front_element();
    pop_back_element();
    pop_front_element();
    insert_element();
    remove_element();
    access_element();
    safe_access();
    modify_element();
    get_front();
    get_back();
    front_mut();
    back_mut();
    deque_length_capacity();
    reserve_capacity();
    shrink_capacity();
    iterate_deque();
    iterate_mut();
    iterate_with_index();
    clear_deque();
    extend_deque();
    append_deques();
    rotate_left();
    rotate_right();
    make_contiguous();
    deque_as_slices();
    contains_element();
    split_off_deque();
    drain_deque();
    retain_elements();
    sort_deque();
    binary_search();
    deque_to_vector();
    vector_to_deque();
    deque_as_slice();
    deque_of_strings();
    deque_memory_size();
    queue_behaviour();
    stack_behaviour();
}

/*
//---------------------------------------------------

Vec vs VecDeque
Operation	Vec	VecDeque
Add at end	push()	push_back()
Remove at end	pop()	pop_back()
Add at front	Costly	push_front()
Remove at front	Costly	pop_front()
Random access	Yes	Yes
Dynamic growth	Yes	Yes
Circular buffer	No	Yes

//---------------------------------------------------
Where is VecDeque useful?

- Operating-system scheduling queues.
- Network packet buffering.
- Producer-consumer systems.
- Message processing.
- Sliding-window algorithms.
- BFS graph traversal.
- Double-ended processing.

//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261106_ds_VecDeque_01.rs -o vecdeque

./vecdeque

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

Empty deque:  []
Deque:        [10, 20, 30, 40]
After push:   [10, 20, 30, 40]
Deque:        [10, 20, 30]
Deque:        [10, 20, 30]
Removed:      Some(30)
Remaining:    [10, 20]
Removed:      Some(10)
Remaining:    [20, 30]
After insert: [10, 20, 30, 40]
Removed:      Some(20)
Remaining:    [10, 30, 40]
Element:      30
Index not found
Modified:     [10, 200, 30]
Front:        Some(10)
Back:         Some(30)
Modified:     [100, 20, 30]
Modified:     [10, 20, 300]
Length:       2
Capacity:     10
Is empty:     false
Capacity:     10
Length:       2
Capacity:     2
Value:        10
Value:        20
Value:        30
Modified:     [20, 40, 60]
Index: 0     Value: 100
Index: 1     Value: 200
Index: 2     Value: 300
After clear:  []
Length:       0
Capacity:     3
Extended:     [10, 20, 30, 40, 50]
First:        [10, 20, 30, 40]
Second:       []
Rotated left: [30, 40, 50, 10, 20]
Rotated right:[40, 50, 10, 20, 30]
Contiguous:   [20, 30, 40, 50]
First slice:  [30, 40, 50]
Second slice: [60, 70]
Contains 20:  true
Contains 90:  false
First deque:  [10, 20]
Second deque: [30, 40, 50]
Drained:      [20, 30]
Remaining:    [10, 40, 50]
Retained:     [10, 20, 30]
Sorted:       [10, 20, 30, 40, 50]
Found at:     2
Vector:       [10, 20, 30]
Deque:        [10, 20, 30]
Slice:        [10, 20, 30]
Messages:     ["Urgent Message", "Message 1"]
Deque size:   32 bytes
Element size: 4 bytes
Length:       4
Capacity:     4
Processed:    Some(10)
Processed:    Some(20)
Remaining:    [30]
Popped:       Some(30)
Popped:       Some(20)
Remaining:    [10]

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/


