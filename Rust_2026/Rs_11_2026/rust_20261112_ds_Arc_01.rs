//---------------------------------------------------
// Arc - Rust Data Structures
// Date: 20261112
//---------------------------------------------------

use std::mem::size_of;
use std::sync::{Arc, Mutex, RwLock, Weak};
use std::thread;
use std::time::Duration;

//---------------------------------------------------
// Structures and functions
//---------------------------------------------------

#[derive(Debug)]
struct Employee {
    id: u32,
    name: String,
}

trait Device {
    fn display(&self);
}

struct Sensor {
    id: u32,
}

impl Device for Sensor {
    fn display(&self) {
        println!("Sensor ID: {}", self.id);
    }
}

fn display_arc(value: &Arc<i32>) {
    println!("Value: {}", value);
}

fn create_arc() -> Arc<String> {
    Arc::new(String::from("Created inside function"))
}

fn consume_arc(value: Arc<String>) {
    println!("Consumed: {}", value);
}

//---------------------------------------------------
// Main
//---------------------------------------------------

fn main() {

    //---------------------------------------------------
    // 01. Creating an Arc
    //---------------------------------------------------

    println!("\n01. Creating an Arc");

    let number = Arc::new(100);

    println!("Value: {}", number);
    println!("Strong count: {}", Arc::strong_count(&number));

    //---------------------------------------------------
    // 02. Cloning an Arc
    //---------------------------------------------------

    println!("\n02. Cloning an Arc");

    let first = Arc::new(200);

    let second = Arc::clone(&first);
    let third = Arc::clone(&first);

    println!("First: {}", first);
    println!("Second: {}", second);
    println!("Third: {}", third);

    println!("Strong count: {}", Arc::strong_count(&first));

    //---------------------------------------------------
    // 03. Reference counting
    //---------------------------------------------------

    println!("\n03. Reference counting");

    let data = Arc::new(300);

    println!("Initial count: {}", Arc::strong_count(&data));

    let data2 = Arc::clone(&data);

    println!("After clone: {}", Arc::strong_count(&data));

    drop(data2);

    println!("After drop: {}", Arc::strong_count(&data));

    //---------------------------------------------------
    // 04. Sharing Arc between threads
    //---------------------------------------------------

    println!("\n04. Sharing Arc between threads");

    let shared = Arc::new(400);

    let shared1 = Arc::clone(&shared);
    let shared2 = Arc::clone(&shared);

    let thread1 = thread::spawn(move || {
        println!("Thread 1: {}", shared1);
    });

    let thread2 = thread::spawn(move || {
        println!("Thread 2: {}", shared2);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("Main thread: {}", shared);

    //---------------------------------------------------
    // 05. Multiple threads reading shared data
    //---------------------------------------------------

    println!("\n05. Multiple threads reading shared data");

    let shared_data = Arc::new(String::from("Rust Arc"));

    let mut handles = Vec::new();

    for id in 1..=4 {

        let data = Arc::clone(&shared_data);

        let handle = thread::spawn(move || {
            println!("Thread {}: {}", id, data);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    //---------------------------------------------------
    // 06. Arc with String
    //---------------------------------------------------

    println!("\n06. Arc with String");

    let message = Arc::new(String::from("Embedded Rust"));

    let message2 = Arc::clone(&message);

    println!("Message 1: {}", message);
    println!("Message 2: {}", message2);

    //---------------------------------------------------
    // 07. Arc with Vec
    //---------------------------------------------------

    println!("\n07. Arc with Vec");

    let numbers = Arc::new(vec![10, 20, 30, 40, 50]);

    let numbers2 = Arc::clone(&numbers);

    println!("Numbers: {:?}", numbers);
    println!("Numbers 2: {:?}", numbers2);

    //---------------------------------------------------
    // 08. Arc with structure
    //---------------------------------------------------

    println!("\n08. Arc with structure");

    let employee = Arc::new(Employee {
        id: 101,
        name: String::from("Bexel"),
    });

    let employee2 = Arc::clone(&employee);

    println!("Employee 1 ID: {}", employee.id);
    println!("Employee 1 Name: {}", employee.name);

    println!("Employee 2 ID: {}", employee2.id);
    println!("Employee 2 Name: {}", employee2.name);

    //---------------------------------------------------
    // 09. Passing Arc to function
    //---------------------------------------------------

    println!("\n09. Passing Arc to function");

    let value = Arc::new(500);

    display_arc(&value);

    //---------------------------------------------------
    // 10. Returning Arc from function
    //---------------------------------------------------

    println!("\n10. Returning Arc from function");

    let result = create_arc();

    println!("Returned: {}", result);

    //---------------------------------------------------
    // 11. Moving Arc into function
    //---------------------------------------------------

    println!("\n11. Moving Arc into function");

    let text = Arc::new(String::from("Rust ownership"));

    consume_arc(Arc::clone(&text));

    println!("Original still available: {}", text);

    //---------------------------------------------------
    // 12. Checking pointer equality
    //---------------------------------------------------

    println!("\n12. Checking pointer equality");

    let first = Arc::new(600);

    let second = Arc::clone(&first);

    let third = Arc::new(600);

    println!(
        "First and second share allocation: {}",
        Arc::ptr_eq(&first, &second)
    );

    println!(
        "First and third share allocation: {}",
        Arc::ptr_eq(&first, &third)
    );

    //---------------------------------------------------
    // 13. Value equality
    //---------------------------------------------------

    println!("\n13. Value equality");

    let first = Arc::new(700);
    let second = Arc::new(700);

    println!("Same value: {}", first == second);

    println!(
        "Same allocation: {}",
        Arc::ptr_eq(&first, &second)
    );

    //---------------------------------------------------
    // 14. Dereferencing Arc
    //---------------------------------------------------

    println!("\n14. Dereferencing Arc");

    let value = Arc::new(800);

    println!("Dereferenced value: {}", *value);

    //---------------------------------------------------
    // 15. Arc is immutable by default
    //---------------------------------------------------

    println!("\n15. Arc is immutable by default");

    let value = Arc::new(900);

    // This is not allowed:
    // *value = 1000;

    println!("Value: {}", value);

    //---------------------------------------------------
    // 16. Arc::get_mut
    //---------------------------------------------------

    println!("\n16. Arc::get_mut");

    let mut value = Arc::new(1000);

    if let Some(data) = Arc::get_mut(&mut value) {
        *data = 1100;
    }

    println!("Modified value: {}", value);

    //---------------------------------------------------
    // 17. Arc::get_mut with multiple owners
    //---------------------------------------------------

    println!("\n17. Arc::get_mut with multiple owners");

    let mut value = Arc::new(1200);

    let second = Arc::clone(&value);

    match Arc::get_mut(&mut value) {
        Some(data) => println!("Mutable access: {}", data),
        None => println!("Cannot get mutable reference: multiple owners"),
    }

    drop(second);

    //---------------------------------------------------
    // 18. Arc::make_mut
    //---------------------------------------------------

    println!("\n18. Arc::make_mut");

    let mut first = Arc::new(1300);

    let second = Arc::clone(&first);

    *Arc::make_mut(&mut first) = 1400;

    println!("First: {}", first);
    println!("Second: {}", second);

    //---------------------------------------------------
    // 19. Arc::try_unwrap
    //---------------------------------------------------

    println!("\n19. Arc::try_unwrap");

    let value = Arc::new(1500);

    match Arc::try_unwrap(value) {
        Ok(data) => println!("Ownership extracted: {}", data),
        Err(shared) => println!("Still shared: {}", shared),
    }

    //---------------------------------------------------
    // 20. Creating Weak reference
    //---------------------------------------------------

    println!("\n20. Creating Weak reference");

    let value = Arc::new(1600);

    let weak: Weak<i32> = Arc::downgrade(&value);

    println!("Strong count: {}", Arc::strong_count(&value));
    println!("Weak count: {}", Arc::weak_count(&value));

    //---------------------------------------------------
    // 21. Upgrading Weak reference
    //---------------------------------------------------

    println!("\n21. Upgrading Weak reference");

    match weak.upgrade() {
        Some(data) => println!("Upgraded value: {}", data),
        None => println!("Original allocation no longer exists"),
    }

    //---------------------------------------------------
    // 22. Weak after strong reference is dropped
    //---------------------------------------------------

    println!("\n22. Weak after strong reference is dropped");

    let weak_reference;

    {
        let value = Arc::new(1700);

        weak_reference = Arc::downgrade(&value);

        println!("Before drop: {}", Arc::strong_count(&value));
    }

    match weak_reference.upgrade() {
        Some(data) => println!("Value: {}", data),
        None => println!("Strong owner dropped; data is unavailable"),
    }

    //---------------------------------------------------
    // 23. Arc with Mutex
    //---------------------------------------------------

    println!("\n23. Arc with Mutex");

    let counter = Arc::new(Mutex::new(0));

    {
        let mut value = counter.lock().unwrap();

        *value += 10;
    }

    println!("Counter: {}", *counter.lock().unwrap());

    //---------------------------------------------------
    // 24. Arc<Mutex<T>> with multiple threads
    //---------------------------------------------------

    println!("\n24. Arc<Mutex<T>> with multiple threads");

    let counter = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();

    for _ in 0..5 {

        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {

            for _ in 0..1000 {

                let mut value = counter_clone.lock().unwrap();

                *value += 1;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter: {}", *counter.lock().unwrap());

    //---------------------------------------------------
    // 25. Arc with RwLock
    //---------------------------------------------------

    println!("\n25. Arc with RwLock");

    let data = Arc::new(RwLock::new(2000));

    {
        let read_value = data.read().unwrap();

        println!("Read value: {}", *read_value);
    }

    {
        let mut write_value = data.write().unwrap();

        *write_value += 500;
    }

    println!("Updated value: {}", *data.read().unwrap());

    //---------------------------------------------------
    // 26. Arc<RwLock<T>> with multiple readers
    //---------------------------------------------------

    println!("\n26. Arc<RwLock<T>> with multiple readers");

    let shared = Arc::new(RwLock::new(3000));

    let mut handles = Vec::new();

    for id in 1..=3 {

        let data = Arc::clone(&shared);

        let handle = thread::spawn(move || {

            let value = data.read().unwrap();

            println!("Reader {}: {}", id, *value);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    //---------------------------------------------------
    // 27. Arc with trait object
    //---------------------------------------------------

    println!("\n27. Arc with trait object");

    let device: Arc<dyn Device> = Arc::new(Sensor { id: 101 });

    let device2 = Arc::clone(&device);

    device.display();
    device2.display();

    //---------------------------------------------------
    // 28. Arc memory size
    //---------------------------------------------------

    println!("\n28. Arc memory size");

    let value = Arc::new(100);

    println!("Arc handle size: {} bytes", size_of::<Arc<i32>>());
    println!("i32 size: {} bytes", size_of::<i32>());

    println!("Strong count: {}", Arc::strong_count(&value));

    //---------------------------------------------------
    // 29. Arc clone versus deep clone
    //---------------------------------------------------

    println!("\n29. Arc clone versus deep clone");

    let original = Arc::new(vec![1, 2, 3, 4]);

    let shared = Arc::clone(&original);

    let independent = Arc::new((*original).clone());

    println!("Original: {:?}", original);
    println!("Shared: {:?}", shared);
    println!("Independent: {:?}", independent);

    println!(
        "Original and shared allocation: {}",
        Arc::ptr_eq(&original, &shared)
    );

    println!(
        "Original and independent allocation: {}",
        Arc::ptr_eq(&original, &independent)
    );

    //---------------------------------------------------
    // 30. Arc ownership across threads
    //---------------------------------------------------

    println!("\n30. Arc ownership across threads");

    let shared = Arc::new(String::from("Shared across threads"));

    let thread_data = Arc::clone(&shared);

    let handle = thread::spawn(move || {

        println!("Worker: {}", thread_data);

        thread::sleep(Duration::from_millis(100));

        println!("Worker completed");
    });

    handle.join().unwrap();

    println!("Main: {}", shared);

    //---------------------------------------------------
    // 31. Arc reference counting demonstration
    //---------------------------------------------------

    println!("\n31. Arc reference counting demonstration");

    let data = Arc::new(4000);

    println!("Initial: {}", Arc::strong_count(&data));

    let data1 = Arc::clone(&data);

    println!("After clone 1: {}", Arc::strong_count(&data));

    let data2 = Arc::clone(&data);

    println!("After clone 2: {}", Arc::strong_count(&data));

    drop(data1);

    println!("After dropping data1: {}", Arc::strong_count(&data));

    drop(data2);

    println!("After dropping data2: {}", Arc::strong_count(&data));

    //---------------------------------------------------
    // 32. End of program
    //---------------------------------------------------

    println!("\n32. End of Arc demonstration");

}


/*
//---------------------------------------------------
Important concepts to understand:

Concept - Purpose

Arc::new()
- Creates shared ownership

Arc::clone()
- Creates another owner

Arc::strong_count()
- Counts strong references

Arc::downgrade()
- Creates a weak reference

Weak::upgrade()
- Attempts to obtain a strong reference

Arc::get_mut()
- Mutable access when uniquely owned

Arc::make_mut()
- Copy-on-write mutation

Arc::try_unwrap()
- Extracts data when uniquely owned

Arc<Mutex<T>>
- Shared mutable data with exclusive locking

Arc<RwLock<T>>
- Shared data with multiple readers or one writer

//---------------------------------------------------


//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261112_ds_Arc_01.rs -o arc_example

./arc_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

01. Creating an Arc
Value: 100
Strong count: 1

02. Cloning an Arc
First: 200
Second: 200
Third: 200
Strong count: 3

03. Reference counting
Initial count: 1
After clone: 2
After drop: 1

04. Sharing Arc between threads
Thread 1: 400
Thread 2: 400
Main thread: 400

05. Multiple threads reading shared data
Thread 1: Rust Arc
Thread 2: Rust Arc
Thread 3: Rust Arc
Thread 4: Rust Arc

06. Arc with String
Message 1: Embedded Rust
Message 2: Embedded Rust

07. Arc with Vec
Numbers: [10, 20, 30, 40, 50]
Numbers 2: [10, 20, 30, 40, 50]

08. Arc with structure
Employee 1 ID: 101
Employee 1 Name: Bexel
Employee 2 ID: 101
Employee 2 Name: Bexel

09. Passing Arc to function
Value: 500

10. Returning Arc from function
Returned: Created inside function

11. Moving Arc into function
Consumed: Rust ownership
Original still available: Rust ownership

12. Checking pointer equality
First and second share allocation: true
First and third share allocation: false

13. Value equality
Same value: true
Same allocation: false

14. Dereferencing Arc
Dereferenced value: 800

15. Arc is immutable by default
Value: 900

16. Arc::get_mut
Modified value: 1100

17. Arc::get_mut with multiple owners
Cannot get mutable reference: multiple owners

18. Arc::make_mut
First: 1400
Second: 1300

19. Arc::try_unwrap
Ownership extracted: 1500

20. Creating Weak reference
Strong count: 1
Weak count: 1

21. Upgrading Weak reference
Upgraded value: 1600

22. Weak after strong reference is dropped
Before drop: 1
Strong owner dropped; data is unavailable

23. Arc with Mutex
Counter: 10

24. Arc<Mutex<T>> with multiple threads
Final counter: 5000

25. Arc with RwLock
Read value: 2000
Updated value: 2500

26. Arc<RwLock<T>> with multiple readers
Reader 1: 3000
Reader 3: 3000
Reader 2: 3000

27. Arc with trait object
Sensor ID: 101
Sensor ID: 101

28. Arc memory size
Arc handle size: 8 bytes
i32 size: 4 bytes
Strong count: 1

29. Arc clone versus deep clone
Original: [1, 2, 3, 4]
Shared: [1, 2, 3, 4]
Independent: [1, 2, 3, 4]
Original and shared allocation: true
Original and independent allocation: false

30. Arc ownership across threads
Worker: Shared across threads
Worker completed
Main: Shared across threads

31. Arc reference counting demonstration
Initial: 1
After clone 1: 2
After clone 2: 3
After dropping data1: 2
After dropping data2: 1

32. End of Arc demonstration

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/



