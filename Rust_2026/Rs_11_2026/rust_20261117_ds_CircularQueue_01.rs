//---------------------------------------------------
// CircularQueue - Rust Data Structures
// Date: 20261117
//---------------------------------------------------

use std::mem::size_of;

//--------------------------------------------------
// Circular Queue structure
//--------------------------------------------------
struct CircularQueue<T> {
    items: Vec<Option<T>>,
    front: usize,
    rear: usize,
    len: usize,
    capacity: usize,
}

//--------------------------------------------------
// Circular Queue implementation
//--------------------------------------------------
impl<T> CircularQueue<T> {

    //--------------------------------------------------
    // 01. Create circular queue
    //--------------------------------------------------
    fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be greater than zero");

        let mut items = Vec::with_capacity(capacity);
        items.resize_with(capacity, || None);

        Self {
            items,
            front: 0,
            rear: 0,
            len: 0,
            capacity,
        }
    }

    //--------------------------------------------------
    // 02. Check whether queue is empty
    //--------------------------------------------------
    fn is_empty(&self) -> bool {
        self.len == 0
    }

    //--------------------------------------------------
    // 03. Check whether queue is full
    //--------------------------------------------------
    fn is_full(&self) -> bool {
        self.len == self.capacity
    }

    //--------------------------------------------------
    // 04. Get queue length
    //--------------------------------------------------
    fn len(&self) -> usize {
        self.len
    }

    //--------------------------------------------------
    // 05. Get queue capacity
    //--------------------------------------------------
    fn capacity(&self) -> usize {
        self.capacity
    }

    //--------------------------------------------------
    // 06. Enqueue
    //--------------------------------------------------
    fn enqueue(&mut self, value: T) -> Result<(), T> {
        if self.is_full() {
            return Err(value);
        }

        self.items[self.rear] = Some(value);

        self.rear = (self.rear + 1) % self.capacity;
        self.len += 1;

        Ok(())
    }

    //--------------------------------------------------
    // 07. Dequeue
    //--------------------------------------------------
    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let value = self.items[self.front].take();

        self.front = (self.front + 1) % self.capacity;
        self.len -= 1;

        value
    }

    //--------------------------------------------------
    // 08. Peek at front
    //--------------------------------------------------
    fn front(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.items[self.front].as_ref()
        }
    }

    //--------------------------------------------------
    // 09. Peek at rear
    //--------------------------------------------------
    fn rear(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            let index = (self.rear + self.capacity - 1) % self.capacity;
            self.items[index].as_ref()
        }
    }

    //--------------------------------------------------
    // 10. Get element by logical index
    //--------------------------------------------------
    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        let actual_index = (self.front + index) % self.capacity;

        self.items[actual_index].as_ref()
    }

    //--------------------------------------------------
    // 11. Clear queue
    //--------------------------------------------------
    fn clear(&mut self) {
        for item in &mut self.items {
            *item = None;
        }

        self.front = 0;
        self.rear = 0;
        self.len = 0;
    }

    //--------------------------------------------------
    // 12. Display queue
    //--------------------------------------------------
    fn display(&self)
    where
        T: std::fmt::Debug,
    {
        print!("Queue: [");

        for index in 0..self.len {
            let actual_index = (self.front + index) % self.capacity;

            if index > 0 {
                print!(", ");
            }

            print!("{:?}", self.items[actual_index]);
        }

        println!("]");
    }

    //--------------------------------------------------
    // 13. Display internal array
    //--------------------------------------------------
    fn display_internal(&self)
    where
        T: std::fmt::Debug,
    {
        println!("Internal array: {:?}", self.items);
        println!("Front index   : {}", self.front);
        println!("Rear index    : {}", self.rear);
        println!("Length        : {}", self.len);
        println!("Capacity      : {}", self.capacity);
    }

    //--------------------------------------------------
    // 14. Get front index
    //--------------------------------------------------
    fn front_index(&self) -> usize {
        self.front
    }

    //--------------------------------------------------
    // 15. Get rear index
    //--------------------------------------------------
    fn rear_index(&self) -> usize {
        self.rear
    }

    //--------------------------------------------------
    // 16. Iterate over queue
    //--------------------------------------------------
    fn iter(&self) -> CircularQueueIter<'_, T> {
        CircularQueueIter {
            queue: self,
            index: 0,
        }
    }
}

