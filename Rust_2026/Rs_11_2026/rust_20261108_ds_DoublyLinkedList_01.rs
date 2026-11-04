//---------------------------------------------------
// DoublyLinkedList - Rust Data Structures
// Date: 20261108
//---------------------------------------------------

use std::mem;
use std::ptr::NonNull;

//==============================================================
// 01. NODE STRUCTURE
//==============================================================

struct Node
{
    data: i32,
    prev: Option<NonNull<Node>>,
    next: Option<Box<Node>>,
}

//==============================================================
// 02. DOUBLY LINKED LIST STRUCTURE
//==============================================================

pub struct DoublyLinkedList
{
    head: Option<Box<Node>>,
    tail: Option<NonNull<Node>>,
    len: usize,
}

//==============================================================
// 03. IMPLEMENT DOUBLY LINKED LIST
//==============================================================

impl DoublyLinkedList
{
    //----------------------------------------------------------
    // 03.1 CREATE EMPTY LIST
    //----------------------------------------------------------

    fn new() -> Self
    {
        Self
        {
            head: None,
            tail: None,
            len: 0,
        }
    }

    //----------------------------------------------------------
    // 03.2 CHECK EMPTY
    //----------------------------------------------------------

    fn is_empty(&self) -> bool
    {
        self.head.is_none()
    }

    //----------------------------------------------------------
    // 03.3 GET LENGTH
    //----------------------------------------------------------

    fn len(&self) -> usize
    {
        self.len
    }

    //----------------------------------------------------------
    // 03.4 INSERT AT FRONT
    //----------------------------------------------------------

    fn push_front(&mut self, value: i32)
    {
        let mut new_node = Box::new(Node
        {
            data: value,
            prev: None,
            next: self.head.take(),
        });

        let new_ptr = NonNull::from(&mut *new_node);

        match new_node.next.as_mut()
        {
            Some(next_node) =>
            {
                next_node.prev = Some(new_ptr);
            }

            None =>
            {
                self.tail = Some(new_ptr);
            }
        }

        self.head = Some(new_node);

        self.len += 1;
    }

    //----------------------------------------------------------
    // 03.5 INSERT AT BACK
    //----------------------------------------------------------

    fn push_back(&mut self, value: i32)
    {
        let mut new_node = Box::new(Node
        {
            data: value,
            prev: self.tail,
            next: None,
        });

        let new_ptr = NonNull::from(&mut *new_node);

        match self.tail
        {
            Some(mut tail_ptr) =>
            {
                unsafe
                {
                    tail_ptr.as_mut().next = Some(new_node);
                }
            }

            None =>
            {
                self.head = Some(new_node);
            }
        }

        self.tail = Some(new_ptr);

        self.len += 1;
    }

    //----------------------------------------------------------
    // 03.6 REMOVE FROM FRONT
    //----------------------------------------------------------

    fn pop_front(&mut self) -> Option<i32>
    {
        let mut removed = self.head.take()?;

        self.head = removed.next.take();

        match self.head.as_mut()
        {
            Some(new_head) =>
            {
                new_head.prev = None;
            }

            None =>
            {
                self.tail = None;
            }
        }

        self.len -= 1;

        Some(removed.data)
    }

    //----------------------------------------------------------
    // 03.7 REMOVE FROM BACK
    //----------------------------------------------------------

    fn pop_back(&mut self) -> Option<i32>
    {
        let mut tail_ptr = self.tail?;

        let value;

        unsafe
        {
            let tail = tail_ptr.as_mut();

            value = tail.data;

            match tail.prev
            {
                Some(mut prev_ptr) =>
                {
                    prev_ptr.as_mut().next = None;
                    self.tail = Some(prev_ptr);
                }

                None =>
                {
                    self.head = None;
                    self.tail = None;
                }
            }
        }

        self.len -= 1;

        Some(value)
    }

    //----------------------------------------------------------
    // 03.8 GET ELEMENT BY INDEX
    //----------------------------------------------------------

    fn get(&self, index: usize) -> Option<i32>
    {
        if index >= self.len
        {
            return None;
        }

        if index <= self.len / 2
        {
            let mut current = self.head.as_deref();

            for _ in 0..index
            {
                current = current?.next.as_deref();
            }

            current.map(|node| node.data)
        }
        else
        {
            let mut current = self.tail;

            for _ in index + 1..self.len
            {
                current = unsafe
                {
                    current?.as_ref().prev
                };
            }

            unsafe
            {
                current.map(|node| node.as_ref().data)
            }
        }
    }

    //----------------------------------------------------------
    // 03.9 SEARCH ELEMENT
    //----------------------------------------------------------

