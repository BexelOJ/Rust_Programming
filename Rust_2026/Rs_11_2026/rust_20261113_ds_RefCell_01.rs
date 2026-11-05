//---------------------------------------------------
// RefCell - Rust Data Structures
// Date: 20261113
//---------------------------------------------------

use std::cell::{Cell, RefCell};
use std::mem::size_of;
use std::rc::Rc;

//---------------------------------------------------
// Structures
//---------------------------------------------------

#[derive(Debug)]
struct Employee {
    id: u32,
    name: String,
    salary: RefCell<u32>,
}

#[derive(Debug)]
struct Sensor {
    id: u32,
    temperature: Cell<i32>,
}

//---------------------------------------------------
// Helper functions
//---------------------------------------------------

fn display_value(value: &RefCell<i32>) {
    println!("Value: {}", value.borrow());
}

fn modify_value(value: &RefCell<i32>, new_value: i32) {
    *value.borrow_mut() = new_value;
}

fn increment_counter(counter: &RefCell<i32>) {
    *counter.borrow_mut() += 1;
}

//---------------------------------------------------
// Main
//---------------------------------------------------

fn main() {

    //---------------------------------------------------
    // 01. Creating a RefCell
    //---------------------------------------------------

    println!("\n01. Creating a RefCell");

    let value = RefCell::new(100);

    println!("Value: {}", value.borrow());

    //---------------------------------------------------
    // 02. RefCell with mutable data
    //---------------------------------------------------

    println!("\n02. RefCell with mutable data");

    let value = RefCell::new(200);

    *value.borrow_mut() = 300;

    println!("Modified value: {}", value.borrow());

    //---------------------------------------------------
    // 03. Multiple immutable borrows
    //---------------------------------------------------

    println!("\n03. Multiple immutable borrows");

    let value = RefCell::new(400);

    let first = value.borrow();
    let second = value.borrow();

    println!("First borrow: {}", *first);
    println!("Second borrow: {}", *second);

    drop(first);
    drop(second);

    //---------------------------------------------------
    // 04. One mutable borrow
    //---------------------------------------------------

    println!("\n04. One mutable borrow");

    let value = RefCell::new(500);

    {
        let mut data = value.borrow_mut();

        *data += 100;

        println!("Inside mutable borrow: {}", *data);
    }

    println!("After mutable borrow: {}", value.borrow());

    //---------------------------------------------------
    // 05. Immutable borrow followed by mutable borrow
    //---------------------------------------------------

    println!("\n05. Immutable borrow followed by mutable borrow");

    let value = RefCell::new(600);

    {
        let data = value.borrow();

        println!("Immutable value: {}", *data);
    }

    {
        let mut data = value.borrow_mut();

        *data += 50;
    }

    println!("Updated value: {}", value.borrow());

    //---------------------------------------------------
    // 06. Multiple mutable operations
    //---------------------------------------------------

    println!("\n06. Multiple mutable operations");

    let numbers = RefCell::new(vec![10, 20, 30]);

    {
        let mut data = numbers.borrow_mut();

        data.push(40);
        data.push(50);
        data.remove(0);
    }

    println!("Vector: {:?}", numbers.borrow());

    //---------------------------------------------------
    // 07. RefCell with String
    //---------------------------------------------------

    println!("\n07. RefCell with String");

    let message = RefCell::new(String::from("Hello"));

    message.borrow_mut().push_str(" Rust");

    println!("Message: {}", message.borrow());

    //---------------------------------------------------
    // 08. RefCell with Vec
    //---------------------------------------------------

    println!("\n08. RefCell with Vec");

    let numbers = RefCell::new(vec![1, 2, 3, 4]);

    numbers.borrow_mut().push(5);

    println!("Numbers: {:?}", numbers.borrow());

    //---------------------------------------------------
    // 09. RefCell with structure
    //---------------------------------------------------

    println!("\n09. RefCell with structure");

    let employee = Employee {
        id: 101,
        name: String::from("Employee A"),
        salary: RefCell::new(50000),
    };

    println!("Employee ID: {}", employee.id);
    println!("Employee Name: {}", employee.name);

    *employee.salary.borrow_mut() += 5000;

    println!("Updated Salary: {}", employee.salary.borrow());

    //---------------------------------------------------
    // 10. RefCell inside an immutable variable
    //---------------------------------------------------

    println!("\n10. RefCell inside an immutable variable");

    let value = RefCell::new(700);

    *value.borrow_mut() = 800;

    println!("Updated value: {}", value.borrow());

    //---------------------------------------------------
    // 11. Passing RefCell to function
    //---------------------------------------------------

    println!("\n11. Passing RefCell to function");

    let value = RefCell::new(900);

    display_value(&value);

    //---------------------------------------------------
    // 12. Modifying RefCell through function
    //---------------------------------------------------

    println!("\n12. Modifying RefCell through function");

    let value = RefCell::new(1000);

    modify_value(&value, 1100);

    println!("Updated value: {}", value.borrow());

    //---------------------------------------------------
    // 13. Incrementing through function
    //---------------------------------------------------

    println!("\n13. Incrementing through function");

    let counter = RefCell::new(0);

    increment_counter(&counter);
    increment_counter(&counter);
    increment_counter(&counter);

    println!("Counter: {}", counter.borrow());

    //---------------------------------------------------
    // 14. try_borrow
    //---------------------------------------------------

    println!("\n14. try_borrow");

    let value = RefCell::new(1200);

    match value.try_borrow() {
        Ok(data) => println!("Borrowed value: {}", *data),
        Err(error) => println!("Borrow failed: {}", error),
    }

    //---------------------------------------------------
    // 15. try_borrow_mut
    //---------------------------------------------------

    println!("\n15. try_borrow_mut");

    let value = RefCell::new(1300);

    match value.try_borrow_mut() {
        Ok(mut data) => {
            *data += 100;
            println!("Modified value: {}", *data);
        }
        Err(error) => println!("Borrow failed: {}", error),
    }

    //---------------------------------------------------
    // 16. try_borrow_mut while immutable borrow exists
    //---------------------------------------------------

    println!("\n16. try_borrow_mut while immutable borrow exists");

    let value = RefCell::new(1400);

    let read_guard = value.borrow();

    match value.try_borrow_mut() {
        Ok(mut data) => {
            *data += 100;
        }
        Err(_) => {
            println!("Mutable borrow rejected");
        }
    }

    drop(read_guard);

    //---------------------------------------------------
    // 17. try_borrow while mutable borrow exists
    //---------------------------------------------------

    println!("\n17. try_borrow while mutable borrow exists");

    let value = RefCell::new(1500);

    let write_guard = value.borrow_mut();

    match value.try_borrow() {
        Ok(data) => println!("Value: {}", *data),
        Err(_) => println!("Immutable borrow rejected"),
    }

    drop(write_guard);

    //---------------------------------------------------
    // 18. Demonstrating borrow panic safely
    //---------------------------------------------------

    println!("\n18. Demonstrating borrow panic safely");

    let value = RefCell::new(1600);

    let result = std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| {
            let _first = value.borrow_mut();

            // Second mutable borrow causes runtime panic.
            let _second = value.borrow_mut();
        })
    );

    match result {
        Ok(_) => println!("No panic"),
        Err(_) => println!("Second mutable borrow caused panic"),
    }

    //---------------------------------------------------
    // 19. RefCell with Rc
    //---------------------------------------------------

    println!("\n19. RefCell with Rc");

    let shared = Rc::new(RefCell::new(1700));

    let first = Rc::clone(&shared);
    let second = Rc::clone(&shared);

    *first.borrow_mut() += 100;

    println!("Original: {}", shared.borrow());
    println!("First: {}", first.borrow());
    println!("Second: {}", second.borrow());

    //---------------------------------------------------
    // 20. Rc<RefCell<T>> with multiple owners
    //---------------------------------------------------

    println!("\n20. Rc<RefCell<T>> with multiple owners");

    let counter = Rc::new(RefCell::new(0));

    let owner1 = Rc::clone(&counter);
    let owner2 = Rc::clone(&counter);

    *owner1.borrow_mut() += 10;
    *owner2.borrow_mut() += 20;

    println!("Counter: {}", counter.borrow());
    println!("Strong count: {}", Rc::strong_count(&counter));

    //---------------------------------------------------
    // 21. Rc<RefCell<T>> with Vec
    //---------------------------------------------------

    println!("\n21. Rc<RefCell<T>> with Vec");

    let shared_numbers = Rc::new(RefCell::new(vec![10, 20]));

    let owner1 = Rc::clone(&shared_numbers);
    let owner2 = Rc::clone(&shared_numbers);

    owner1.borrow_mut().push(30);
    owner2.borrow_mut().push(40);

    println!("Shared vector: {:?}", shared_numbers.borrow());

    //---------------------------------------------------
    // 22. RefCell with Cell
    //---------------------------------------------------

    println!("\n22. RefCell with Cell");

    let counter = Cell::new(100);

    counter.set(200);

    println!("Cell value: {}", counter.get());

    //---------------------------------------------------
    // 23. Cell with structure
    //---------------------------------------------------

    println!("\n23. Cell with structure");

    let sensor = Sensor {
        id: 101,
        temperature: Cell::new(25),
    };

    sensor.temperature.set(30);

    println!("Sensor ID: {}", sensor.id);
    println!("Temperature: {}", sensor.temperature.get());

    //---------------------------------------------------
    // 24. Cell versus RefCell
    //---------------------------------------------------

    println!("\n24. Cell versus RefCell");

    let cell_value = Cell::new(300);

    cell_value.set(400);

    let refcell_value = RefCell::new(500);

    *refcell_value.borrow_mut() = 600;

    println!("Cell: {}", cell_value.get());
    println!("RefCell: {}", refcell_value.borrow());

    //---------------------------------------------------
    // 25. RefCell with Option
    //---------------------------------------------------

    println!("\n25. RefCell with Option");

    let value = RefCell::new(Some(700));

    *value.borrow_mut() = Some(800);

    println!("Option value: {:?}", value.borrow());

    //---------------------------------------------------
    // 26. RefCell with HashMap
    //---------------------------------------------------

    println!("\n26. RefCell with HashMap");

    let map = RefCell::new(std::collections::HashMap::new());

    map.borrow_mut().insert("CPU", 70);
    map.borrow_mut().insert("RAM", 60);

    println!("Map: {:?}", map.borrow());

    //---------------------------------------------------
    // 27. RefCell memory size
    //---------------------------------------------------

    println!("\n27. RefCell memory size");

    let value = RefCell::new(900);

    println!("RefCell<i32> size: {} bytes", size_of::<RefCell<i32>>());
    println!("i32 size: {} bytes", size_of::<i32>());

    println!("Value: {}", value.borrow());

    //---------------------------------------------------
    // 28. RefCell borrow scope
    //---------------------------------------------------

    println!("\n28. RefCell borrow scope");

    let value = RefCell::new(1000);

    {
        let mut data = value.borrow_mut();

        *data += 200;

        println!("Inside scope: {}", *data);
    }

    println!("Outside scope: {}", value.borrow());

    //---------------------------------------------------
    // 29. RefCell ownership
    //---------------------------------------------------

    println!("\n29. RefCell ownership");

    let first = RefCell::new(String::from("Rust"));

    let second = first;

    println!("Moved RefCell: {}", second.borrow());

    //---------------------------------------------------
    // 30. RefCell with nested data
    //---------------------------------------------------

    println!("\n30. RefCell with nested data");

    let data = RefCell::new(vec![vec![1, 2], vec![3, 4]]);

    data.borrow_mut()[0].push(5);

    println!("Nested vector: {:?}", data.borrow());

    //---------------------------------------------------
    // 31. RefCell with shared application state
    //---------------------------------------------------

    println!("\n31. RefCell with shared application state");

    let application_state = Rc::new(RefCell::new(String::from("Idle")));

    let worker1 = Rc::clone(&application_state);
    let worker2 = Rc::clone(&application_state);

    *worker1.borrow_mut() = String::from("Running");

    println!("Worker 1 state: {}", worker1.borrow());

    *worker2.borrow_mut() = String::from("Completed");

    println!("Application state: {}", application_state.borrow());

    //---------------------------------------------------
    // 32. End of program
    //---------------------------------------------------

    println!("\n32. End of RefCell demonstration");
}


