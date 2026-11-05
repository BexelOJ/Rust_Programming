//---------------------------------------------------
// CircularLinkedList - Rust Data Structures
// Date: 20261109
//---------------------------------------------------

use std::mem::size_of;

//====================================================
// Circular Linked List - Node
//====================================================

#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<usize>,
}

//====================================================
// Circular Linked List
//====================================================

struct CircularLinkedList {
    nodes: Vec<Option<Node>>,
    free_slots: Vec<usize>,
    head: Option<usize>,
    tail: Option<usize>,
    len: usize,
}

//====================================================
// Implementation
//====================================================

impl CircularLinkedList {

    //------------------------------------------------
    // 01. Create an empty list
    //------------------------------------------------

    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            free_slots: Vec::new(),
            head: None,
            tail: None,
            len: 0,
        }
    }

    //------------------------------------------------
    // 02. Check whether list is empty
    //------------------------------------------------

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    //------------------------------------------------
    // 03. Return list length
    //------------------------------------------------

    fn len(&self) -> usize {
        self.len
    }

    //------------------------------------------------
    // 04. Allocate a node
    //------------------------------------------------

    fn allocate_node(&mut self, data: i32, next: Option<usize>) -> usize {

        let node = Some(Node { data, next });

        if let Some(index) = self.free_slots.pop() {
            self.nodes[index] = node;
            index
        } else {
            let index = self.nodes.len();
            self.nodes.push(node);
            index
        }
    }

    //------------------------------------------------
    // 05. Push element at front
    //------------------------------------------------

    fn push_front(&mut self, data: i32) {

        if self.is_empty() {

            let index = self.allocate_node(data, None);

            self.nodes[index].as_mut().unwrap().next = Some(index);

            self.head = Some(index);
            self.tail = Some(index);

        } else {

            let old_head = self.head.unwrap();
            let tail = self.tail.unwrap();

            let index = self.allocate_node(data, Some(old_head));

            self.nodes[tail].as_mut().unwrap().next = Some(index);

            self.head = Some(index);
        }

        self.len += 1;
    }

    //------------------------------------------------
    // 06. Push element at back
    //------------------------------------------------

    fn push_back(&mut self, data: i32) {

        if self.is_empty() {

            self.push_front(data);
            return;
        }

        let head = self.head.unwrap();

        let index = self.allocate_node(data, Some(head));

        let tail = self.tail.unwrap();

        self.nodes[tail].as_mut().unwrap().next = Some(index);

        self.tail = Some(index);

        self.len += 1;
    }

    //------------------------------------------------
    // 07. Pop element from front
    //------------------------------------------------

    fn pop_front(&mut self) -> Option<i32> {

        let head = self.head?;

        let data = self.nodes[head].as_ref()?.data;

        if self.len == 1 {

            self.head = None;
            self.tail = None;

        } else {

            let new_head = self.nodes[head].as_ref()?.next?;

            let tail = self.tail.unwrap();

            self.nodes[tail].as_mut()?.next = Some(new_head);

            self.head = Some(new_head);
        }

        self.nodes[head] = None;
        self.free_slots.push(head);

        self.len -= 1;

        Some(data)
    }

    //------------------------------------------------
    // 08. Pop element from back
    //------------------------------------------------

    fn pop_back(&mut self) -> Option<i32> {

        let tail = self.tail?;

        let data = self.nodes[tail].as_ref()?.data;

        if self.len == 1 {

            self.head = None;
            self.tail = None;

        } else {

            let mut current = self.head.unwrap();

            loop {

                let next = self.nodes[current].as_ref()?.next?;

                if next == tail {
                    break;
                }

                current = next;
            }

            let head = self.head.unwrap();

            self.nodes[current].as_mut()?.next = Some(head);

            self.tail = Some(current);
        }

        self.nodes[tail] = None;
        self.free_slots.push(tail);

        self.len -= 1;

        Some(data)
    }

    //------------------------------------------------
    // 09. Get element by index
    //------------------------------------------------

    fn get(&self, index: usize) -> Option<i32> {

        if index >= self.len {
            return None;
        }

        let mut current = self.head?;

        for _ in 0..index {
            current = self.nodes[current].as_ref()?.next?;
        }

        self.nodes[current].as_ref().map(|node| node.data)
    }

    //------------------------------------------------
    // 10. Modify element by index
    //------------------------------------------------

    fn set(&mut self, index: usize, data: i32) -> bool {

        if index >= self.len {
            return false;
        }

        let mut current = self.head.unwrap();

        for _ in 0..index {
            current = self.nodes[current]
                .as_ref()
                .unwrap()
                .next
                .unwrap();
        }

        self.nodes[current].as_mut().unwrap().data = data;

        true
    }

    //------------------------------------------------
    // 11. Insert at index
    //------------------------------------------------

    fn insert(&mut self, index: usize, data: i32) -> bool {

        if index > self.len {
            return false;
        }

        if index == 0 {
            self.push_front(data);
            return true;
        }

        if index == self.len {
            self.push_back(data);
            return true;
        }

        let mut previous = self.head.unwrap();

        for _ in 0..index - 1 {
            previous = self.nodes[previous]
                .as_ref()
                .unwrap()
                .next
                .unwrap();
        }

        let next = self.nodes[previous]
            .as_ref()
            .unwrap()
            .next
            .unwrap();

        let new_index = self.allocate_node(data, Some(next));

        self.nodes[previous].as_mut().unwrap().next = Some(new_index);

        self.len += 1;

        true
    }

    //------------------------------------------------
    // 12. Remove at index
    //------------------------------------------------

    fn remove(&mut self, index: usize) -> Option<i32> {

        if index >= self.len {
            return None;
        }

        if index == 0 {
            return self.pop_front();
        }

        if index == self.len - 1 {
            return self.pop_back();
        }

        let mut previous = self.head.unwrap();

        for _ in 0..index - 1 {
            previous = self.nodes[previous]
                .as_ref()
                .unwrap()
                .next
                .unwrap();
        }

        let target = self.nodes[previous]
            .as_ref()?
            .next?;

        let next = self.nodes[target]
            .as_ref()?
            .next?;

        let data = self.nodes[target].as_ref()?.data;

        self.nodes[previous].as_mut()?.next = Some(next);

        self.nodes[target] = None;
        self.free_slots.push(target);

        self.len -= 1;

        Some(data)
    }

    //------------------------------------------------
    // 13. Search element
    //------------------------------------------------

    fn contains(&self, data: i32) -> bool {

        let mut current = match self.head {
            Some(index) => index,
            None => return false,
        };

        for _ in 0..self.len {

            let node = self.nodes[current].as_ref().unwrap();

            if node.data == data {
                return true;
            }

            current = node.next.unwrap();
        }

        false
    }

    //------------------------------------------------
    // 14. Find element position
    //------------------------------------------------

    fn find(&self, data: i32) -> Option<usize> {

        let mut current = self.head?;

        for index in 0..self.len {

            let node = self.nodes[current].as_ref()?;

            if node.data == data {
                return Some(index);
            }

            current = node.next?;
        }

        None
    }

    //------------------------------------------------
    // 15. Reverse circular linked list
    //------------------------------------------------

    fn reverse(&mut self) {

        if self.len <= 1 {
            return;
        }

        let mut indices = Vec::with_capacity(self.len);

        let mut current = self.head.unwrap();

        for _ in 0..self.len {
            indices.push(current);

            current = self.nodes[current]
                .as_ref()
                .unwrap()
                .next
                .unwrap();
        }

        for i in 0..self.len {

            let previous = if i == 0 {
                indices[self.len - 1]
            } else {
                indices[i - 1]
            };

            self.nodes[indices[i]].as_mut().unwrap().next = Some(previous);
        }

        std::mem::swap(&mut self.head, &mut self.tail);
    }

    //------------------------------------------------
    // 16. Display circular linked list
    //------------------------------------------------

    fn display(&self) {

        if self.is_empty() {
            println!("List is empty");
            return;
        }

        let mut current = self.head.unwrap();

        print!("HEAD -> ");

        for _ in 0..self.len {

            let node = self.nodes[current].as_ref().unwrap();

            print!("{} -> ", node.data);

            current = node.next.unwrap();
        }

        println!("HEAD");
    }

    //------------------------------------------------
    // 17. Display in reverse order
    //------------------------------------------------

    fn display_reverse(&self) {

        if self.is_empty() {
            println!("List is empty");
            return;
        }

        let mut values = Vec::with_capacity(self.len);

        let mut current = self.head.unwrap();

        for _ in 0..self.len {

            let node = self.nodes[current].as_ref().unwrap();

            values.push(node.data);

            current = node.next.unwrap();
        }

        print!("HEAD -> ");

        for value in values.iter().rev() {
            print!("{} -> ", value);
        }

        println!("HEAD");
    }

    //------------------------------------------------
    // 18. Calculate sum
    //------------------------------------------------

    fn sum(&self) -> i32 {

        let mut total = 0;

        let mut current = self.head;

        for _ in 0..self.len {

            if let Some(index) = current {

                let node = self.nodes[index].as_ref().unwrap();

                total += node.data;

                current = node.next;
            }
        }

        total
    }

    //------------------------------------------------
    // 19. Find minimum
    //------------------------------------------------

    fn min(&self) -> Option<i32> {

        let mut current = self.head?;

        let mut minimum = self.nodes[current].as_ref()?.data;

        for _ in 0..self.len {

            let node = self.nodes[current].as_ref()?;

            minimum = minimum.min(node.data);

            current = node.next?;
        }

        Some(minimum)
    }

    //------------------------------------------------
    // 20. Find maximum
    //------------------------------------------------

    fn max(&self) -> Option<i32> {

        let mut current = self.head?;

        let mut maximum = self.nodes[current].as_ref()?.data;

        for _ in 0..self.len {

            let node = self.nodes[current].as_ref()?;

            maximum = maximum.max(node.data);

            current = node.next?;
        }

        Some(maximum)
    }

    //------------------------------------------------
    // 21. Clear list
    //------------------------------------------------

    fn clear(&mut self) {

        while self.pop_front().is_some() {}
    }

    //------------------------------------------------
    // 22. Clone list
    //------------------------------------------------

    fn clone_list(&self) -> Self {

        let mut new_list = Self::new();

        let mut current = self.head;

        for _ in 0..self.len {

            if let Some(index) = current {

                let node = self.nodes[index].as_ref().unwrap();

                new_list.push_back(node.data);

                current = node.next;
            }
        }

        new_list
    }
}