    fn contains(&self, value: i32) -> bool
    {
        let mut current = self.head.as_deref();

        while let Some(node) = current
        {
            if node.data == value
            {
                return true;
            }

            current = node.next.as_deref();
        }

        false
    }

    //----------------------------------------------------------
    // 03.10 FIND ELEMENT INDEX
    //----------------------------------------------------------

    fn find(&self, value: i32) -> Option<usize>
    {
        let mut current = self.head.as_deref();
        let mut index = 0;

        while let Some(node) = current
        {
            if node.data == value
            {
                return Some(index);
            }

            current = node.next.as_deref();
            index += 1;
        }

        None
    }

    //----------------------------------------------------------
    // 03.11 INSERT AT INDEX
    //----------------------------------------------------------

    fn insert(&mut self, index: usize, value: i32) -> bool
    {
        if index > self.len
        {
            return false;
        }

        if index == 0
        {
            self.push_front(value);
            return true;
        }

        if index == self.len
        {
            self.push_back(value);
            return true;
        }

        let mut current = self.head.as_mut().unwrap();

        for _ in 0..index - 1
        {
            current = current.next.as_mut().unwrap();
        }

        let mut new_node = Box::new(Node
        {
            data: value,
            prev: Some(NonNull::from(&mut **current)),
            next: current.next.take(),
        });

        let new_ptr = NonNull::from(&mut *new_node);

        if let Some(next_node) = new_node.next.as_mut()
        {
            next_node.prev = Some(new_ptr);
        }

        current.next = Some(new_node);

        self.len += 1;

        true
    }

    //----------------------------------------------------------
    // 03.12 REMOVE AT INDEX
    //----------------------------------------------------------

    fn remove(&mut self, index: usize) -> Option<i32>
    {
        if index >= self.len
        {
            return None;
        }

        if index == 0
        {
            return self.pop_front();
        }

        if index == self.len - 1
        {
            return self.pop_back();
        }

        let mut current = self.head.as_mut()?;

        for _ in 0..index - 1
        {
            current = current.next.as_mut()?;
        }

        let mut removed = current.next.take()?;

        current.next = removed.next.take();

        let previous_ptr = NonNull::from(&mut **current);

        if let Some(next_node) = current.next.as_mut()
        {
            next_node.prev = Some(previous_ptr);
        }

        self.len -= 1;

        Some(removed.data)
    }

    //----------------------------------------------------------
    // 03.13 DISPLAY FORWARD
    //----------------------------------------------------------

    fn display_forward(&self)
    {
        let mut current = self.head.as_deref();

        print!("NULL <-> ");

        while let Some(node) = current
        {
            print!("{} <-> ", node.data);

            current = node.next.as_deref();
        }

        println!("NULL");
    }

    //----------------------------------------------------------
    // 03.14 DISPLAY BACKWARD
    //----------------------------------------------------------

    fn display_backward(&self)
    {
        let mut current = self.tail;

        print!("NULL <-> ");

        while let Some(node_ptr) = current
        {
            unsafe
            {
                let node = node_ptr.as_ref();

                print!("{} <-> ", node.data);

                current = node.prev;
            }
        }

        println!("NULL");
    }

    //----------------------------------------------------------
    // 03.15 MODIFY ELEMENT
    //----------------------------------------------------------
    
    fn set(&mut self, index: usize, value: i32) -> bool {
        if index >= self.len {
            return false;
        }

        let mut current = self.head.as_deref_mut();

        for _ in 0..index {
            match current {
                Some(node) => {
                    current = node.next.as_deref_mut();
                }
                None => return false,
            }
        }

        match current {
            Some(node) => {
                node.data = value;
                true
            }
            None => false,
        }
    }


    //----------------------------------------------------------
    // 03.16 REVERSE LIST
    //----------------------------------------------------------


    fn reverse(&mut self) {
        if self.len <= 1 {
            return;
        }

        // The original head becomes the new tail.
        let old_head_ptr = self.head.as_mut().map(
            |node| NonNull::from(&mut **node)
        );

        let mut current = self.head.take();
        let mut new_head: Option<Box<Node>> = None;

        while let Some(mut node) = current {
            // Preserve the original forward link.
            let next = node.next.take();

            // In the reversed list, the old next node
            // becomes the previous node.
            node.prev = next.as_ref().map(
                |next_node| NonNull::from(&**next_node)
            );

            // Update the previous reversed node's
            // backward pointer to point to this node.
            let node_ptr = NonNull::from(&mut *node);

            if let Some(previous_node) = new_head.as_mut() {
                previous_node.prev = Some(node_ptr);
         }

            // Reverse the ownership link.
            node.next = new_head;
                
            // Move to the next node in the original list.
         current = next;
            
            // This node becomes the new head.
            new_head = Some(node);
        }

        self.head = new_head;
        self.tail = old_head_ptr;
    }


