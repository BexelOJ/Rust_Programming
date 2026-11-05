//---------------------------------------------------
// Deque - Rust Data Structures
// Date: 20261118
//---------------------------------------------------

use std::collections::VecDeque;
use std::mem::size_of;

//--------------------------------------------------
// Generic Queue structure
//--------------------------------------------------
struct Queue<T> {
    items: VecDeque<T>,
}

//--------------------------------------------------
// Queue implementation
//--------------------------------------------------
impl<T> Queue<T> {

    // 01. Create an empty queue
    fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    // 02. Create queue with capacity
    fn with_capacity(capacity: usize) -> Self {
        Self {
            items: VecDeque::with_capacity(capacity),
        }
    }

    // 03. Enqueue: insert at rear
    fn enqueue(&mut self, value: T) {
        self.items.push_back(value);
    }

    // 04. Dequeue: remove from front
    fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    // 05. Peek at front
    fn front(&self) -> Option<&T> {
        self.items.front()
    }

    // 06. Peek at rear
    fn rear(&self) -> Option<&T> {
        self.items.back()
    }

    // 07. Get queue length
    fn len(&self) -> usize {
        self.items.len()
    }

    // 08. Check whether queue is empty
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    // 09. Get allocated capacity
    fn capacity(&self) -> usize {
        self.items.capacity()
    }

    // 10. Clear queue
    fn clear(&mut self) {
        self.items.clear();
    }

    // 11. Reserve additional capacity
    fn reserve(&mut self, additional: usize) {
        self.items.reserve(additional);
    }

    // 12. Shrink allocated capacity
    fn shrink_to_fit(&mut self) {
        self.items.shrink_to_fit();
    }

    // 13. Iterate over queue
    fn iter(&self) -> std::collections::vec_deque::Iter<'_, T> {
        self.items.iter()
    }

    // 14. Mutable iteration
    fn iter_mut(&mut self) -> std::collections::vec_deque::IterMut<'_, T> {
        self.items.iter_mut()
    }

    // 15. Get element by index
    fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    // 16. Get mutable element by index
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.items.get_mut(index)
    }

    // 17. Convert queue into VecDeque
    fn into_vecdeque(self) -> VecDeque<T> {
        self.items
    }

    // 18. Display queue
    fn display(&self)
    where
        T: std::fmt::Debug,
    {
        println!("{:?}", self.items);
    }

    // 19. Extend queue
    fn extend<I>(&mut self, values: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.items.extend(values);
    }

    // 20. Remove all elements and return them
    fn drain(&mut self) -> std::collections::vec_deque::Drain<'_, T> {
        self.items.drain(..)
    }
}