//====================================================
// Main
//====================================================

fn main() {

    println!("\n========================================");
    println!("       CIRCULAR LINKED LIST");
    println!("========================================");

    //------------------------------------------------
    // 23. Create list
    //------------------------------------------------

    let mut list = CircularLinkedList::new();

    println!("\n23. Create list");

    println!("Empty           : {}", list.is_empty());
    println!("Length          : {}", list.len());

    //------------------------------------------------
    // 24. Push front and back
    //------------------------------------------------

    println!("\n24. Push front and back");

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);
    list.push_front(5);

    print!("List            : ");
    list.display();

    //------------------------------------------------
    // 25. Insert
    //------------------------------------------------

    println!("\n25. Insert at index");

    list.insert(2, 15);

    print!("After insert    : ");
    list.display();

    //------------------------------------------------
    // 26. Access and modify
    //------------------------------------------------

    println!("\n26. Access and modify");

    println!("Element at 2    : {:?}", list.get(2));

    list.set(2, 17);

    print!("After set       : ");
    list.display();

    //------------------------------------------------
    // 27. Search
    //------------------------------------------------

    println!("\n27. Search");

    println!("Contains 20     : {}", list.contains(20));
    println!("Position of 30  : {:?}", list.find(30));

    //------------------------------------------------
    // 28. Remove
    //------------------------------------------------

    println!("\n28. Remove");

    println!("Removed index 2 : {:?}", list.remove(2));

    print!("Current list    : ");
    list.display();

    //------------------------------------------------
    // 29. Pop operations
    //------------------------------------------------

    println!("\n29. Pop operations");

    println!("Pop front       : {:?}", list.pop_front());
    println!("Pop back        : {:?}", list.pop_back());

    print!("Current list    : ");
    list.display();

    //------------------------------------------------
    // 30. Reverse
    //------------------------------------------------

    println!("\n30. Reverse");

    list.reverse();

    print!("Reversed        : ");
    list.display();

    print!("Reverse display : ");
    list.display_reverse();

    //------------------------------------------------
    // 31. Statistics
    //------------------------------------------------

    println!("\n31. Statistics");

    println!("Length          : {}", list.len());
    println!("Sum             : {}", list.sum());
    println!("Minimum         : {:?}", list.min());
    println!("Maximum         : {:?}", list.max());

    //------------------------------------------------
    // 32. Clone
    //------------------------------------------------

    println!("\n32. Clone");

    let cloned = list.clone_list();

    print!("Original        : ");
    list.display();

    print!("Cloned          : ");
    cloned.display();

    //------------------------------------------------
    // 33. Clear
    //------------------------------------------------

    println!("\n33. Clear");

    list.clear();

    println!("Empty           : {}", list.is_empty());
    println!("Length          : {}", list.len());

    //------------------------------------------------
    // 34. Reuse freed slots
    //------------------------------------------------

    println!("\n34. Reuse freed slots");

    list.push_back(100);
    list.push_back(200);
    list.push_back(300);

    print!("Reused list     : ");
    list.display();

    //------------------------------------------------
    // 35. Memory information
    //------------------------------------------------

    println!("\n35. Memory information");

    println!(
        "Node size       : {} bytes",
        size_of::<Node>()
    );

    println!(
        "List size       : {} bytes",
        size_of::<CircularLinkedList>()
    );

    println!(
        "Node slots      : {}",
        list.nodes.len()
    );

    println!(
        "Free slots      : {}",
        list.free_slots.len()
    );

    println!(
        "Active nodes    : {}",
        list.len()
    );

    //------------------------------------------------
    // End
    //------------------------------------------------

    println!("\n========================================");
    println!("       PROGRAM COMPLETED");
    println!("========================================");
}