    //----------------------------------------------------------
    // 03.17 CLEAR LIST
    //----------------------------------------------------------

    fn clear(&mut self)
    {
        self.head = None;
        self.tail = None;
        self.len = 0;
    }

    //----------------------------------------------------------
    // 03.18 SUM
    //----------------------------------------------------------

    fn sum(&self) -> i32
    {
        let mut total = 0;
        let mut current = self.head.as_deref();

        while let Some(node) = current
        {
            total += node.data;
            current = node.next.as_deref();
        }

        total
    }

    //----------------------------------------------------------
    // 03.19 CLONE LIST
    //----------------------------------------------------------

    fn clone_list(&self) -> Self
    {
        let mut result = Self::new();

        let mut current = self.head.as_deref();

        while let Some(node) = current
        {
            result.push_back(node.data);
            current = node.next.as_deref();
        }

        result
    }
}

//==============================================================
// 04. DROP IMPLEMENTATION
//==============================================================

impl Drop for DoublyLinkedList
{
    fn drop(&mut self)
    {
        self.clear();
    }
}

//==============================================================
// 05. CREATE AND DISPLAY
//==============================================================

fn create_list()
{
    let mut list = DoublyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    list.display_forward();
}

//==============================================================
// 06. INSERT AT FRONT
//==============================================================

fn insert_front()
{
    let mut list = DoublyLinkedList::new();

    list.push_front(30);
    list.push_front(20);
    list.push_front(10);

    list.display_forward();
}

//==============================================================
// 07. INSERT AT BACK
//==============================================================

fn insert_back()
{
    let mut list = DoublyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    list.display_forward();
}

//==============================================================
// 08. REMOVE FROM FRONT
//==============================================================

fn remove_front()
{
    let mut list = DoublyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    println!("Removed:      {:?}", list.pop_front());

    list.display_forward();
}

//==============================================================
// 09. REMOVE FROM BACK
//==============================================================

fn remove_back()
{
    let mut list = DoublyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    println!("Removed:      {:?}", list.pop_back());

    list.display_forward();
}

//==============================================================
// 10. INSERT AT INDEX
//==============================================================

fn insert_at_index()
{
    let mut list = DoublyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(40);

    list.insert(2, 30);

    list.display_forward();
}

//==============================================================
// 11. REMOVE AT INDEX
//==============================================================

fn remove_at_index()
{
    let mut list = DoublyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);
    list.push_back(40);

    println!("Removed:      {:?}", list.remove(2));

    list.display_forward();
}

//==============================================================
// 12. ACCESS BY INDEX
//==============================================================

fn access_element()
{
    let mut list = DoublyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    println!("Element:      {:?}", list.get(1));
}

//==============================================================
// 13. MODIFY ELEMENT
//==============================================================

fn modify_element()
{
    let mut list = DoublyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    list.set(1, 200);

    list.display_forward();
}

//==============================================================
// 14. SEARCH ELEMENT
//==============================================================

fn search_element()
{
    let mut list = DoublyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    println!("Contains 20:  {}", list.contains(20));
    println!("Contains 90:  {}", list.contains(90));
}

//==============================================================
// 15. FIND ELEMENT INDEX
//==============================================================

fn find_element()
{
    let mut list = DoublyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    println!("Index:        {:?}", list.find(30));
}

//==============================================================
// 16. DISPLAY BACKWARD
//==============================================================

fn backward_traversal()
{
    let mut list = DoublyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    list.display_backward();
}

//==============================================================
// 17. REVERSE LIST
//==============================================================

fn reverse_list()
{
    let mut list = DoublyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    list.reverse();

    list.display_forward();
}

//==============================================================
// 18. CHECK LENGTH
//==============================================================

fn check_length()
{
    let mut list = DoublyLinkedList::new();

    list.push_back(10);
    list.push_back(20);

    println!("Length:       {}", list.len());
}

//==============================================================
// 19. CHECK EMPTY
//==============================================================

fn check_empty()
{
    let list = DoublyLinkedList::new();

    println!("Is empty:     {}", list.is_empty());
}

//==============================================================
// 20. SUM
//==============================================================

fn calculate_sum()
{
    let mut list = DoublyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    println!("Sum:          {}", list.sum());
}

//==============================================================
// 21. CLONE LIST
//==============================================================