//--------------------------------------------------
// Circular Queue iterator
//--------------------------------------------------
struct CircularQueueIter<'a, T> {
    queue: &'a CircularQueue<T>,
    index: usize,
}

impl<'a, T> Iterator for CircularQueueIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.queue.len {
            return None;
        }

        let value = self.queue.get(self.index);

        self.index += 1;

        value
    }
}

//--------------------------------------------------
// Main function
//--------------------------------------------------
fn main() {

    //--------------------------------------------------
    // 01. Create circular queue
    //--------------------------------------------------
    println!("\n01. Create circular queue");

    let mut queue: CircularQueue<i32> = CircularQueue::new(5);

    println!("Capacity: {}", queue.capacity());
    println!("Is empty: {}", queue.is_empty());


    //--------------------------------------------------
    // 02. Enqueue elements
    //--------------------------------------------------
    println!("\n02. Enqueue");

    for value in [10, 20, 30] {
        println!("Enqueue {}: {:?}", value, queue.enqueue(value));
    }

    queue.display();


    //--------------------------------------------------
    // 03. Dequeue element
    //--------------------------------------------------
    println!("\n03. Dequeue");

    println!("Removed: {:?}", queue.dequeue());

    queue.display();


    //--------------------------------------------------
    // 04. Reuse freed position
    //--------------------------------------------------
    println!("\n04. Reuse freed position");

    println!("Enqueue 40: {:?}", queue.enqueue(40));
    println!("Enqueue 50: {:?}", queue.enqueue(50));

    queue.display_internal();


    //--------------------------------------------------
    // 05. Circular wrap-around
    //--------------------------------------------------
    println!("\n05. Circular wrap-around");

    println!("Enqueue 60: {:?}", queue.enqueue(60));

    queue.display_internal();

    println!("Enqueue 70: {:?}", queue.enqueue(70));

    queue.display();


    //--------------------------------------------------
    // 06. Queue full condition
    //--------------------------------------------------
    println!("\n06. Queue full");

    println!("Is full: {}", queue.is_full());

    match queue.enqueue(80) {
        Ok(()) => println!("Inserted successfully"),
        Err(value) => println!("Queue full, rejected: {}", value),
    }


    //--------------------------------------------------
    // 07. FIFO behaviour
    //--------------------------------------------------
    println!("\n07. FIFO behaviour");

    while let Some(value) = queue.dequeue() {
        println!("Dequeued: {}", value);
    }

    println!("Is empty: {}", queue.is_empty());


    //--------------------------------------------------
    // 08. Dequeue from empty queue
    //--------------------------------------------------
    println!("\n08. Empty dequeue");

    println!("Removed: {:?}", queue.dequeue());


    //--------------------------------------------------
    // 09. Front element
    //--------------------------------------------------
    println!("\n09. Front element");

    let mut queue: CircularQueue<i32> = CircularQueue::new(4);

    queue.enqueue(100).unwrap();
    queue.enqueue(200).unwrap();
    queue.enqueue(300).unwrap();

    println!("Front: {:?}", queue.front());


    //--------------------------------------------------
    // 10. Rear element
    //--------------------------------------------------
    println!("\n10. Rear element");

    println!("Rear: {:?}", queue.rear());


    //--------------------------------------------------
    // 11. Queue length
    //--------------------------------------------------
    println!("\n11. Queue length");

    println!("Length: {}", queue.len());


    //--------------------------------------------------
    // 12. Logical indexing
    //--------------------------------------------------
    println!("\n12. Logical indexing");

    for index in 0..queue.len() {
        println!("Index {}: {:?}", index, queue.get(index));
    }


    //--------------------------------------------------
    // 13. Invalid index
    //--------------------------------------------------
    println!("\n13. Invalid index");

    println!("Index 10: {:?}", queue.get(10));


    //--------------------------------------------------
    // 14. Internal array
    //--------------------------------------------------
    println!("\n14. Internal array");

    queue.display_internal();


    //--------------------------------------------------
    // 15. Iterate over queue
    //--------------------------------------------------
    println!("\n15. Iterator");

    for value in queue.iter() {
        println!("Value: {}", value);
    }


    //--------------------------------------------------
    // 16. Multiple wrap-arounds
    //--------------------------------------------------
    println!("\n16. Multiple wrap-arounds");

    let mut queue: CircularQueue<i32> = CircularQueue::new(3);

    for value in [1, 2, 3] {
        queue.enqueue(value).unwrap();
    }

    queue.dequeue();
    queue.dequeue();

    queue.enqueue(4).unwrap();
    queue.enqueue(5).unwrap();

    queue.display_internal();
    queue.display();


    //--------------------------------------------------
    // 17. Capacity remains fixed
    //--------------------------------------------------
    println!("\n17. Fixed capacity");

    println!("Capacity: {}", queue.capacity());

    println!("Enqueue 6: {:?}", queue.enqueue(6));
    println!("Enqueue 7: {:?}", queue.enqueue(7));


    //--------------------------------------------------
    // 18. Clear queue
    //--------------------------------------------------
    println!("\n18. Clear");

    queue.clear();

    queue.display_internal();

    println!("Is empty: {}", queue.is_empty());


    //--------------------------------------------------
    // 19. Queue with String
    //--------------------------------------------------
    println!("\n19. Queue<String>");

    let mut names: CircularQueue<String> = CircularQueue::new(3);

    names.enqueue(String::from("Arun")).unwrap();
    names.enqueue(String::from("Bharath")).unwrap();
    names.enqueue(String::from("Kiran")).unwrap();

    names.display();

    println!("Removed: {:?}", names.dequeue());

    names.display();


    //--------------------------------------------------
    // 20. Queue with characters
    //--------------------------------------------------
    println!("\n20. Queue<char>");

    let mut characters: CircularQueue<char> = CircularQueue::new(4);

    for character in ['A', 'B', 'C', 'D'] {
        characters.enqueue(character).unwrap();
    }

    characters.display();


    //--------------------------------------------------
    // 21. Queue with structures
    //--------------------------------------------------
    println!("\n21. Queue with structure");

    #[derive(Debug)]
    struct Task {
        id: u32,
        name: String,
    }

    let mut tasks: CircularQueue<Task> = CircularQueue::new(3);

    tasks.enqueue(Task {
        id: 101,
        name: String::from("Compile"),
    }).unwrap();

    tasks.enqueue(Task {
        id: 102,
        name: String::from("Link"),
    }).unwrap();

    while let Some(task) = tasks.dequeue() {
        println!("Task ID: {}", task.id);
        println!("Task Name: {}", task.name);
    }


    //--------------------------------------------------
    // 22. Queue memory size
    //--------------------------------------------------
    println!("\n22. Memory size");

    println!("Size of i32: {} bytes", size_of::<i32>());

    println!(
        "Size of CircularQueue<i32>: {} bytes",
        size_of::<CircularQueue<i32>>()
    );


    //--------------------------------------------------
    // 23. Circular queue index calculation
    //--------------------------------------------------
    println!("\n23. Index calculation");

    let capacity = 5;

    for index in 0..10 {
        let next = (index + 1) % capacity;

        println!("Current: {}, Next: {}", index % capacity, next);
    }


    //--------------------------------------------------
    // 24. Queue with capacity one
    //--------------------------------------------------
    println!("\n24. Capacity one");

    let mut single: CircularQueue<i32> = CircularQueue::new(1);

    println!("Enqueue 10: {:?}", single.enqueue(10));
    println!("Enqueue 20: {:?}", single.enqueue(20));

    println!("Dequeue: {:?}", single.dequeue());

    println!("Enqueue 30: {:?}", single.enqueue(30));

    single.display();


    //--------------------------------------------------
    // 25. Queue processing
    //--------------------------------------------------
    println!("\n25. Task processing");

    let mut job_queue: CircularQueue<i32> = CircularQueue::new(5);

    for job in [101, 102, 103] {
        job_queue.enqueue(job).unwrap();
    }

    while let Some(job) = job_queue.dequeue() {
        println!("Processing job: {}", job);
    }


    //--------------------------------------------------
    // 26. Front and rear after wrap-around
    //--------------------------------------------------
    println!("\n26. Front and rear after wrap-around");

    let mut queue: CircularQueue<i32> = CircularQueue::new(4);

    for value in [10, 20, 30, 40] {
        queue.enqueue(value).unwrap();
    }

    queue.dequeue();
    queue.dequeue();

    queue.enqueue(50).unwrap();
    queue.enqueue(60).unwrap();

    println!("Front: {:?}", queue.front());
    println!("Rear : {:?}", queue.rear());

    queue.display_internal();


    //--------------------------------------------------
    // 27. Final queue state
    //--------------------------------------------------
    println!("\n27. Final queue state");

    println!("Length   : {}", queue.len());
    println!("Capacity : {}", queue.capacity());
    println!("Front idx: {}", queue.front_index());
    println!("Rear idx : {}", queue.rear_index());

    queue.display();
}

