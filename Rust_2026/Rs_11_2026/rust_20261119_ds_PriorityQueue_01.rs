//---------------------------------------------------
// PriorityQueue - Rust Data Structures
// Date: 20261119
//---------------------------------------------------

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::mem::size_of;

//--------------------------------------------------
// Priority Queue Node
//--------------------------------------------------
#[derive(Debug, Clone)]
struct PriorityItem<T> {
    priority: i32,
    sequence: usize,
    value: T,
}

//--------------------------------------------------
// Ordering implementation
//--------------------------------------------------
impl<T> PartialEq for PriorityItem<T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
            && self.sequence == other.sequence
    }
}

impl<T> Eq for PriorityItem<T> {}

impl<T> PartialOrd for PriorityItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for PriorityItem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority
            .cmp(&other.priority)
            // Earlier insertion wins when priorities are equal.
            .then_with(|| other.sequence.cmp(&self.sequence))
    }
}

//--------------------------------------------------
// Generic Priority Queue
//--------------------------------------------------
struct PriorityQueue<T> {
    items: BinaryHeap<PriorityItem<T>>,
    next_sequence: usize,
}

//--------------------------------------------------
// Priority Queue implementation
//--------------------------------------------------
impl<T> PriorityQueue<T> {

    //--------------------------------------------------
    // 01. Create empty priority queue
    //--------------------------------------------------
    fn new() -> Self {
        Self {
            items: BinaryHeap::new(),
            next_sequence: 0,
        }
    }

    //--------------------------------------------------
    // 02. Create priority queue with capacity
    //--------------------------------------------------
    fn with_capacity(capacity: usize) -> Self {
        Self {
            items: BinaryHeap::with_capacity(capacity),
            next_sequence: 0,
        }
    }

    //--------------------------------------------------
    // 03. Insert an element
    //--------------------------------------------------
    fn enqueue(&mut self, value: T, priority: i32) {
        let item = PriorityItem {
            priority,
            sequence: self.next_sequence,
            value,
        };

        self.next_sequence += 1;

        self.items.push(item);
    }

    //--------------------------------------------------
    // 04. Remove highest-priority element
    //--------------------------------------------------
    fn dequeue(&mut self) -> Option<(T, i32)> {
        self.items
            .pop()
            .map(|item| (item.value, item.priority))
    }

    //--------------------------------------------------
    // 05. Peek highest-priority element
    //--------------------------------------------------
    fn peek(&self) -> Option<(&T, i32)> {
        self.items
            .peek()
            .map(|item| (&item.value, item.priority))
    }

    //--------------------------------------------------
    // 06. Get queue length
    //--------------------------------------------------
    fn len(&self) -> usize {
        self.items.len()
    }

    //--------------------------------------------------
    // 07. Check whether queue is empty
    //--------------------------------------------------
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    //--------------------------------------------------
    // 08. Get capacity
    //--------------------------------------------------
    fn capacity(&self) -> usize {
        self.items.capacity()
    }

    //--------------------------------------------------
    // 09. Clear queue
    //--------------------------------------------------
    fn clear(&mut self) {
        self.items.clear();
    }

    //--------------------------------------------------
    // 10. Reserve capacity
    //--------------------------------------------------
    fn reserve(&mut self, additional: usize) {
        self.items.reserve(additional);
    }

    //--------------------------------------------------
    // 11. Shrink capacity
    //--------------------------------------------------
    fn shrink_to_fit(&mut self) {
        self.items.shrink_to_fit();
    }

    //--------------------------------------------------
    // 12. Display priority queue
    //--------------------------------------------------
    fn display(&self)
    where
        T: std::fmt::Debug + Clone,
    {
        let mut items = self.items.clone();

        while let Some(item) = items.pop() {
            println!(
                "Priority: {:>3} | Sequence: {:>3} | Value: {:?}",
                item.priority,
                item.sequence,
                item.value
            );
        }
    }

    //--------------------------------------------------
    // 13. Get highest priority
    //--------------------------------------------------
    fn highest_priority(&self) -> Option<i32> {
        self.items.peek().map(|item| item.priority)
    }

    //--------------------------------------------------
    // 14. Get all elements in priority order
    //--------------------------------------------------
    fn into_sorted_vec(self) -> Vec<(T, i32)> {
        self.items
            .into_sorted_vec()
            .into_iter()
            .map(|item| (item.value, item.priority))
            .collect()
    }
}