fn clone_list()
{
    let mut original = DoublyLinkedList::new();

    original.push_back(10);
    original.push_back(20);
    original.push_back(30);

    let copied = original.clone_list();

    println!("Original:");
    original.display_forward();

    println!("Copied:");
    copied.display_forward();
}

//==============================================================
// 22. CLEAR LIST
//==============================================================

fn clear_list()
{
    let mut list = DoublyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    list.clear();

    println!("Is empty:     {}", list.is_empty());
    println!("Length:       {}", list.len());
}

//==============================================================
// 23. MEMORY SIZE
//==============================================================

fn memory_size()
{
    println!(
        "Node size:    {} bytes",
        mem::size_of::<Node>()
    );

    println!(
        "List size:    {} bytes",
        mem::size_of::<DoublyLinkedList>()
    );

    println!(
        "Box size:     {} bytes",
        mem::size_of::<Option<Box<Node>>>()
    );

    println!(
        "Pointer size: {} bytes",
        mem::size_of::<Option<NonNull<Node>>>()
    );
}

//==============================================================
// 24. MAIN FUNCTION
//==============================================================

fn main()
{
    println!("\n01. CREATE");
    create_list();

    println!("\n02. INSERT FRONT");
    insert_front();

    println!("\n03. INSERT BACK");
    insert_back();

    println!("\n04. REMOVE FRONT");
    remove_front();

    println!("\n05. REMOVE BACK");
    remove_back();

    println!("\n06. INSERT AT INDEX");
    insert_at_index();

    println!("\n07. REMOVE AT INDEX");
    remove_at_index();

    println!("\n08. ACCESS ELEMENT");
    access_element();

    println!("\n09. MODIFY ELEMENT");
    modify_element();

    println!("\n10. SEARCH");
    search_element();

    println!("\n11. FIND INDEX");
    find_element();

    println!("\n12. BACKWARD TRAVERSAL");
    backward_traversal();

    println!("\n13. REVERSE");
    reverse_list();

    println!("\n14. LENGTH");
    check_length();

    println!("\n15. EMPTY");
    check_empty();

    println!("\n16. SUM");
    calculate_sum();

    println!("\n17. CLONE");
    clone_list();

    println!("\n18. CLEAR");
    clear_list();

    println!("\n19. MEMORY");
    memory_size();
}


/*
//---------------------------------------------------
Singly vs Doubly Linked List:

Feature	Singly	Doubly
Links per node	1	2
Forward traversal	Yes	Yes
Backward traversal	No	Yes
Insert at front	O(1)	O(1)
Insert at back with tail	O(1)	O(1)
Remove from back with tail	O(n)	O(1)
Memory per node	Lower	Higher

//---------------------------------------------------
Where are doubly linked lists useful?
- Browser back/forward navigation.
- Undo/redo operations.
- LRU cache implementations.
- Operating-system list management.
- Bidirectional iterators.

//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261108_ds_DoublyLinkedList_01.rs -o doublylinkedlist

./doublylinkedlist

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

01. CREATE
NULL <-> 10 <-> 20 <-> 30 <-> NULL

02. INSERT FRONT
NULL <-> 10 <-> 20 <-> 30 <-> NULL

03. INSERT BACK
NULL <-> 10 <-> 20 <-> 30 <-> NULL

04. REMOVE FRONT
Removed:      Some(10)
NULL <-> 20 <-> 30 <-> NULL

05. REMOVE BACK
Removed:      Some(30)
NULL <-> 10 <-> 20 <-> NULL

06. INSERT AT INDEX
NULL <-> 10 <-> 20 <-> 30 <-> 40 <-> NULL

07. REMOVE AT INDEX
Removed:      Some(30)
NULL <-> 10 <-> 20 <-> 40 <-> NULL

08. ACCESS ELEMENT
Element:      Some(20)

09. MODIFY ELEMENT
NULL <-> 10 <-> 200 <-> 30 <-> NULL

10. SEARCH
Contains 20:  true
Contains 90:  false

11. FIND INDEX
Index:        Some(2)

12. BACKWARD TRAVERSAL
NULL <-> 30 <-> 20 <-> 10 <-> NULL

13. REVERSE
NULL <-> 30 <-> 20 <-> 10 <-> NULL

14. LENGTH
Length:       2

15. EMPTY
Is empty:     true

16. SUM
Sum:          60

17. CLONE
Original:
NULL <-> 10 <-> 20 <-> 30 <-> NULL
Copied:
NULL <-> 10 <-> 20 <-> 30 <-> NULL

18. CLEAR
Is empty:     true
Length:       0

19. MEMORY
Node size:    24 bytes
List size:    24 bytes
Box size:     8 bytes
Pointer size: 8 bytes

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/



