//---------------------------------------------------
// Weak - Rust Data Structures
// Date: 20261114
//---------------------------------------------------

use std::mem::size_of;
use std::rc::{Rc, Weak};
use std::sync::{Arc, Weak as ArcWeak};
use std::thread;

//---------------------------------------------------
// Structures
//---------------------------------------------------

#[derive(Debug)]
struct Employee {
    id: u32,
    name: String,
}

#[derive(Debug)]
struct Department {
    name: String,
    employees: Vec<Weak<Employee>>,
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: Option<Weak<Node>>,
    children: Vec<Rc<Node>>,
}

#[derive(Debug)]
struct Device {
    id: u32,
    name: String,
}

//---------------------------------------------------
// Main
//---------------------------------------------------

fn main() {

    //---------------------------------------------------
    // 01. Creating an Rc
    //---------------------------------------------------

    println!("\n01. Creating an Rc");

    let value = Rc::new(100);

    println!("Value: {}", value);
    println!("Strong count: {}", Rc::strong_count(&value));

    //---------------------------------------------------
    // 02. Creating a Weak reference
    //---------------------------------------------------

    println!("\n02. Creating a Weak reference");

    let value = Rc::new(200);

    let weak_value: Weak<i32> = Rc::downgrade(&value);

    println!("Weak reference can upgrade: {}", weak_value.upgrade().is_some());

    println!("Strong count: {}", Rc::strong_count(&value));
    println!("Weak count: {}", Rc::weak_count(&value));

    //---------------------------------------------------
    // 03. Upgrading Weak to Rc
    //---------------------------------------------------

    println!("\n03. Upgrading Weak to Rc");

    let value = Rc::new(300);

    let weak_value = Rc::downgrade(&value);

    match weak_value.upgrade() {
        Some(data) => println!("Upgraded value: {}", data),
        None => println!("Original allocation no longer exists"),
    }

    //---------------------------------------------------
    // 04. Weak does not own the data
    //---------------------------------------------------

    println!("\n04. Weak does not own the data");

    let value = Rc::new(400);

    let weak_value = Rc::downgrade(&value);

    println!("Before drop:");
    println!("Strong count: {}", Rc::strong_count(&value));

    drop(value);

    println!("After drop:");

    match weak_value.upgrade() {
        Some(data) => println!("Value: {}", data),
        None => println!("Data has been dropped"),
    }

    //---------------------------------------------------
    // 05. Multiple Weak references
    //---------------------------------------------------

    println!("\n05. Multiple Weak references");

    let value = Rc::new(500);

    let weak1 = Rc::downgrade(&value);
    let weak2 = Rc::downgrade(&value);
    let weak3 = Rc::downgrade(&value);

    println!("Weak count: {}", Rc::weak_count(&value));

    println!("Weak 1 valid: {}", weak1.upgrade().is_some());
    println!("Weak 2 valid: {}", weak2.upgrade().is_some());
    println!("Weak 3 valid: {}", weak3.upgrade().is_some());

    println!("Strong count: {}", Rc::strong_count(&value));
    println!("Weak count: {}", Rc::weak_count(&value));

    //---------------------------------------------------
    // 06. Weak reference count
    //---------------------------------------------------

    println!("\n06. Weak reference count");

    let value = Rc::new(600);

    println!("Initial weak count: {}", Rc::weak_count(&value));

    let weak1 = Rc::downgrade(&value);

    println!("After first downgrade: {}", Rc::weak_count(&value));

    let weak2 = Rc::downgrade(&value);

    println!("After second downgrade: {}", Rc::weak_count(&value));

    drop(weak1);

    println!("After dropping weak1: {}", Rc::weak_count(&value));

    drop(weak2);

    println!("After dropping weak2: {}", Rc::weak_count(&value));

    //---------------------------------------------------
    // 07. Weak::upgrade returning Some
    //---------------------------------------------------

    println!("\n07. Weak::upgrade returning Some");

    let value = Rc::new(700);

    let weak = Rc::downgrade(&value);

    if let Some(strong) = weak.upgrade() {
        println!("Value available: {}", strong);
    }

    //---------------------------------------------------
    // 08. Weak::upgrade returning None
    //---------------------------------------------------

    println!("\n08. Weak::upgrade returning None");

    let weak;

    {
        let value = Rc::new(800);

        weak = Rc::downgrade(&value);

        println!("Inside scope: {}", value);
    }

    match weak.upgrade() {
        Some(data) => println!("Value: {}", data),
        None => println!("Cannot upgrade: strong owner no longer exists"),
    }

    //---------------------------------------------------
    // 09. Weak with String
    //---------------------------------------------------

    println!("\n09. Weak with String");

    let message = Rc::new(String::from("Rust Weak Pointer"));

    let weak_message = Rc::downgrade(&message);

    if let Some(data) = weak_message.upgrade() {
        println!("Message: {}", data);
    }

    //---------------------------------------------------
    // 10. Weak with Vec
    //---------------------------------------------------

    println!("\n10. Weak with Vec");

    let numbers = Rc::new(vec![10, 20, 30, 40]);

    let weak_numbers = Rc::downgrade(&numbers);

    if let Some(data) = weak_numbers.upgrade() {
        println!("Numbers: {:?}", data);
    }

    //---------------------------------------------------
    // 11. Weak with structure
    //---------------------------------------------------

    println!("\n11. Weak with structure");

    let employee = Rc::new(Employee {
        id: 101,
        name: String::from("Employee A"),
    });

    let weak_employee = Rc::downgrade(&employee);

    if let Some(data) = weak_employee.upgrade() {
        println!("Employee ID: {}", data.id);
        println!("Employee Name: {}", data.name);
    }

    //---------------------------------------------------
    // 12. Weak reference with multiple owners
    //---------------------------------------------------

    println!("\n12. Weak reference with multiple owners");

    let value = Rc::new(900);

    let strong1 = Rc::clone(&value);
    let strong2 = Rc::clone(&value);

    let weak = Rc::downgrade(&value);

    println!("Strong count: {}", Rc::strong_count(&value));
    println!("Weak count: {}", Rc::weak_count(&value));

    drop(strong1);

    println!("After drop: {}", Rc::strong_count(&value));

    drop(strong2);

    println!("After dropping strong owners: {}", Rc::strong_count(&value));

    println!("Weak upgrade: {}", weak.upgrade().is_some());

    //---------------------------------------------------
    // 13. Rc::ptr_eq
    //---------------------------------------------------

    println!("\n13. Rc::ptr_eq");

    let first = Rc::new(1000);

    let second = Rc::clone(&first);

    let weak = Rc::downgrade(&first);

    if let Some(strong) = weak.upgrade() {
        println!("Same allocation: {}", Rc::ptr_eq(&first, &strong));
        println!("Second owner same allocation: {}", Rc::ptr_eq(&first, &second));
    }

    //---------------------------------------------------
    // 14. Weak reference ownership
    //---------------------------------------------------

    println!("\n14. Weak reference ownership");

    let value = Rc::new(1100);

    let weak = Rc::downgrade(&value);
    println!("Weak reference valid: {}", weak.upgrade().is_some());

    println!("Strong count: {}", Rc::strong_count(&value));
    println!("Weak count: {}", Rc::weak_count(&value));

    //---------------------------------------------------
    // 15. Weak reference with Option
    //---------------------------------------------------

    println!("\n15. Weak reference with Option");

    let value = Rc::new(1200);

    let weak: Option<Weak<i32>> = Some(Rc::downgrade(&value));

    if let Some(reference) = weak {
        match reference.upgrade() {
            Some(data) => println!("Value: {}", data),
            None => println!("Value unavailable"),
        }
    }

    //---------------------------------------------------
    // 16. Weak in a parent-child relationship
    //---------------------------------------------------

    println!("\n16. Weak in a parent-child relationship");

    let parent = Rc::new(Node {
        value: 1,
        parent: None,
        children: Vec::new(),
    });

    let child = Rc::new(Node {
        value: 2,
        parent: Some(Rc::downgrade(&parent)),
        children: Vec::new(),
    });

    if let Some(parent_reference) = child.parent.as_ref() {
        if let Some(parent_data) = parent_reference.upgrade() {
            println!("Child: {}", child.value);
            println!("Parent: {}", parent_data.value);
        }
    }

    //---------------------------------------------------
    // 17. Parent owns child, child references parent
    //---------------------------------------------------

    println!("\n17. Parent owns child, child references parent");

    let parent = Rc::new(Node {
        value: 10,
        parent: None,
        children: Vec::new(),
    });

    let child = Rc::new(Node {
        value: 20,
        parent: Some(Rc::downgrade(&parent)),
        children: Vec::new(),
    });

    // Parent owns the child.
    let mut_parent = Rc::clone(&parent);

    // This example's Node children field is immutable after construction.
    // Use interior mutability in the TreeNode example when modifying children.
    println!("Parent child count: {}", mut_parent.children.len());

    println!("Parent: {}", parent.value);
    println!("Child: {}", child.value);

    println!("Parent strong count: {}", Rc::strong_count(&parent));

    //---------------------------------------------------
    // 18. Demonstrating a reference cycle
    //---------------------------------------------------

    println!("\n18. Demonstrating a reference cycle");

    struct CycleNode {
        value: i32,
        next: std::cell::RefCell<Option<Rc<CycleNode>>>,
    }

    let first = Rc::new(CycleNode {
        value: 1,
        next: std::cell::RefCell::new(None),
    });

    let second = Rc::new(CycleNode {
        value: 2,
        next: std::cell::RefCell::new(None),
    });

    *first.next.borrow_mut() = Some(Rc::clone(&second));
    *second.next.borrow_mut() = Some(Rc::clone(&first));

    println!("First value: {}", first.value);
    println!("Second value: {}", second.value);

    println!("First strong count: {}", Rc::strong_count(&first));
    println!("Second strong count: {}", Rc::strong_count(&second));

    // Break the cycle manually to avoid leaking memory.
    first.next.borrow_mut().take();
    second.next.borrow_mut().take();

    //---------------------------------------------------
    // 19. Preventing reference cycles using Weak
    //---------------------------------------------------

    println!("\n19. Preventing reference cycles using Weak");

    struct TreeNode {
        value: i32,
        parent: Option<Weak<TreeNode>>,
        children: std::cell::RefCell<Vec<Rc<TreeNode>>>,
    }

    let root = Rc::new(TreeNode {
        value: 100,
        parent: None,
        children: std::cell::RefCell::new(Vec::new()),
    });

    let child = Rc::new(TreeNode {
        value: 200,
        parent: Some(Rc::downgrade(&root)),
        children: std::cell::RefCell::new(Vec::new()),
    });

    root.children.borrow_mut().push(Rc::clone(&child));

    println!("Root: {}", root.value);
    println!("Child: {}", child.value);

    if let Some(parent) = child.parent.as_ref().and_then(Weak::upgrade) {
        println!("Child's parent: {}", parent.value);
    }

    println!("Root strong count: {}", Rc::strong_count(&root));
    println!("Child strong count: {}", Rc::strong_count(&child));

    //---------------------------------------------------
    // 20. Weak with Department and Employee
    //---------------------------------------------------

    println!("\n20. Weak with Department and Employee");

    let employee = Rc::new(Employee {
        id: 201,
        name: String::from("Employee B"),
    });

    let department = Department {
        name: String::from("Engineering"),
        employees: vec![Rc::downgrade(&employee)],
    };

    println!("Department: {}", department.name);

    for weak_employee in &department.employees {
        if let Some(employee) = weak_employee.upgrade() {
            println!("Employee: {} - {}", employee.id, employee.name);
        }
    }

    //---------------------------------------------------
    // 21. Employee disappears after owner is dropped
    //---------------------------------------------------

    println!("\n21. Employee disappears after owner is dropped");

    let weak_employee;

    {
        let employee = Rc::new(Employee {
            id: 301,
            name: String::from("Temporary Employee"),
        });

        weak_employee = Rc::downgrade(&employee);

        println!("Employee exists: {}", weak_employee.upgrade().is_some());
    }

    println!("Employee exists: {}", weak_employee.upgrade().is_some());

    //---------------------------------------------------
    // 22. Weak with Arc
    //---------------------------------------------------

    println!("\n22. Weak with Arc");

    let value = Arc::new(1300);

    let weak: ArcWeak<i32> = Arc::downgrade(&value);

    println!("Strong count: {}", Arc::strong_count(&value));
    println!("Weak count: {}", Arc::weak_count(&value));

    if let Some(data) = weak.upgrade() {
        println!("Upgraded value: {}", data);
    }

    //---------------------------------------------------
    // 23. Arc Weak across threads
    //---------------------------------------------------

    println!("\n23. Arc Weak across threads");

    let shared = Arc::new(Device {
        id: 401,
        name: String::from("Network Sensor"),
    });

    let weak_device = Arc::downgrade(&shared);

    let handle = thread::spawn(move || {
        if let Some(device) = weak_device.upgrade() {
            println!("Thread device ID: {}", device.id);
            println!("Thread device name: {}", device.name);
        } else {
            println!("Device no longer exists");
        }
    });

    handle.join().unwrap();

    //---------------------------------------------------
    // 24. Arc Weak after strong owner is dropped
    //---------------------------------------------------

    println!("\n24. Arc Weak after strong owner is dropped");

    let weak;

    {
        let value = Arc::new(1400);

        weak = Arc::downgrade(&value);

        println!("Inside scope: {}", weak.upgrade().is_some());
    }

    println!("Outside scope: {}", weak.upgrade().is_some());

    //---------------------------------------------------
    // 25. Weak with multiple Arc owners
    //---------------------------------------------------

    println!("\n25. Weak with multiple Arc owners");

    let value = Arc::new(1500);

    let strong1 = Arc::clone(&value);
    let strong2 = Arc::clone(&value);

    let weak = Arc::downgrade(&value);

    println!("Strong count: {}", Arc::strong_count(&value));

    drop(strong1);

    println!("After first drop: {}", Arc::strong_count(&value));

    drop(strong2);

    println!("After second drop: {}", Arc::strong_count(&value));

    println!("Weak upgrade: {}", weak.upgrade().is_some());

    //---------------------------------------------------
    // 26. Weak reference with memory size
    //---------------------------------------------------

    println!("\n26. Weak reference with memory size");

    println!("Rc<i32> size: {} bytes", size_of::<Rc<i32>>());

    println!("Weak<i32> size: {} bytes", size_of::<Weak<i32>>());

    println!("Arc<i32> size: {} bytes", size_of::<Arc<i32>>());

    println!("ArcWeak<i32> size: {} bytes", size_of::<ArcWeak<i32>>());

    //---------------------------------------------------
    // 27. Weak reference lifecycle
    //---------------------------------------------------

    println!("\n27. Weak reference lifecycle");

    let value = Rc::new(1600);

    let weak = Rc::downgrade(&value);

    println!("Strong count: {}", Rc::strong_count(&value));
    println!("Weak count: {}", Rc::weak_count(&value));

    drop(value);

    println!("After strong owner drop:");

    println!("Upgrade successful: {}", weak.upgrade().is_some());

    //---------------------------------------------------
    // 28. Weak does not prevent destruction
    //---------------------------------------------------

    println!("\n28. Weak does not prevent destruction");

    let weak;

    {
        let value = Rc::new(String::from("Temporary data"));

        weak = Rc::downgrade(&value);

        println!("Data: {}", value);
    }

    match weak.upgrade() {
        Some(data) => println!("Data: {}", data),
        None => println!("Temporary data has been destroyed"),
    }

    //---------------------------------------------------
    // 29. Weak reference with collection
    //---------------------------------------------------

    println!("\n29. Weak reference with collection");

    let values: Vec<Rc<i32>> = vec![
        Rc::new(10),
        Rc::new(20),
        Rc::new(30),
    ];

    let weak_values: Vec<Weak<i32>> = values
        .iter()
        .map(Rc::downgrade)
        .collect();

    for weak in &weak_values {
        match weak.upgrade() {
            Some(value) => println!("Value: {}", value),
            None => println!("Value unavailable"),
        }
    }

    //---------------------------------------------------
    // 30. Dropping collection owners
    //---------------------------------------------------

    println!("\n30. Dropping collection owners");

    let values: Vec<Rc<i32>> = vec![
        Rc::new(100),
        Rc::new(200),
        Rc::new(300),
    ];

    let weak_values: Vec<Weak<i32>> = values
        .iter()
        .map(Rc::downgrade)
        .collect();

    drop(values);

    for weak in &weak_values {
        println!("Available: {}", weak.upgrade().is_some());
    }

    //---------------------------------------------------
    // 31. Weak and strong ownership comparison
    //---------------------------------------------------

    println!("\n31. Weak and strong ownership comparison");

    let value = Rc::new(1700);

    let strong = Rc::clone(&value);

    let weak = Rc::downgrade(&value);

    println!("Strong count: {}", Rc::strong_count(&value));
    println!("Weak count: {}", Rc::weak_count(&value));

    drop(strong);

    println!("After dropping strong clone:");
    println!("Strong count: {}", Rc::strong_count(&value));

    println!("Weak can upgrade: {}", weak.upgrade().is_some());

    //---------------------------------------------------
    // 32. End of program
    //---------------------------------------------------

    println!("\n32. End of Weak demonstration");
}