/*
//---------------------------------------------------
RefCell<T> allows you to mutate data even when the 
RefCell itself is not declared mutable. 
Unlike normal Rust borrowing, its borrowing rules 
are checked at runtime rather than compile time.

Cell<T> 
– interior mutability for suitable Copy values.

RefCell<T>
– runtime-checked borrowing.

Rc<RefCell<T>>
– shared mutable ownership in a single thread.

Arc<Mutex<T>>
– shared mutable ownership across threads.

//---------------------------------------------------
Important concepts:

Method - Purpose

RefCell::new()
- Creates a RefCell

borrow()
- Immutable runtime borrow

borrow_mut()
- Mutable runtime borrow

try_borrow()
- Attempts immutable borrow without panicking

try_borrow_mut()
- Attempts mutable borrow without panicking

get_mut()
- Mutable access through &mut RefCell<T>

into_inner()
- Consumes RefCell and extracts its value

//---------------------------------------------------


//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261113_ds_RefCell_01.rs -o refcell_example

./refcell_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

01. Creating a RefCell
Value: 100

02. RefCell with mutable data
Modified value: 300

03. Multiple immutable borrows
First borrow: 400
Second borrow: 400

04. One mutable borrow
Inside mutable borrow: 600
After mutable borrow: 600

05. Immutable borrow followed by mutable borrow
Immutable value: 600
Updated value: 650

06. Multiple mutable operations
Vector: [20, 30, 40, 50]

07. RefCell with String
Message: Hello Rust

08. RefCell with Vec
Numbers: [1, 2, 3, 4, 5]

09. RefCell with structure
Employee ID: 101
Employee Name: Employee A
Updated Salary: 55000

10. RefCell inside an immutable variable
Updated value: 800

11. Passing RefCell to function
Value: 900

12. Modifying RefCell through function
Updated value: 1100

13. Incrementing through function
Counter: 3

14. try_borrow
Borrowed value: 1200

15. try_borrow_mut
Modified value: 1400

16. try_borrow_mut while immutable borrow exists
Mutable borrow rejected

17. try_borrow while mutable borrow exists
Immutable borrow rejected

18. Demonstrating borrow panic safely

thread 'main' (125501) panicked at rust_20261113_ds_RefCell_01.rs:317:33:
RefCell already borrowed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Second mutable borrow caused panic

19. RefCell with Rc
Original: 1800
First: 1800
Second: 1800

20. Rc<RefCell<T>> with multiple owners
Counter: 30
Strong count: 3

21. Rc<RefCell<T>> with Vec
Shared vector: [10, 20, 30, 40]

22. RefCell with Cell
Cell value: 200

23. Cell with structure
Sensor ID: 101
Temperature: 30

24. Cell versus RefCell
Cell: 400
RefCell: 600

25. RefCell with Option
Option value: Some(800)

26. RefCell with HashMap
Map: {"CPU": 70, "RAM": 60}

27. RefCell memory size
RefCell<i32> size: 16 bytes
i32 size: 4 bytes
Value: 900

28. RefCell borrow scope
Inside scope: 1200
Outside scope: 1200

29. RefCell ownership
Moved RefCell: Rust

30. RefCell with nested data
Nested vector: [[1, 2, 5], [3, 4]]

31. RefCell with shared application state
Worker 1 state: Running
Application state: Completed

32. End of RefCell demonstration

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/