//--------------------------------------------------
// Task structure
//--------------------------------------------------
#[derive(Debug)]
struct Task {
    id: u32,
    name: String,
}

//--------------------------------------------------
// Main function
//--------------------------------------------------
fn main() {

    //--------------------------------------------------
    // 01. Create priority queue
    //--------------------------------------------------
    println!("\n01. Create priority queue");

    let mut queue: PriorityQueue<&str> = PriorityQueue::new();

    println!("Is empty: {}", queue.is_empty());


    //--------------------------------------------------
    // 02. Insert elements
    //--------------------------------------------------
    println!("\n02. Insert elements");

    queue.enqueue("Low priority task", 1);
    queue.enqueue("Medium priority task", 5);
    queue.enqueue("High priority task", 10);

    queue.display();


    //--------------------------------------------------
    // 03. Remove highest-priority element
    //--------------------------------------------------
    println!("\n03. Dequeue");

    println!("Removed: {:?}", queue.dequeue());


    //--------------------------------------------------
    // 04. Peek highest-priority element
    //--------------------------------------------------
    println!("\n04. Peek");

    println!("Next element: {:?}", queue.peek());


    //--------------------------------------------------
    // 05. Queue length
    //--------------------------------------------------
    println!("\n05. Length");

    println!("Length: {}", queue.len());


    //--------------------------------------------------
    // 06. Highest priority
    //--------------------------------------------------
    println!("\n06. Highest priority");

    println!("Priority: {:?}", queue.highest_priority());


    //--------------------------------------------------
    // 07. Process all elements
    //--------------------------------------------------
    println!("\n07. Process all elements");

    while let Some((value, priority)) = queue.dequeue() {
        println!("Processing: {} | Priority: {}", value, priority);
    }


    //--------------------------------------------------
    // 08. Equal priority
    //--------------------------------------------------
    println!("\n08. Equal priority");

    let mut queue: PriorityQueue<&str> = PriorityQueue::new();

    queue.enqueue("Task A", 5);
    queue.enqueue("Task B", 5);
    queue.enqueue("Task C", 5);

    while let Some((value, priority)) = queue.dequeue() {
        println!("{} | Priority: {}", value, priority);
    }


    //--------------------------------------------------
    // 09. Negative priorities
    //--------------------------------------------------
    println!("\n09. Negative priorities");

    let mut queue: PriorityQueue<&str> = PriorityQueue::new();

    queue.enqueue("Low", -10);
    queue.enqueue("Medium", 0);
    queue.enqueue("High", 10);

    queue.display();


    //--------------------------------------------------
    // 10. Priority queue with integers
    //--------------------------------------------------
    println!("\n10. Priority queue with integers");

    let mut numbers: PriorityQueue<i32> = PriorityQueue::new();

    numbers.enqueue(100, 2);
    numbers.enqueue(200, 8);
    numbers.enqueue(300, 5);

    while let Some((value, priority)) = numbers.dequeue() {
        println!("Value: {} | Priority: {}", value, priority);
    }


    //--------------------------------------------------
    // 11. Priority queue with String
    //--------------------------------------------------
    println!("\n11. Priority queue with String");

    let mut names: PriorityQueue<String> = PriorityQueue::new();

    names.enqueue(String::from("Arun"), 3);
    names.enqueue(String::from("Bharath"), 8);
    names.enqueue(String::from("Kiran"), 5);

    while let Some((name, priority)) = names.dequeue() {
        println!("Name: {} | Priority: {}", name, priority);
    }


    //--------------------------------------------------
    // 12. Priority queue with structure
    //--------------------------------------------------
    println!("\n12. Priority queue with structure");

    let mut tasks: PriorityQueue<Task> = PriorityQueue::new();

    tasks.enqueue(
        Task {
            id: 101,
            name: String::from("Compile"),
        },
        3,
    );

    tasks.enqueue(
        Task {
            id: 102,
            name: String::from("Critical security fix"),
        },
        10,
    );

    tasks.enqueue(
        Task {
            id: 103,
            name: String::from("Documentation"),
        },
        1,
    );

    while let Some((task, priority)) = tasks.dequeue() {
        println!(
            "Task ID: {} | Name: {} | Priority: {}",
            task.id,
            task.name,
            priority
        );
    }


    //--------------------------------------------------
    // 13. Capacity
    //--------------------------------------------------
    println!("\n13. Capacity");

    let mut queue: PriorityQueue<i32> = PriorityQueue::with_capacity(10);

    println!("Capacity: {}", queue.capacity());

    queue.enqueue(100, 1);
    queue.enqueue(200, 2);

    println!("Length: {}", queue.len());
    println!("Capacity: {}", queue.capacity());


    //--------------------------------------------------
    // 14. Reserve
    //--------------------------------------------------
    println!("\n14. Reserve");

    queue.reserve(20);

    println!("Capacity after reserve: {}", queue.capacity());


    //--------------------------------------------------
    // 15. Shrink to fit
    //--------------------------------------------------
    println!("\n15. Shrink to fit");

    queue.shrink_to_fit();

    println!("Capacity: {}", queue.capacity());


    //--------------------------------------------------
    // 16. Clone and display
    //--------------------------------------------------
    println!("\n16. Display without removing original elements");

    let mut queue: PriorityQueue<&str> = PriorityQueue::new();

    queue.enqueue("Job A", 2);
    queue.enqueue("Job B", 8);
    queue.enqueue("Job C", 5);

    queue.display();

    println!("Original queue length: {}", queue.len());


    //--------------------------------------------------
    // 17. Convert to sorted vector
    //--------------------------------------------------
    println!("\n17. Sorted vector");

    let mut queue: PriorityQueue<i32> = PriorityQueue::new();

    queue.enqueue(100, 3);
    queue.enqueue(200, 9);
    queue.enqueue(300, 5);

    let sorted = queue.into_sorted_vec();

    println!("Sorted: {:?}", sorted);


    //--------------------------------------------------
    // 18. Clear queue
    //--------------------------------------------------
    println!("\n18. Clear");

    let mut queue: PriorityQueue<i32> = PriorityQueue::new();

    queue.enqueue(10, 1);
    queue.enqueue(20, 2);

    queue.clear();

    println!("Is empty: {}", queue.is_empty());


    //--------------------------------------------------
    // 19. Empty dequeue
    //--------------------------------------------------
    println!("\n19. Empty dequeue");

    let mut queue: PriorityQueue<i32> = PriorityQueue::new();

    println!("Removed: {:?}", queue.dequeue());


    //--------------------------------------------------
    // 20. Peek on empty queue
    //--------------------------------------------------
    println!("\n20. Empty peek");

    println!("Peek: {:?}", queue.peek());


    //--------------------------------------------------
    // 21. CPU scheduling example
    //--------------------------------------------------
    println!("\n21. CPU scheduling");

    let mut processes: PriorityQueue<&str> = PriorityQueue::new();

    processes.enqueue("Process A", 2);
    processes.enqueue("Process B", 8);
    processes.enqueue("Process C", 5);

    while let Some((process, priority)) = processes.dequeue() {
        println!("Running: {} | Priority: {}", process, priority);
    }


    //--------------------------------------------------
    // 22. Emergency response example
    //--------------------------------------------------
    println!("\n22. Emergency response");

    let mut patients: PriorityQueue<&str> = PriorityQueue::new();

    patients.enqueue("Normal", 1);
    patients.enqueue("Critical", 10);
    patients.enqueue("Urgent", 7);

    while let Some((patient, priority)) = patients.dequeue() {
        println!("Treating: {} | Priority: {}", patient, priority);
    }


    //--------------------------------------------------
    // 23. Network packet scheduling
    //--------------------------------------------------
    println!("\n23. Network packet scheduling");

    let mut packets: PriorityQueue<&str> = PriorityQueue::new();

    packets.enqueue("Background packet", 1);
    packets.enqueue("Control packet", 10);
    packets.enqueue("Normal packet", 5);

    while let Some((packet, priority)) = packets.dequeue() {
        println!("Sending: {} | Priority: {}", packet, priority);
    }


    //--------------------------------------------------
    // 24. Memory size
    //--------------------------------------------------
    println!("\n24. Memory size");

    println!("Size of i32: {} bytes", size_of::<i32>());

    println!(
        "Size of BinaryHeap<i32>: {} bytes",
        size_of::<BinaryHeap<i32>>()
    );

    println!(
        "Size of PriorityQueue<i32>: {} bytes",
        size_of::<PriorityQueue<i32>>()
    );


    //--------------------------------------------------
    // 25. Priority ordering demonstration
    //--------------------------------------------------
    println!("\n25. Priority ordering");

    let mut queue: PriorityQueue<&str> = PriorityQueue::new();

    queue.enqueue("Priority 3", 3);
    queue.enqueue("Priority 9", 9);
    queue.enqueue("Priority 1", 1);
    queue.enqueue("Priority 7", 7);

    println!("Processing order:");

    while let Some((value, priority)) = queue.dequeue() {
        println!("{} -> {}", priority, value);
    }


    //--------------------------------------------------
    // 26. Final queue status
    //--------------------------------------------------
    println!("\n26. Final queue status");

    let mut queue: PriorityQueue<i32> = PriorityQueue::new();

    queue.enqueue(100, 4);
    queue.enqueue(200, 9);
    queue.enqueue(300, 6);

    println!("Length: {}", queue.len());
    println!("Highest priority: {:?}", queue.highest_priority());

    queue.display();
}


