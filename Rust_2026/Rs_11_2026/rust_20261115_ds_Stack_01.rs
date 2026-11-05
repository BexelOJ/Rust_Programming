//---------------------------------------------------
// Stack - Rust Data Structures
// Date: 20261115
//---------------------------------------------------

use std::mem::size_of;

//---------------------------------------------------
// Structure
//---------------------------------------------------

#[derive(Debug, Clone)]
struct Employee {
    id: u32,
    name: String,
}

//---------------------------------------------------
// Generic Stack implementation
//---------------------------------------------------

#[derive(Debug, Clone)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {

    // Create an empty stack.
    fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    // Create a stack with initial capacity.
    fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }

    // Push an element onto the stack.
    fn push(&mut self, value: T) {
        self.items.push(value);
    }

    // Remove and return the top element.
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    // Return a reference to the top element.
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    // Return a mutable reference to the top element.
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.items.last_mut()
    }

    // Return the number of elements.
    fn len(&self) -> usize {
        self.items.len()
    }

    // Check whether the stack is empty.
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    // Return allocated capacity.
    fn capacity(&self) -> usize {
        self.items.capacity()
    }

    // Remove all elements.
    fn clear(&mut self) {
        self.items.clear();
    }

    // Reserve additional capacity.
    fn reserve(&mut self, additional: usize) {
        self.items.reserve(additional);
    }

    // Reduce allocated capacity.
    fn shrink_to_fit(&mut self) {
        self.items.shrink_to_fit();
    }

    // Iterate from bottom to top.
    fn iter(&self) -> std::slice::Iter<'_, T> {
        self.items.iter()
    }

    // Iterate mutably from bottom to top.
    fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.items.iter_mut()
    }

    // Return a slice of the stack elements.
    fn as_slice(&self) -> &[T] {
        self.items.as_slice()
    }

    // Consume the stack and return its Vec.
    fn into_vec(self) -> Vec<T> {
        self.items
    }

    // Create a stack from an existing Vec.
    fn from_vec(items: Vec<T>) -> Self {
        Self { items }
    }
}

//---------------------------------------------------
// Methods requiring PartialEq
//---------------------------------------------------

impl<T: PartialEq> Stack<T> {

    fn contains(&self, value: &T) -> bool {
        self.items.contains(value)
    }

    fn position(&self, value: &T) -> Option<usize> {
        self.items.iter().position(|item| item == value)
    }
}

//---------------------------------------------------
// Helper functions
//---------------------------------------------------

fn display_stack<T: std::fmt::Debug>(stack: &Stack<T>) {
    println!("Stack: {:?}", stack.as_slice());
}

fn reverse_stack<T>(stack: &mut Stack<T>) {
    let mut temporary = Vec::new();

    while let Some(value) = stack.pop() {
        temporary.push(value);
    }

    while let Some(value) = temporary.pop() {
        stack.push(value);
    }
}

//---------------------------------------------------
// Main
//---------------------------------------------------

