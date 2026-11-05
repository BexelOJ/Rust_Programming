//---------------------------------------------------
// Rc - Rust Data Structures
// Date: 20261111
//---------------------------------------------------

use std::mem::size_of;
use std::rc::{Rc, Weak};

//--------------------------------------------------
// Employee structure
//--------------------------------------------------
#[derive(Debug)]
struct Employee {
    id: u32,
    name: String,
}

//--------------------------------------------------
// Department structure
//--------------------------------------------------
#[derive(Debug)]
struct Department {
    name: String,
    employees: Vec<Rc<Employee>>,
}

//--------------------------------------------------
// Device trait
//--------------------------------------------------
trait Device {
    fn device_name(&self) -> &str;
}

struct Sensor {
    name: String,
}

impl Device for Sensor {
    fn device_name(&self) -> &str {
        &self.name
    }
}

//--------------------------------------------------
// Function accepting Rc
//--------------------------------------------------
fn display_number(value: Rc<i32>) {
    println!("Value inside function: {}", value);
    println!("Strong count inside function: {}", Rc::strong_count(&value));
}

//--------------------------------------------------
// Function accepting reference to Rc
//--------------------------------------------------
fn display_rc_reference(value: &Rc<i32>) {
    println!("Value: {}", value);
}

//--------------------------------------------------
// Main
//--------------------------------------------------
fn main() {

    //--------------------------------------------------
    // 01. Create Rc using Rc::new()
    //--------------------------------------------------
    println!("\n01. Rc::new()");

    let number = Rc::new(100);

    println!("Value       : {}", number);
    println!("Strong count: {}", Rc::strong_count(&number));


    //--------------------------------------------------
    // 02. Clone Rc
    //--------------------------------------------------
    println!("\n02. Rc::clone()");

    let value = Rc::new(200);
    let weak_value = Rc::downgrade(&value);
    let value2 = Rc::clone(&value);

    println!("Value 1     : {}", value);
    println!("Value 2     : {}", value2);
    println!("Strong count: {}", Rc::strong_count(&value));
    println!("Weak can upgrade: {}", weak_value.upgrade().is_some());


    //--------------------------------------------------
    // 03. Shared ownership
    //--------------------------------------------------
    println!("\n03. Shared ownership");

    let data1 = Rc::new(300);
    let data2 = Rc::clone(&data1);
    let data3 = Rc::clone(&data1);

    println!("Data 1      : {}", data1);
    println!("Data 2      : {}", data2);
    println!("Data 3      : {}", data3);
    println!("Strong count: {}", Rc::strong_count(&data1));


    //--------------------------------------------------
    // 04. Rc strong count
    //--------------------------------------------------
    println!("\n04. Rc::strong_count()");

    let item = Rc::new(400);

    println!("Initial count: {}", Rc::strong_count(&item));

    let item2 = Rc::clone(&item);

    println!("After clone  : {}", Rc::strong_count(&item));

    drop(item2);

    println!("After drop   : {}", Rc::strong_count(&item));


    //--------------------------------------------------
    // 05. Rc weak count
    //--------------------------------------------------
    println!("\n05. Rc::weak_count()");

    let value = Rc::new(500);

    let weak1 = Rc::downgrade(&value);
    let weak2 = Rc::downgrade(&value);
    let weak3 = Rc::downgrade(&value);

    println!("Strong count: {}", Rc::strong_count(&value));
    println!("Weak count  : {}", Rc::weak_count(&value));

    println!("Weak 1 valid: {}", weak1.upgrade().is_some());
    println!("Weak 2 valid: {}", weak2.upgrade().is_some());
    println!("Weak 3 valid: {}", weak3.upgrade().is_some());


    //--------------------------------------------------
    // 06. Rc with String
    //--------------------------------------------------
    println!("\n06. Rc<String>");

    let message = Rc::new(String::from("Rust programming"));

    let message2 = Rc::clone(&message);

    println!("Message 1: {}", message);
    println!("Message 2: {}", message2);


    //--------------------------------------------------
    // 07. Rc with Vec
    //--------------------------------------------------
    println!("\n07. Rc<Vec<T>>");

    let numbers = Rc::new(vec![10, 20, 30, 40, 50]);

    let numbers2 = Rc::clone(&numbers);

    println!("Numbers 1: {:?}", numbers);
    println!("Numbers 2: {:?}", numbers2);


    //--------------------------------------------------
    // 08. Rc with structure
    //--------------------------------------------------
    println!("\n08. Rc with structure");

    let employee = Rc::new(Employee {
        id: 101,
        name: String::from("Arun"),
    });

    let employee2 = Rc::clone(&employee);

    println!("Employee 1 ID  : {}", employee.id);
    println!("Employee 1 Name: {}", employee.name);
    println!("Employee 2 ID  : {}", employee2.id);
    println!("Employee 2 Name: {}", employee2.name);

    println!("Strong count: {}", Rc::strong_count(&employee));


    //--------------------------------------------------
    // 09. Rc passed to function
    //--------------------------------------------------
    println!("\n09. Rc passed to function");

    let value = Rc::new(600);

    display_number(Rc::clone(&value));

    println!("Strong count after function: {}", Rc::strong_count(&value));


    //--------------------------------------------------
    // 10. Reference to Rc
    //--------------------------------------------------
    println!("\n10. Reference to Rc");

    let value = Rc::new(700);

    display_rc_reference(&value);

    println!("Strong count: {}", Rc::strong_count(&value));


    //--------------------------------------------------
    // 11. Rc pointer equality
    //--------------------------------------------------
    println!("\n11. Rc::ptr_eq()");

    let first = Rc::new(800);
    let second = Rc::clone(&first);

    println!("Same allocation: {}", Rc::ptr_eq(&first, &second));


    //--------------------------------------------------
    // 12. Rc value equality
    //--------------------------------------------------
    println!("\n12. Rc value equality");

    let first = Rc::new(900);
    let second = Rc::new(900);

    println!("Same value: {}", first == second);
    println!("Same allocation: {}", Rc::ptr_eq(&first, &second));


    //--------------------------------------------------
    // 13. Dereferencing Rc
    //--------------------------------------------------
    println!("\n13. Dereferencing Rc");

    let value = Rc::new(1000);

    println!("Value: {}", *value);


    //--------------------------------------------------
    // 14. Rc immutability
    //--------------------------------------------------
    println!("\n14. Rc immutability");

    let value = Rc::new(1100);
    let value2 = Rc::clone(&value);

    println!("Value 1: {}", value);
    println!("Value 2: {}", value2);

    // Rc does not provide direct mutable access
    // while shared owners exist.


    //--------------------------------------------------
    // 15. Rc::get_mut()
    //--------------------------------------------------
    println!("\n15. Rc::get_mut()");

    let mut value = Rc::new(1200);

    if let Some(data) = Rc::get_mut(&mut value) {
        *data += 100;
    }

    println!("Updated value: {}", value);


    //--------------------------------------------------
    // 16. get_mut() with multiple owners
    //--------------------------------------------------
    println!("\n16. get_mut() with multiple owners");

    let mut value = Rc::new(1300);
    let value2 = Rc::clone(&value);

    match Rc::get_mut(&mut value) {
        Some(data) => println!("Mutable value: {}", data),
        None => println!("Cannot get mutable access: multiple owners exist"),
    }

    println!("Second owner: {}", value2);


    //--------------------------------------------------
    // 17. Rc::make_mut()
    //--------------------------------------------------
    println!("\n17. Rc::make_mut()");

    let mut value = Rc::new(1400);
    let value2 = Rc::clone(&value);

    *Rc::make_mut(&mut value) += 100;

    println!("Value 1: {}", value);
    println!("Value 2: {}", value2);


    //--------------------------------------------------
    // 18. Rc::try_unwrap()
    //--------------------------------------------------
    println!("\n18. Rc::try_unwrap()");

    let value = Rc::new(1500);

    match Rc::try_unwrap(value) {
        Ok(data) => println!("Ownership extracted: {}", data),
        Err(shared) => println!("Still shared: {}", shared),
    }


    //--------------------------------------------------
    // 19. Rc downgrade to Weak
    //--------------------------------------------------
    println!("\n19. Rc::downgrade()");

    let value = Rc::new(1600);

    let weak: Weak<i32> = Rc::downgrade(&value);

    println!("Strong count: {}", Rc::strong_count(&value));
    println!("Weak count  : {}", Rc::weak_count(&value));


    //--------------------------------------------------
    // 20. Weak::upgrade()
    //--------------------------------------------------
    println!("\n20. Weak::upgrade()");

    if let Some(data) = weak.upgrade() {
        println!("Upgraded value: {}", data);
    } else {
        println!("Original value no longer exists");
    }


    //--------------------------------------------------
    // 21. Weak after Rc is dropped
    //--------------------------------------------------
    println!("\n21. Weak after Rc drop");

    let weak_reference;

    {
        let value = Rc::new(1700);

        weak_reference = Rc::downgrade(&value);

        println!("Before drop: {}", weak_reference.upgrade().is_some());
    }

    println!("After drop : {}", weak_reference.upgrade().is_some());


    //--------------------------------------------------
    // 22. Multiple owners with Employee
    //--------------------------------------------------
    println!("\n22. Multiple owners with Employee");

    let employee = Rc::new(Employee {
        id: 201,
        name: String::from("Bharath"),
    });

    let manager_view = Rc::clone(&employee);
    let payroll_view = Rc::clone(&employee);

    println!("Employee: {:?}", employee);
    println!("Manager view: {:?}", manager_view);
    println!("Payroll view: {:?}", payroll_view);

    println!("Strong count: {}", Rc::strong_count(&employee));


    //--------------------------------------------------
    // 23. Rc inside Department
    //--------------------------------------------------
    println!("\n23. Rc inside Department");

    let employee1 = Rc::new(Employee {
        id: 301,
        name: String::from("Anil"),
    });

    let employee2 = Rc::new(Employee {
        id: 302,
        name: String::from("Kiran"),
    });

    let department = Department {
        name: String::from("Embedded Systems"),
        employees: vec![
            Rc::clone(&employee1),
            Rc::clone(&employee2),
        ],
    };

    println!("Department: {}", department.name);

    for employee in &department.employees {
        println!("Employee: {:?}", employee);
    }

    println!("Employee 1 owners: {}", Rc::strong_count(&employee1));


    //--------------------------------------------------
    // 24. Shared data between variables
    //--------------------------------------------------
    println!("\n24. Shared data");

    let shared_data = Rc::new(String::from("Shared configuration"));

    let application1 = Rc::clone(&shared_data);
    let application2 = Rc::clone(&shared_data);

    println!("Application 1: {}", application1);
    println!("Application 2: {}", application2);
    println!("Owner count: {}", Rc::strong_count(&shared_data));


    //--------------------------------------------------
    // 25. Rc with trait objects
    //--------------------------------------------------
    println!("\n25. Rc<dyn Trait>");

    let sensor: Rc<dyn Device> = Rc::new(Sensor {
        name: String::from("Temperature Sensor"),
    });

    let sensor2 = Rc::clone(&sensor);

    println!("Device 1: {}", sensor.device_name());
    println!("Device 2: {}", sensor2.device_name());


    //--------------------------------------------------
    // 26. Rc clone versus data clone
    //--------------------------------------------------
    println!("\n26. Rc clone versus data clone");

    let original = Rc::new(vec![10, 20, 30]);

    let shared_copy = Rc::clone(&original);
    let independent_copy = (*original).clone();

    println!("Original: {:?}", original);
    println!("Shared copy: {:?}", shared_copy);
    println!("Independent copy: {:?}", independent_copy);

    println!("Original owners: {}", Rc::strong_count(&original));


    //--------------------------------------------------
    // 27. Rc inside a block
    //--------------------------------------------------
    println!("\n27. Rc scope");

    let value = Rc::new(1800);

    {
        let value2 = Rc::clone(&value);

        println!("Inside block count: {}", Rc::strong_count(&value));
        println!("Value 2: {}", value2);
    }

    println!("Outside block count: {}", Rc::strong_count(&value));


    //--------------------------------------------------
    // 28. Explicit drop
    //--------------------------------------------------
    println!("\n28. Explicit drop");

    let value = Rc::new(1900);
    let value2 = Rc::clone(&value);

    println!("Before drop: {}", Rc::strong_count(&value));

    drop(value2);

    println!("After drop : {}", Rc::strong_count(&value));


    //--------------------------------------------------
    // 29. Rc memory size
    //--------------------------------------------------
    println!("\n29. Rc memory size");

    let value = Rc::new(2000);

    println!("Size of i32: {} bytes", size_of::<i32>());
    println!("Size of Rc<i32>: {} bytes", size_of::<Rc<i32>>());
    println!("Size of Rc variable: {} bytes", size_of_val_rc(&value));


    //--------------------------------------------------
    // 30. Rc with nested vectors
    //--------------------------------------------------
    println!("\n30. Rc with nested vectors");

    let matrix = Rc::new(vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ]);

    let matrix2 = Rc::clone(&matrix);

    println!("Matrix 1: {:?}", matrix);
    println!("Matrix 2: {:?}", matrix2);


    //--------------------------------------------------
    // 31. Rc ownership lifecycle
    //--------------------------------------------------
    println!("\n31. Rc ownership lifecycle");

    let data = Rc::new(2100);

    println!("Initial count: {}", Rc::strong_count(&data));

    let data2 = Rc::clone(&data);
    let data3 = Rc::clone(&data);

    println!("After clones: {}", Rc::strong_count(&data));

    drop(data2);

    println!("After dropping data2: {}", Rc::strong_count(&data));

    drop(data3);

    println!("After dropping data3: {}", Rc::strong_count(&data));


    //--------------------------------------------------
    // 32. Rc is single-threaded
    //--------------------------------------------------
    println!("\n32. Rc thread-safety");

    println!("Rc<T> is designed for single-threaded shared ownership.");
    println!("For multi-threaded shared ownership, use Arc<T>.");

}