/*
//---------------------------------------------------
Weak<T> is a non-owning smart pointer. 
It allows you to refer to data managed by 
Rc<T> or Arc<T> 
without increasing its strong reference count.

Why is this important?

Rc<T> and Arc<T> 
- maintain ownership.

Weak<T> 
- does not maintain ownership.

upgrade() 
- attempts to obtain a strong reference.

downgrade() 
- creates a weak reference.

Weak references help prevent reference cycles 
and memory leaks.

//---------------------------------------------------
Important concepts:

Method - Purpose

Rc::downgrade()	
- Creates a weak reference from Rc

Arc::downgrade()	
- Creates a weak reference from Arc

Weak::upgrade()
- Attempts to obtain a strong reference

Rc::strong_count()	
- Counts strong Rc owners

Rc::weak_count()	
- Counts explicit weak Rc references

Arc::strong_count()	
- Counts strong Arc owners

Arc::weak_count()	
- Counts explicit weak Arc references


//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261114_ds_Weak_01.rs -o weak_example

./weak_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

01. Creating an Rc
Value: 100
Strong count: 1

02. Creating a Weak reference
Weak reference can upgrade: true
Strong count: 1
Weak count: 1

03. Upgrading Weak to Rc
Upgraded value: 300

04. Weak does not own the data
Before drop:
Strong count: 1
After drop:
Data has been dropped

05. Multiple Weak references
Weak count: 3
Weak 1 valid: true
Weak 2 valid: true
Weak 3 valid: true
Strong count: 1
Weak count: 3

06. Weak reference count
Initial weak count: 0
After first downgrade: 1
After second downgrade: 2
After dropping weak1: 1
After dropping weak2: 0

07. Weak::upgrade returning Some
Value available: 700

08. Weak::upgrade returning None
Inside scope: 800
Cannot upgrade: strong owner no longer exists

09. Weak with String
Message: Rust Weak Pointer

10. Weak with Vec
Numbers: [10, 20, 30, 40]

11. Weak with structure
Employee ID: 101
Employee Name: Employee A

12. Weak reference with multiple owners
Strong count: 3
Weak count: 1
After drop: 2
After dropping strong owners: 1
Weak upgrade: true

13. Rc::ptr_eq
Same allocation: true
Second owner same allocation: true

14. Weak reference ownership
Weak reference valid: true
Strong count: 1
Weak count: 1

15. Weak reference with Option
Value: 1200

16. Weak in a parent-child relationship
Child: 2
Parent: 1

17. Parent owns child, child references parent
Parent child count: 0
Parent: 10
Child: 20
Parent strong count: 2

18. Demonstrating a reference cycle
First value: 1
Second value: 2
First strong count: 2
Second strong count: 2

19. Preventing reference cycles using Weak
Root: 100
Child: 200
Child's parent: 100
Root strong count: 1
Child strong count: 2

20. Weak with Department and Employee
Department: Engineering
Employee: 201 - Employee B

21. Employee disappears after owner is dropped
Employee exists: true
Employee exists: false

22. Weak with Arc
Strong count: 1
Weak count: 1
Upgraded value: 1300

23. Arc Weak across threads
Thread device ID: 401
Thread device name: Network Sensor

24. Arc Weak after strong owner is dropped
Inside scope: true
Outside scope: false

25. Weak with multiple Arc owners
Strong count: 3
After first drop: 2
After second drop: 1
Weak upgrade: true

26. Weak reference with memory size
Rc<i32> size: 8 bytes
Weak<i32> size: 8 bytes
Arc<i32> size: 8 bytes
ArcWeak<i32> size: 8 bytes

27. Weak reference lifecycle
Strong count: 1
Weak count: 1
After strong owner drop:
Upgrade successful: false

28. Weak does not prevent destruction
Data: Temporary data
Temporary data has been destroyed

29. Weak reference with collection
Value: 10
Value: 20
Value: 30

30. Dropping collection owners
Available: false
Available: false
Available: false

31. Weak and strong ownership comparison
Strong count: 2
Weak count: 1
After dropping strong clone:
Strong count: 1
Weak can upgrade: true

32. End of Weak demonstration

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/