fn main() {

    //---------------------------------------------------
    // 01. Creating a stack
    //---------------------------------------------------

    println!("\n01. Creating a stack");

    let mut stack: Stack<i32> = Stack::new();

    println!("Is empty: {}", stack.is_empty());

    //---------------------------------------------------
    // 02. Push operation
    //---------------------------------------------------

    println!("\n02. Push operation");

    stack.push(10);
    stack.push(20);
    stack.push(30);

    display_stack(&stack);

    //---------------------------------------------------
    // 03. Pop operation
    //---------------------------------------------------

    println!("\n03. Pop operation");

    if let Some(value) = stack.pop() {
        println!("Popped: {}", value);
    }

    display_stack(&stack);

    //---------------------------------------------------
    // 04. Peek operation
    //---------------------------------------------------

    println!("\n04. Peek operation");

    match stack.peek() {
        Some(value) => println!("Top element: {}", value),
        None => println!("Stack is empty"),
    }

    //---------------------------------------------------
    // 05. Peek without removing
    //---------------------------------------------------

    println!("\n05. Peek without removing");

    println!("Before peek: {}", stack.len());

    let top = stack.peek();

    println!("Peeked value: {:?}", top);
    println!("After peek: {}", stack.len());

    //---------------------------------------------------
    // 06. Stack length
    //---------------------------------------------------

    println!("\n06. Stack length");

    println!("Length: {}", stack.len());

    //---------------------------------------------------
    // 07. Stack capacity
    //---------------------------------------------------

    println!("\n07. Stack capacity");

    println!("Capacity: {}", stack.capacity());

    //---------------------------------------------------
    // 08. Checking empty stack
    //---------------------------------------------------

    println!("\n08. Checking empty stack");

    let empty_stack: Stack<i32> = Stack::new();

    println!("Is empty: {}", empty_stack.is_empty());

    //---------------------------------------------------
    // 09. Pop from empty stack
    //---------------------------------------------------

    println!("\n09. Pop from empty stack");

    match empty_stack.into_vec().pop() {
        Some(value) => println!("Popped: {}", value),
        None => println!("Cannot pop: stack is empty"),
    }

    //---------------------------------------------------
    // 10. Multiple push operations
    //---------------------------------------------------

    println!("\n10. Multiple push operations");

    let mut numbers = Stack::new();

    for value in 1..=5 {
        numbers.push(value * 10);
    }

    display_stack(&numbers);

    //---------------------------------------------------
    // 11. Multiple pop operations
    //---------------------------------------------------

    println!("\n11. Multiple pop operations");

    while let Some(value) = numbers.pop() {
        println!("Popped: {}", value);
    }

    //---------------------------------------------------
    // 12. Stack using String
    //---------------------------------------------------

    println!("\n12. Stack using String");

    let mut messages = Stack::new();

    messages.push(String::from("Linux"));
    messages.push(String::from("Rust"));
    messages.push(String::from("Embedded"));

    display_stack(&messages);

    println!("Top: {:?}", messages.peek());

    //---------------------------------------------------
    // 13. Stack using char
    //---------------------------------------------------

    println!("\n13. Stack using char");

    let mut characters = Stack::new();

    characters.push('A');
    characters.push('B');
    characters.push('C');

    display_stack(&characters);

    //---------------------------------------------------
    // 14. Stack using structures
    //---------------------------------------------------

    println!("\n14. Stack using structures");

    let mut employees = Stack::new();

    employees.push(Employee {
        id: 101,
        name: String::from("Employee A"),
    });

    employees.push(Employee {
        id: 102,
        name: String::from("Employee B"),
    });

    if let Some(employee) = employees.peek() {
        println!("ID: {}", employee.id);
        println!("Name: {}", employee.name);
    }

    //---------------------------------------------------
    // 15. Mutable peek
    //---------------------------------------------------

    println!("\n15. Mutable peek");

    let mut numbers = Stack::new();

    numbers.push(100);
    numbers.push(200);

    if let Some(top) = numbers.peek_mut() {
        *top = 250;
    }

    display_stack(&numbers);

    //---------------------------------------------------
    // 16. Iterating through stack
    //---------------------------------------------------

    println!("\n16. Iterating through stack");

    let mut numbers = Stack::new();

    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    for value in numbers.iter() {
        println!("Value: {}", value);
    }

    //---------------------------------------------------
    // 17. Mutable iteration
    //---------------------------------------------------

    println!("\n17. Mutable iteration");

    for value in numbers.iter_mut() {
        *value += 5;
    }

    display_stack(&numbers);

    //---------------------------------------------------
    // 18. Contains operation
    //---------------------------------------------------

    println!("\n18. Contains operation");

    println!("Contains 15: {}", numbers.contains(&15));
    println!("Contains 100: {}", numbers.contains(&100));

    //---------------------------------------------------
    // 19. Position operation
    //---------------------------------------------------

    println!("\n19. Position operation");

    println!("Position of 25: {:?}", numbers.position(&25));

    //---------------------------------------------------
    // 20. Clone a stack
    //---------------------------------------------------

    println!("\n20. Clone a stack");

    let original = numbers.clone();

    display_stack(&original);

    //---------------------------------------------------
    // 21. Reverse a stack
    //---------------------------------------------------

    println!("\n21. Reverse a stack");

    let mut numbers = Stack::new();

    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    numbers.push(4);

    println!("Before reverse:");
    display_stack(&numbers);

    reverse_stack(&mut numbers);

    println!("After reverse:");
    display_stack(&numbers);

    //---------------------------------------------------
    // 22. Clear a stack
    //---------------------------------------------------

    println!("\n22. Clear a stack");

    numbers.clear();

    println!("Length: {}", numbers.len());
    println!("Is empty: {}", numbers.is_empty());

    //---------------------------------------------------
    // 23. Stack with capacity
    //---------------------------------------------------

    println!("\n23. Stack with capacity");

    let mut numbers = Stack::with_capacity(10);

    println!("Initial capacity: {}", numbers.capacity());

    numbers.push(100);
    numbers.push(200);

    println!("Length: {}", numbers.len());
    println!("Capacity: {}", numbers.capacity());

    //---------------------------------------------------
    // 24. Reserve capacity
    //---------------------------------------------------

    println!("\n24. Reserve capacity");

    numbers.reserve(20);

    println!("Capacity after reserve: {}", numbers.capacity());

    //---------------------------------------------------
    // 25. Shrink capacity
    //---------------------------------------------------

    println!("\n25. Shrink capacity");

    numbers.shrink_to_fit();

    println!("Capacity after shrink: {}", numbers.capacity());

    //---------------------------------------------------
    // 26. Create stack from Vec
    //---------------------------------------------------

    println!("\n26. Create stack from Vec");

    let values = vec![10, 20, 30];

    let stack = Stack::from_vec(values);

    display_stack(&stack);

    //---------------------------------------------------
    // 27. Convert stack into Vec
    //---------------------------------------------------

    println!("\n27. Convert stack into Vec");

    let values = stack.into_vec();

    println!("Vector: {:?}", values);

    //---------------------------------------------------
    // 28. Stack ownership
    //---------------------------------------------------

    println!("\n28. Stack ownership");

    let mut first = Stack::new();

    first.push(500);
    first.push(600);

    let second = first;

    display_stack(&second);

    // first is no longer usable because ownership moved.

    //---------------------------------------------------
    // 29. Nested stack
    //---------------------------------------------------

    println!("\n29. Nested stack");

    let mut outer: Stack<Stack<i32>> = Stack::new();

    let mut inner = Stack::new();

    inner.push(1);
    inner.push(2);

    outer.push(inner);

    if let Some(inner_stack) = outer.peek() {
        display_stack(inner_stack);
    }

    //---------------------------------------------------
    // 30. Stack of vectors
    //---------------------------------------------------

    println!("\n30. Stack of vectors");

    let mut stack = Stack::new();

    stack.push(vec![1, 2, 3]);
    stack.push(vec![4, 5, 6]);

    display_stack(&stack);

    //---------------------------------------------------
    // 31. LIFO demonstration
    //---------------------------------------------------

    println!("\n31. LIFO demonstration");

    let mut lifo = Stack::new();

    lifo.push(100);
    lifo.push(200);
    lifo.push(300);

    while let Some(value) = lifo.pop() {
        println!("Removed: {}", value);
    }

    //---------------------------------------------------
    // 32. Parentheses balancing using stack
    //---------------------------------------------------

    println!("\n32. Parentheses balancing using stack");

    let expression = "((10 + 20) * (30 + 40))";

    let mut brackets = Stack::new();

    let mut balanced = true;

    for character in expression.chars() {

        if character == '(' {
            brackets.push(character);
        }

        if character == ')' {
            if brackets.pop().is_none() {
                balanced = false;
                break;
            }
        }
    }

    if !brackets.is_empty() {
        balanced = false;
    }

    println!("Expression: {}", expression);
    println!("Balanced: {}", balanced);

    //---------------------------------------------------
    // 33. Reversing a string using stack
    //---------------------------------------------------

    println!("\n33. Reversing a string using stack");

    let text = "RUST";

    let mut characters = Stack::new();

    for character in text.chars() {
        characters.push(character);
    }

    let mut reversed = String::new();

    while let Some(character) = characters.pop() {
        reversed.push(character);
    }

    println!("Original: {}", text);
    println!("Reversed: {}", reversed);

    //---------------------------------------------------
    // 34. Stack memory size
    //---------------------------------------------------

    println!("\n34. Stack memory size");

    println!("Stack<i32> size: {} bytes", size_of::<Stack<i32>>());
    println!("i32 size: {} bytes", size_of::<i32>());

    //---------------------------------------------------
    // 35. Standard library Vec as a stack
    //---------------------------------------------------

    println!("\n35. Standard library Vec as a stack");

    let mut standard_stack: Vec<i32> = Vec::new();

    standard_stack.push(10);
    standard_stack.push(20);
    standard_stack.push(30);

    println!("Stack: {:?}", standard_stack);

    println!("Peek: {:?}", standard_stack.last());

    println!("Pop: {:?}", standard_stack.pop());

    println!("After pop: {:?}", standard_stack);

    //---------------------------------------------------
    // 36. End of program
    //---------------------------------------------------

    println!("\n36. End of Stack demonstration");
}