//--------------------------------------------------
// Helper function for Rc memory size
//--------------------------------------------------
fn size_of_val_rc<T>(_: &Rc<T>) -> usize {
    size_of::<Rc<T>>()
}


/*
//---------------------------------------------------
Important concepts:

Rc::clone() 
- clones the reference-counted pointer, 
not the underlying data.

Rc<T> 
- enables multiple owners of the same allocation.

Rc<T> 
- is not thread-safe; use Arc<T> for 
cross-thread shared ownership.

Weak<T> 
- provides a non-owning reference and 
helps prevent reference cycles.

Rc::make_mut() 
- uses clone-on-write when shared ownership exists.

//---------------------------------------------------


//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261111_ds_Rc_01.rs -o rc_example

./rc_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

01. Rc::new()
Value       : 100
Strong count: 1

02. Rc::clone()
Value 1     : 200
Value 2     : 200
Strong count: 2
Weak can upgrade: true

03. Shared ownership
Data 1      : 300
Data 2      : 300
Data 3      : 300
Strong count: 3

04. Rc::strong_count()
Initial count: 1
After clone  : 2
After drop   : 1

05. Rc::weak_count()
Strong count: 1
Weak count  : 3
Weak 1 valid: true
Weak 2 valid: true
Weak 3 valid: true

06. Rc<String>
Message 1: Rust programming
Message 2: Rust programming

07. Rc<Vec<T>>
Numbers 1: [10, 20, 30, 40, 50]
Numbers 2: [10, 20, 30, 40, 50]

08. Rc with structure
Employee 1 ID  : 101
Employee 1 Name: Arun
Employee 2 ID  : 101
Employee 2 Name: Arun
Strong count: 2

09. Rc passed to function
Value inside function: 600
Strong count inside function: 2
Strong count after function: 1

10. Reference to Rc
Value: 700
Strong count: 1

11. Rc::ptr_eq()
Same allocation: true

12. Rc value equality
Same value: true
Same allocation: false

13. Dereferencing Rc
Value: 1000

14. Rc immutability
Value 1: 1100
Value 2: 1100

15. Rc::get_mut()
Updated value: 1300

16. get_mut() with multiple owners
Cannot get mutable access: multiple owners exist
Second owner: 1300

17. Rc::make_mut()
Value 1: 1500
Value 2: 1400

18. Rc::try_unwrap()
Ownership extracted: 1500

19. Rc::downgrade()
Strong count: 1
Weak count  : 1

20. Weak::upgrade()
Upgraded value: 1600

21. Weak after Rc drop
Before drop: true
After drop : false

22. Multiple owners with Employee
Employee: Employee { id: 201, name: "Bharath" }
Manager view: Employee { id: 201, name: "Bharath" }
Payroll view: Employee { id: 201, name: "Bharath" }
Strong count: 3

23. Rc inside Department
Department: Embedded Systems
Employee: Employee { id: 301, name: "Anil" }
Employee: Employee { id: 302, name: "Kiran" }
Employee 1 owners: 2

24. Shared data
Application 1: Shared configuration
Application 2: Shared configuration
Owner count: 3

25. Rc<dyn Trait>
Device 1: Temperature Sensor
Device 2: Temperature Sensor

26. Rc clone versus data clone
Original: [10, 20, 30]
Shared copy: [10, 20, 30]
Independent copy: [10, 20, 30]
Original owners: 2

27. Rc scope
Inside block count: 2
Value 2: 1800
Outside block count: 1

28. Explicit drop
Before drop: 2
After drop : 1

29. Rc memory size
Size of i32: 4 bytes
Size of Rc<i32>: 8 bytes
Size of Rc variable: 8 bytes

30. Rc with nested vectors
Matrix 1: [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
Matrix 2: [[1, 2, 3], [4, 5, 6], [7, 8, 9]]

31. Rc ownership lifecycle
Initial count: 1
After clones: 3
After dropping data2: 2
After dropping data3: 1

32. Rc thread-safety
Rc<T> is designed for single-threaded shared ownership.
For multi-threaded shared ownership, use Arc<T>.

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/