//--------------------------------------------------
// Main function
//--------------------------------------------------
fn main() {

    //--------------------------------------------------
    // 01. Create an empty queue
    //--------------------------------------------------
    println!("\n01. Queue::new()");

    let mut queue: Queue<i32> = Queue::new();

    println!("Is empty: {}", queue.is_empty());


    //--------------------------------------------------
    // 02. Enqueue elements
    //--------------------------------------------------
    println!("\n02. Enqueue");

    queue.enqueue(10);
    queue.enqueue(20);
    queue.enqueue(30);
    queue.enqueue(40);

    queue.display();


    //--------------------------------------------------
    // 03. Dequeue elements
    //--------------------------------------------------
    println!("\n03. Dequeue");

    if let Some(value) = queue.dequeue() {
        println!("Removed: {}", value);
    }

    queue.display();


    //--------------------------------------------------
    // 04. FIFO behaviour
    //--------------------------------------------------
    println!("\n04. FIFO behaviour");

    let mut fifo: Queue<i32> = Queue::new();

    fifo.enqueue(100);
    fifo.enqueue(200);
    fifo.enqueue(300);

    println!("First removed : {:?}", fifo.dequeue());
    println!("Second removed: {:?}", fifo.dequeue());
    println!("Third removed : {:?}", fifo.dequeue());


    //--------------------------------------------------
    // 05. Peek at front
    //--------------------------------------------------
    println!("\n05. Front element");

    let mut queue: Queue<i32> = Queue::new();

    queue.enqueue(11);
    queue.enqueue(22);
    queue.enqueue(33);

    println!("Front: {:?}", queue.front());


    //--------------------------------------------------
    // 06. Peek at rear
    //--------------------------------------------------
    println!("\n06. Rear element");

    println!("Rear: {:?}", queue.rear());


    //--------------------------------------------------
    // 07. Queue length
    //--------------------------------------------------
    println!("\n07. Queue length");

    println!("Length: {}", queue.len());


    //--------------------------------------------------
    // 08. Check empty queue
    //--------------------------------------------------
    println!("\n08. is_empty()");

    println!("Is empty: {}", queue.is_empty());


    //--------------------------------------------------
    // 09. Dequeue from empty queue
    //--------------------------------------------------
    println!("\n09. Empty queue dequeue");

    let mut empty_queue: Queue<i32> = Queue::new();

    match empty_queue.dequeue() {
        Some(value) => println!("Removed: {}", value),
        None => println!("Queue is empty"),
    }


    //--------------------------------------------------
    // 10. Queue capacity
    //--------------------------------------------------
    println!("\n10. Queue capacity");

    let mut capacity_queue: Queue<i32> = Queue::with_capacity(10);

    println!("Initial capacity: {}", capacity_queue.capacity());

    capacity_queue.enqueue(100);
    capacity_queue.enqueue(200);

    println!("Length: {}", capacity_queue.len());
    println!("Capacity: {}", capacity_queue.capacity());


    //--------------------------------------------------
    // 11. Reserve capacity
    //--------------------------------------------------
    println!("\n11. reserve()");

    let mut reserve_queue: Queue<i32> = Queue::new();

    reserve_queue.reserve(20);

    println!("Capacity: {}", reserve_queue.capacity());


    //--------------------------------------------------
    // 12. Shrink capacity
    //--------------------------------------------------
    println!("\n12. shrink_to_fit()");

    reserve_queue.enqueue(1);
    reserve_queue.enqueue(2);

    reserve_queue.shrink_to_fit();

    println!("Capacity: {}", reserve_queue.capacity());


    //--------------------------------------------------
    // 13. Iterate over queue
    //--------------------------------------------------
    println!("\n13. iter()");

    let mut queue: Queue<i32> = Queue::new();

    queue.extend([10, 20, 30, 40]);

    for value in queue.iter() {
        println!("Value: {}", value);
    }


    //--------------------------------------------------
    // 14. Mutable iteration
    //--------------------------------------------------
    println!("\n14. iter_mut()");

    for value in queue.iter_mut() {
        *value += 5;
    }

    queue.display();


    //--------------------------------------------------
    // 15. Access by index
    //--------------------------------------------------
    println!("\n15. get()");

    println!("Index 0: {:?}", queue.get(0));
    println!("Index 2: {:?}", queue.get(2));
    println!("Index 10: {:?}", queue.get(10));


    //--------------------------------------------------
    // 16. Mutable access by index
    //--------------------------------------------------
    println!("\n16. get_mut()");

    if let Some(value) = queue.get_mut(0) {
        *value = 999;
    }

    queue.display();


    //--------------------------------------------------
    // 17. Clear queue
    //--------------------------------------------------
    println!("\n17. clear()");

    queue.clear();

    println!("Length after clear: {}", queue.len());
    println!("Is empty: {}", queue.is_empty());


    //--------------------------------------------------
    // 18. Queue with String
    //--------------------------------------------------
    println!("\n18. Queue<String>");

    let mut names: Queue<String> = Queue::new();

    names.enqueue(String::from("Arun"));
    names.enqueue(String::from("Bharath"));
    names.enqueue(String::from("Kiran"));

    names.display();

    println!("Removed name: {:?}", names.dequeue());


    //--------------------------------------------------
    // 19. Queue with characters
    //--------------------------------------------------
    println!("\n19. Queue<char>");

    let mut characters: Queue<char> = Queue::new();

    characters.enqueue('A');
    characters.enqueue('B');
    characters.enqueue('C');

    characters.display();


    //--------------------------------------------------
    // 20. Queue with structure
    //--------------------------------------------------
    println!("\n20. Queue with structure");

    #[derive(Debug)]
    struct Employee {
        id: u32,
        name: String,
    }

    let mut employees: Queue<Employee> = Queue::new();

    employees.enqueue(Employee {
        id: 101,
        name: String::from("Arun"),
    });

    employees.enqueue(Employee {
        id: 102,
        name: String::from("Kiran"),
    });

    while let Some(employee) = employees.dequeue() {
        println!("Employee ID: {}", employee.id);
        println!("Employee Name: {}", employee.name);
    }


    //--------------------------------------------------
    // 21. Extend queue
    //--------------------------------------------------
    println!("\n21. extend()");

    let mut queue: Queue<i32> = Queue::new();

    queue.extend([10, 20, 30, 40, 50]);

    queue.display();


    //--------------------------------------------------
    // 22. Drain queue
    //--------------------------------------------------
    println!("\n22. drain()");

    let mut queue: Queue<i32> = Queue::new();

    queue.extend([1, 2, 3, 4, 5]);

    for value in queue.drain() {
        println!("Drained: {}", value);
    }

    println!("Queue length: {}", queue.len());


    //--------------------------------------------------
    // 23. Convert Queue into VecDeque
    //--------------------------------------------------
    println!("\n23. into_vecdeque()");

    let mut queue: Queue<i32> = Queue::new();

    queue.extend([100, 200, 300]);

    let deque = queue.into_vecdeque();

    println!("VecDeque: {:?}", deque);


    //--------------------------------------------------
    // 24. Queue ownership
    //--------------------------------------------------
    println!("\n24. Ownership");

    let mut queue: Queue<String> = Queue::new();

    queue.enqueue(String::from("Rust"));
    queue.enqueue(String::from("Linux"));

    let first = queue.dequeue();

    println!("Transferred value: {:?}", first);

    queue.display();


    //--------------------------------------------------
    // 25. Queue with VecDeque
    //--------------------------------------------------
    println!("\n25. Direct VecDeque");

    let mut numbers: VecDeque<i32> = VecDeque::new();

    numbers.push_back(10);
    numbers.push_back(20);
    numbers.push_back(30);

    println!("Queue: {:?}", numbers);

    println!("Dequeue: {:?}", numbers.pop_front());


    //--------------------------------------------------
    // 26. Queue as a printer job scheduler
    //--------------------------------------------------
    println!("\n26. Printer job scheduler");

    let mut printer_jobs: Queue<String> = Queue::new();

    printer_jobs.enqueue(String::from("Document_A.pdf"));
    printer_jobs.enqueue(String::from("Document_B.pdf"));
    printer_jobs.enqueue(String::from("Document_C.pdf"));

    while let Some(job) = printer_jobs.dequeue() {
        println!("Printing: {}", job);
    }


    //--------------------------------------------------
    // 27. Queue as a customer service system
    //--------------------------------------------------
    println!("\n27. Customer service queue");

    let mut customers: Queue<&str> = Queue::new();

    customers.enqueue("Customer 1");
    customers.enqueue("Customer 2");
    customers.enqueue("Customer 3");

    while let Some(customer) = customers.dequeue() {
        println!("Serving: {}", customer);
    }


    //--------------------------------------------------
    // 28. Queue memory size
    //--------------------------------------------------
    println!("\n28. Memory size");

    println!("Size of i32: {} bytes", size_of::<i32>());
    println!("Size of Queue<i32>: {} bytes", size_of::<Queue<i32>>());
    println!("Size of VecDeque<i32>: {} bytes", size_of::<VecDeque<i32>>());


    //--------------------------------------------------
    // 29. Queue reversal using VecDeque
    //--------------------------------------------------
    println!("\n29. Reverse queue");

    let mut queue: Queue<i32> = Queue::new();

    queue.extend([10, 20, 30, 40]);

    let mut deque = queue.into_vecdeque();

    deque.make_contiguous().reverse();

    println!("Reversed queue: {:?}", deque);


    //--------------------------------------------------
    // 30. Queue processing until empty
    //--------------------------------------------------
    println!("\n30. Process until empty");

    let mut tasks: Queue<i32> = Queue::new();

    tasks.extend([101, 102, 103, 104]);

    while !tasks.is_empty() {
        if let Some(task) = tasks.dequeue() {
            println!("Processing task: {}", task);
        }
    }

    println!("All tasks completed");

}