/*
//---------------------------------------------------
Important concepts:

FIFO: 
- First In, First Out.

Fixed capacity: 
- This implementation does not automatically grow.

Wrap-around: 
- The modulo operator brings indices back to zero.

No shifting: 
- Dequeue simply takes the front element and advances the index.

Time complexity: 
- Enqueue and dequeue are both O(1).

//---------------------------------------------------


//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261117_ds_CircularQueue_01.rs -o circularqueue_example

./circularqueue_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

01. Create circular queue
Capacity: 5
Is empty: true

02. Enqueue
Enqueue 10: Ok(())
Enqueue 20: Ok(())
Enqueue 30: Ok(())
Queue: [Some(10), Some(20), Some(30)]

03. Dequeue
Removed: Some(10)
Queue: [Some(20), Some(30)]

04. Reuse freed position
Enqueue 40: Ok(())
Enqueue 50: Ok(())
Internal array: [None, Some(20), Some(30), Some(40), Some(50)]
Front index   : 1
Rear index    : 0
Length        : 4
Capacity      : 5

05. Circular wrap-around
Enqueue 60: Ok(())
Internal array: [Some(60), Some(20), Some(30), Some(40), Some(50)]
Front index   : 1
Rear index    : 1
Length        : 5
Capacity      : 5
Enqueue 70: Err(70)
Queue: [Some(20), Some(30), Some(40), Some(50), Some(60)]

06. Queue full
Is full: true
Queue full, rejected: 80

07. FIFO behaviour
Dequeued: 20
Dequeued: 30
Dequeued: 40
Dequeued: 50
Dequeued: 60
Is empty: true

08. Empty dequeue
Removed: None

09. Front element
Front: Some(100)

10. Rear element
Rear: Some(300)

11. Queue length
Length: 3

12. Logical indexing
Index 0: Some(100)
Index 1: Some(200)
Index 2: Some(300)

13. Invalid index
Index 10: None

14. Internal array
Internal array: [Some(100), Some(200), Some(300), None]
Front index   : 0
Rear index    : 3
Length        : 3
Capacity      : 4

15. Iterator
Value: 100
Value: 200
Value: 300

16. Multiple wrap-arounds
Internal array: [Some(4), Some(5), Some(3)]
Front index   : 2
Rear index    : 2
Length        : 3
Capacity      : 3
Queue: [Some(3), Some(4), Some(5)]

17. Fixed capacity
Capacity: 3
Enqueue 6: Err(6)
Enqueue 7: Err(7)

18. Clear
Internal array: [None, None, None]
Front index   : 0
Rear index    : 0
Length        : 0
Capacity      : 3
Is empty: true

19. Queue<String>
Queue: [Some("Arun"), Some("Bharath"), Some("Kiran")]
Removed: Some("Arun")
Queue: [Some("Bharath"), Some("Kiran")]

20. Queue<char>
Queue: [Some('A'), Some('B'), Some('C'), Some('D')]

21. Queue with structure
Task ID: 101
Task Name: Compile
Task ID: 102
Task Name: Link

22. Memory size
Size of i32: 4 bytes
Size of CircularQueue<i32>: 56 bytes

23. Index calculation
Current: 0, Next: 1
Current: 1, Next: 2
Current: 2, Next: 3
Current: 3, Next: 4
Current: 4, Next: 0
Current: 0, Next: 1
Current: 1, Next: 2
Current: 2, Next: 3
Current: 3, Next: 4
Current: 4, Next: 0

24. Capacity one
Enqueue 10: Ok(())
Enqueue 20: Err(20)
Dequeue: Some(10)
Enqueue 30: Ok(())
Queue: [Some(30)]

25. Task processing
Processing job: 101
Processing job: 102
Processing job: 103

26. Front and rear after wrap-around
Front: Some(30)
Rear : Some(60)
Internal array: [Some(50), Some(60), Some(30), Some(40)]
Front index   : 2
Rear index    : 2
Length        : 4
Capacity      : 4

27. Final queue state
Length   : 4
Capacity : 4
Front idx: 2
Rear idx : 2
Queue: [Some(30), Some(40), Some(50), Some(60)]

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/