/*
//---------------------------------------------------
Important Rust concepts:

Concept	Purpose

Vec<Option<Node>>	Stores node allocations
Option<usize>	Represents the next node's index
head	First node index
tail	Last node index
free_slots	Reuses deleted node slots
unwrap()	Extracts a known-present value
Some() / None	Represents optional values

//---------------------------------------------------
Time complexity:

Operation	Complexity

Push front	O(1)
Push back	O(1)
Pop front	O(1)
Pop back	O(n)
Get by index	O(n)
Insert at index	O(n)
Remove at index	O(n)
Search	O(n)
Reverse	O(n)

//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261109_ds_CircularLinkedList_01.rs -o circularlinkedlist

./circularlinkedlist

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

========================================
       CIRCULAR LINKED LIST
========================================

23. Create list
Empty           : true
Length          : 0

24. Push front and back
List            : HEAD -> 5 -> 10 -> 20 -> 30 -> HEAD

25. Insert at index
After insert    : HEAD -> 5 -> 10 -> 15 -> 20 -> 30 -> HEAD

26. Access and modify
Element at 2    : Some(15)
After set       : HEAD -> 5 -> 10 -> 17 -> 20 -> 30 -> HEAD

27. Search
Contains 20     : true
Position of 30  : Some(4)

28. Remove
Removed index 2 : Some(17)
Current list    : HEAD -> 5 -> 10 -> 20 -> 30 -> HEAD

29. Pop operations
Pop front       : Some(5)
Pop back        : Some(30)
Current list    : HEAD -> 10 -> 20 -> HEAD

30. Reverse
Reversed        : HEAD -> 20 -> 10 -> HEAD
Reverse display : HEAD -> 10 -> 20 -> HEAD

31. Statistics
Length          : 2
Sum             : 30
Minimum         : Some(10)
Maximum         : Some(20)

32. Clone
Original        : HEAD -> 20 -> 10 -> HEAD
Cloned          : HEAD -> 20 -> 10 -> HEAD

33. Clear
Empty           : true
Length          : 0

34. Reuse freed slots
Reused list     : HEAD -> 100 -> 200 -> 300 -> HEAD

35. Memory information
Node size       : 24 bytes
List size       : 88 bytes
Node slots      : 5
Free slots      : 2
Active nodes    : 3

========================================
       PROGRAM COMPLETED
========================================

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/