/*
//---------------------------------------------------
A Priority Queue is different from a normal Queue:

Normal Queue → FIFO (First In, First Out).

Priority Queue → The element with the highest priority is removed first.

If two elements have the same priority, we can use an additional sequence number to preserve insertion order.

//---------------------------------------------------
Important concepts:

Operation - Time complexity

Insert - O(log n)
Remove highest priority - O(log n)
Peek - O(1)
Length - O(1)
Build heap from collection - O(n)

//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261119_ds_PriorityQueue_01.rs -o priorityqueue_example

./priorityqueue_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

01. Create priority queue
Is empty: true

02. Insert elements
Priority:  10 | Sequence:   2 | Value: "High priority task"
Priority:   5 | Sequence:   1 | Value: "Medium priority task"
Priority:   1 | Sequence:   0 | Value: "Low priority task"

03. Dequeue
Removed: Some(("High priority task", 10))

04. Peek
Next element: Some(("Medium priority task", 5))

05. Length
Length: 2

06. Highest priority
Priority: Some(5)

07. Process all elements
Processing: Medium priority task | Priority: 5
Processing: Low priority task | Priority: 1

08. Equal priority
Task A | Priority: 5
Task B | Priority: 5
Task C | Priority: 5

09. Negative priorities
Priority:  10 | Sequence:   2 | Value: "High"
Priority:   0 | Sequence:   1 | Value: "Medium"
Priority: -10 | Sequence:   0 | Value: "Low"

10. Priority queue with integers
Value: 200 | Priority: 8
Value: 300 | Priority: 5
Value: 100 | Priority: 2

11. Priority queue with String
Name: Bharath | Priority: 8
Name: Kiran | Priority: 5
Name: Arun | Priority: 3

12. Priority queue with structure
Task ID: 102 | Name: Critical security fix | Priority: 10
Task ID: 101 | Name: Compile | Priority: 3
Task ID: 103 | Name: Documentation | Priority: 1

13. Capacity
Capacity: 10
Length: 2
Capacity: 10

14. Reserve
Capacity after reserve: 22

15. Shrink to fit
Capacity: 2

16. Display without removing original elements
Priority:   8 | Sequence:   1 | Value: "Job B"
Priority:   5 | Sequence:   2 | Value: "Job C"
Priority:   2 | Sequence:   0 | Value: "Job A"
Original queue length: 3

17. Sorted vector
Sorted: [(100, 3), (300, 5), (200, 9)]

18. Clear
Is empty: true

19. Empty dequeue
Removed: None

20. Empty peek
Peek: None

21. CPU scheduling
Running: Process B | Priority: 8
Running: Process C | Priority: 5
Running: Process A | Priority: 2

22. Emergency response
Treating: Critical | Priority: 10
Treating: Urgent | Priority: 7
Treating: Normal | Priority: 1

23. Network packet scheduling
Sending: Control packet | Priority: 10
Sending: Normal packet | Priority: 5
Sending: Background packet | Priority: 1

24. Memory size
Size of i32: 4 bytes
Size of BinaryHeap<i32>: 24 bytes
Size of PriorityQueue<i32>: 32 bytes

25. Priority ordering
Processing order:
9 -> Priority 9
7 -> Priority 7
3 -> Priority 3
1 -> Priority 1

26. Final queue status
Length: 3
Highest priority: Some(9)
Priority:   9 | Sequence:   1 | Value: 200
Priority:   6 | Sequence:   2 | Value: 300
Priority:   4 | Sequence:   0 | Value: 100

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/