/*
//---------------------------------------------------
Important Stack operations:

Operation - Time complexity - Purpose

Push - O(1) amortized - Insert at top
Pop - O(1) - Remove top
Peek - O(1) - Read top
is_empty - O(1) - Check empty
len - O(1) - Count elements
contains - O(n) - Search element
Clear - O(n) - Remove all elements

//---------------------------------------------------
Where are stacks used?

1. Function call stack and recursion.
2. Expression evaluation and parentheses validation.
3. Undo/redo functionality.
4. Depth First Search (DFS).
5. Backtracking algorithms.
6. Interrupt and exception handling concepts.
7. Parsing compilers and interpreters.

//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261115_ds_Stack_01.rs -o stack_example

./stack_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

01. Creating a stack
Is empty: true

02. Push operation
Stack: [10, 20, 30]

03. Pop operation
Popped: 30
Stack: [10, 20]

04. Peek operation
Top element: 20

05. Peek without removing
Before peek: 2
Peeked value: Some(20)
After peek: 2

06. Stack length
Length: 2

07. Stack capacity
Capacity: 4

08. Checking empty stack
Is empty: true

09. Pop from empty stack
Cannot pop: stack is empty

10. Multiple push operations
Stack: [10, 20, 30, 40, 50]

11. Multiple pop operations
Popped: 50
Popped: 40
Popped: 30
Popped: 20
Popped: 10

12. Stack using String
Stack: ["Linux", "Rust", "Embedded"]
Top: Some("Embedded")

13. Stack using char
Stack: ['A', 'B', 'C']

14. Stack using structures
ID: 102
Name: Employee B

15. Mutable peek
Stack: [100, 250]

16. Iterating through stack
Value: 10
Value: 20
Value: 30

17. Mutable iteration
Stack: [15, 25, 35]

18. Contains operation
Contains 15: true
Contains 100: false

19. Position operation
Position of 25: Some(1)

20. Clone a stack
Stack: [15, 25, 35]

21. Reverse a stack
Before reverse:
Stack: [1, 2, 3, 4]
After reverse:
Stack: [1, 2, 3, 4]

22. Clear a stack
Length: 0
Is empty: true

23. Stack with capacity
Initial capacity: 10
Length: 2
Capacity: 10

24. Reserve capacity
Capacity after reserve: 22

25. Shrink capacity
Capacity after shrink: 2

26. Create stack from Vec
Stack: [10, 20, 30]

27. Convert stack into Vec
Vector: [10, 20, 30]

28. Stack ownership
Stack: [500, 600]

29. Nested stack
Stack: [1, 2]

30. Stack of vectors
Stack: [[1, 2, 3], [4, 5, 6]]

31. LIFO demonstration
Removed: 300
Removed: 200
Removed: 100

32. Parentheses balancing using stack
Expression: ((10 + 20) * (30 + 40))
Balanced: true

33. Reversing a string using stack
Original: RUST
Reversed: TSUR

34. Stack memory size
Stack<i32> size: 24 bytes
i32 size: 4 bytes

35. Standard library Vec as a stack
Stack: [10, 20, 30]
Peek: Some(30)
Pop: Some(30)
After pop: [10, 20]

36. End of Stack demonstration

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/