/*
//---------------------------------------------------
Important Queue concepts:

Operation - Behaviour

Enqueue - Insert at rear
Dequeue - Remove from front
Front - Read first element
Rear - Read last element
FIFO - First In, First Out

//---------------------------------------------------


//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261116_ds_Queue_01.rs -o queue_example

./queue_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

01. Queue::new()
Is empty: true

02. Enqueue
[10, 20, 30, 40]

03. Dequeue
Removed: 10
[20, 30, 40]

04. FIFO behaviour
First removed : Some(100)
Second removed: Some(200)
Third removed : Some(300)

05. Front element
Front: Some(11)

06. Rear element
Rear: Some(33)

07. Queue length
Length: 3

08. is_empty()
Is empty: false

09. Empty queue dequeue
Queue is empty

10. Queue capacity
Initial capacity: 10
Length: 2
Capacity: 10

11. reserve()
Capacity: 20

12. shrink_to_fit()
Capacity: 2

13. iter()
Value: 10
Value: 20
Value: 30
Value: 40

14. iter_mut()
[15, 25, 35, 45]

15. get()
Index 0: Some(15)
Index 2: Some(35)
Index 10: None

16. get_mut()
[999, 25, 35, 45]

17. clear()
Length after clear: 0
Is empty: true

18. Queue<String>
["Arun", "Bharath", "Kiran"]
Removed name: Some("Arun")

19. Queue<char>
['A', 'B', 'C']

20. Queue with structure
Employee ID: 101
Employee Name: Arun
Employee ID: 102
Employee Name: Kiran

21. extend()
[10, 20, 30, 40, 50]

22. drain()
Drained: 1
Drained: 2
Drained: 3
Drained: 4
Drained: 5
Queue length: 0

23. into_vecdeque()
VecDeque: [100, 200, 300]

24. Ownership
Transferred value: Some("Rust")
["Linux"]

25. Direct VecDeque
Queue: [10, 20, 30]
Dequeue: Some(10)

26. Printer job scheduler
Printing: Document_A.pdf
Printing: Document_B.pdf
Printing: Document_C.pdf

27. Customer service queue
Serving: Customer 1
Serving: Customer 2
Serving: Customer 3

28. Memory size
Size of i32: 4 bytes
Size of Queue<i32>: 32 bytes
Size of VecDeque<i32>: 32 bytes

29. Reverse queue
Reversed queue: [40, 30, 20, 10]

30. Process until empty
Processing task: 101
Processing task: 102
Processing task: 103
Processing task: 104
All tasks completed

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/
