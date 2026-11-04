//---------------------------------------------------
// SinglyLinkedList - Rust Data Structures
// Date: 20261107
//---------------------------------------------------


use std::mem;

//==============================================================
// 01. NODE STRUCTURE
//==============================================================

#[derive(Debug, Clone)]
struct Node
{
    data: i32,
    next: Option<Box<Node>>,
}

//==============================================================
// 02. SINGLY LINKED LIST STRUCTURE
//==============================================================

#[derive(Debug, Clone)]
struct SinglyLinkedList
{
    head: Option<Box<Node>>,
    len: usize,
}

//==============================================================
// 03. IMPLEMENT LINKED LIST OPERATIONS
//==============================================================

impl SinglyLinkedList
{
    //----------------------------------------------------------
    // 03.1 CREATE EMPTY LIST
    //----------------------------------------------------------

    fn new() -> Self
    {
        Self
        {
            head: None,
            len: 0,
        }
    }

    //----------------------------------------------------------
    // 03.2 CHECK WHETHER LIST IS EMPTY
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
        let new_node = Box::new(Node
        {
            data: value,
            next: self.head.take(),
        });

        self.head = Some(new_node);
        self.len += 1;
    }

    //----------------------------------------------------------
    // 03.5 INSERT AT BACK
    //----------------------------------------------------------

    fn push_back(&mut self, value: i32)
    {
        let new_node = Box::new(Node
        {
            data: value,
            next: None,
        });

        let mut current = &mut self.head;

        while let Some(node) = current
        {
            current = &mut node.next;
        }

        *current = Some(new_node);
        self.len += 1;
    }

    //----------------------------------------------------------
    // 03.6 REMOVE FROM FRONT
    //----------------------------------------------------------

    fn pop_front(&mut self) -> Option<i32>
    {
        let mut removed = self.head.take()?;

        self.head = removed.next.take();

        self.len -= 1;

        Some(removed.data)
    }

    //----------------------------------------------------------
    // 03.7 REMOVE FROM BACK
    //----------------------------------------------------------

    fn pop_back(&mut self) -> Option<i32>
    {
        if self.head.is_none()
        {
            return None;
        }

        if self.head.as_ref()?.next.is_none()
        {
            return self.pop_front();
        }

        let mut current = self.head.as_mut()?;

        while current.next.as_ref()?.next.is_some()
        {
            current = current.next.as_mut()?;
        }

        let removed = current.next.take()?;

        self.len -= 1;

        Some(removed.data)
    }

    //----------------------------------------------------------
    // 03.8 INSERT AT SPECIFIC INDEX
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

        let mut current = &mut self.head;

        for _ in 0..index
        {
            match current
            {
                Some(node) => current = &mut node.next,
                None => return false,
            }
        }

        let new_node = Box::new(Node
        {
            data: value,
            next: current.take(),
        });

        *current = Some(new_node);

        self.len += 1;

        true
    }

    //----------------------------------------------------------
    // 03.9 REMOVE AT SPECIFIC INDEX
    //----------------------------------------------------------

    fn remove(&mut self, index: usize) -> Option<i32>
    {
        if index >= self.len
        {
            return None;
        }

        let mut current = &mut self.head;

        for _ in 0..index
        {
            current = &mut current.as_mut()?.next;
        }

        let mut removed = current.take()?;

        *current = removed.next.take();

        self.len -= 1;

        Some(removed.data)
    }

    //----------------------------------------------------------
    // 03.10 GET ELEMENT AT INDEX
    //----------------------------------------------------------

    fn get(&self, index: usize) -> Option<&i32>
    {
        let mut current = self.head.as_deref();

        for _ in 0..index
        {
            current = current?.next.as_deref();
        }

        current.map(|node| &node.data)
    }

    //----------------------------------------------------------
    // 03.11 MODIFY ELEMENT AT INDEX
    //----------------------------------------------------------

    fn get_mut(&mut self, index: usize) -> Option<&mut i32>
    {
        let mut current = self.head.as_deref_mut();

        for _ in 0..index
        {
            current = current?.next.as_deref_mut();
        }

        current.map(|node| &mut node.data)
    }

    //----------------------------------------------------------
    // 03.12 SEARCH ELEMENT
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
    // 03.13 FIND ELEMENT INDEX
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
    // 03.14 REVERSE LINKED LIST
    //----------------------------------------------------------

    fn reverse(&mut self)
    {
        let mut previous: Option<Box<Node>> = None;

        let mut current = self.head.take();

        while let Some(mut node) = current
        {
            let next = node.next.take();

            node.next = previous;

            previous = Some(node);

            current = next;
        }

        self.head = previous;
    }

    //----------------------------------------------------------
    // 03.15 CLEAR LINKED LIST
    //----------------------------------------------------------

    fn clear(&mut self)
    {
        self.head = None;
        self.len = 0;
    }

    //----------------------------------------------------------
    // 03.16 DISPLAY LINKED LIST
    //----------------------------------------------------------

    fn display(&self)
    {
        let mut current = self.head.as_deref();

        print!("HEAD -> ");

        while let Some(node) = current
        {
            print!("{} -> ", node.data);

            current = node.next.as_deref();
        }

        println!("NULL");
    }

    //----------------------------------------------------------
    // 03.17 SUM OF ELEMENTS
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
    // 03.18 MINIMUM ELEMENT
    //----------------------------------------------------------

    fn min(&self) -> Option<i32>
    {
        let mut current = self.head.as_deref();

        let mut minimum = current?.data;

        while let Some(node) = current
        {
            if node.data < minimum
            {
                minimum = node.data;
            }

            current = node.next.as_deref();
        }

        Some(minimum)
    }

    //----------------------------------------------------------
    // 03.19 MAXIMUM ELEMENT
    //----------------------------------------------------------

    fn max(&self) -> Option<i32>
    {
        let mut current = self.head.as_deref();

        let mut maximum = current?.data;

        while let Some(node) = current
        {
            if node.data > maximum
            {
                maximum = node.data;
            }

            current = node.next.as_deref();
        }

        Some(maximum)
    }

    //----------------------------------------------------------
    // 03.20 APPEND ANOTHER LINKED LIST
    //----------------------------------------------------------

    fn append(&mut self, other: &mut Self)
    {
        if other.is_empty()
        {
            return;
        }

        let mut current = &mut self.head;

        while let Some(node) = current
        {
            current = &mut node.next;
        }

        *current = other.head.take();

        self.len += other.len;
        other.len = 0;
    }

    //----------------------------------------------------------
    // 03.21 CLONE LINKED LIST
    //----------------------------------------------------------

    fn clone_list(&self) -> Self
    {
        self.clone()
    }

    //----------------------------------------------------------
    // 03.22 ITERATE THROUGH LINKED LIST
    //----------------------------------------------------------

    fn iter(&self) -> Iter<'_>
    {
        Iter
        {
            next: self.head.as_deref(),
        }
    }
}

//==============================================================
// 04. LINKED LIST ITERATOR
//==============================================================

struct Iter<'a>
{
    next: Option<&'a Node>,
}

impl<'a> Iterator for Iter<'a>
{
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item>
    {
        let node = self.next?;

        self.next = node.next.as_deref();

        Some(&node.data)
    }
}

//==============================================================
// 05. CREATE AND DISPLAY LINKED LIST
//==============================================================

fn create_and_display()
{
    let mut list = SinglyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    list.display();
}

//==============================================================
// 06. INSERT AT FRONT
//==============================================================

fn insert_front()
{
    let mut list = SinglyLinkedList::new();

    list.push_front(30);
    list.push_front(20);
    list.push_front(10);

    list.display();
}

//==============================================================
// 07. INSERT AT BACK
//==============================================================

fn insert_back()
{
    let mut list = SinglyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    list.display();
}

//==============================================================
// 08. REMOVE FROM FRONT
//==============================================================

fn remove_front()
{
    let mut list = SinglyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    println!("Removed:      {:?}", list.pop_front());

    list.display();
}

//==============================================================
// 09. REMOVE FROM BACK
//==============================================================

fn remove_back()
{
    let mut list = SinglyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    println!("Removed:      {:?}", list.pop_back());

    list.display();
}

//==============================================================
// 10. INSERT AT INDEX
//==============================================================

fn insert_at_index()
{
    let mut list = SinglyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(40);

    list.insert(2, 30);

    list.display();
}

//==============================================================
// 11. REMOVE AT INDEX
//==============================================================

fn remove_at_index()
{
    let mut list = SinglyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);
    list.push_back(40);

    println!("Removed:      {:?}", list.remove(2));

    list.display();
}

//==============================================================
// 12. ACCESS ELEMENT
//==============================================================

fn access_element()
{
    let mut list = SinglyLinkedList::new();

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
    let mut list = SinglyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    if let Some(value) = list.get_mut(1)
    {
        *value = 200;
    }

    list.display();
}

//==============================================================
// 14. SEARCH ELEMENT
//==============================================================

fn search_element()
{
    let mut list = SinglyLinkedList::new();

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
    let mut list = SinglyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    println!("Index:        {:?}", list.find(20));
}

//==============================================================
// 16. REVERSE LINKED LIST
//==============================================================

fn reverse_list()
{
    let mut list = SinglyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    list.reverse();

    list.display();
}

//==============================================================
// 17. CHECK LENGTH
//==============================================================

fn check_length()
{
    let mut list = SinglyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    println!("Length:       {}", list.len());
}

//==============================================================
// 18. CHECK EMPTY
//==============================================================

fn check_empty()
{
    let list = SinglyLinkedList::new();

    println!("Is empty:     {}", list.is_empty());
}

//==============================================================
// 19. SUM OF ELEMENTS
//==============================================================

fn calculate_sum()
{
    let mut list = SinglyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    println!("Sum:          {}", list.sum());
}

//==============================================================
// 20. MINIMUM ELEMENT
//==============================================================

fn find_minimum()
{
    let mut list = SinglyLinkedList::new();

    list.push_back(30);
    list.push_back(10);
    list.push_back(20);

    println!("Minimum:      {:?}", list.min());
}

//==============================================================
// 21. MAXIMUM ELEMENT
//==============================================================

fn find_maximum()
{
    let mut list = SinglyLinkedList::new();

    list.push_back(30);
    list.push_back(10);
    list.push_back(20);

    println!("Maximum:      {:?}", list.max());
}

//==============================================================
// 22. APPEND TWO LINKED LISTS
//==============================================================

fn append_lists()
{
    let mut first = SinglyLinkedList::new();
    let mut second = SinglyLinkedList::new();

    first.push_back(10);
    first.push_back(20);

    second.push_back(30);
    second.push_back(40);

    first.append(&mut second);

    println!("First list:");
    first.display();

    println!("Second list:");
    second.display();
}

//==============================================================
// 23. CLONE LINKED LIST
//==============================================================

fn clone_list()
{
    let mut original = SinglyLinkedList::new();

    original.push_back(10);
    original.push_back(20);
    original.push_back(30);

    let copied = original.clone_list();

    println!("Original:");
    original.display();

    println!("Copied:");
    copied.display();
}

//==============================================================
// 24. ITERATE USING ITERATOR
//==============================================================

fn iterate_list()
{
    let mut list = SinglyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    for value in list.iter()
    {
        println!("Value:        {}", value);
    }
}

//==============================================================
// 25. CLEAR LINKED LIST
//==============================================================

fn clear_list()
{
    let mut list = SinglyLinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    list.clear();

    println!("Is empty:     {}", list.is_empty());
    println!("Length:       {}", list.len());
}

//==============================================================
// 26. MEMORY SIZE
//==============================================================

fn memory_size()
{
    println!(
        "Node size:    {} bytes",
        mem::size_of::<Node>()
    );

    println!(
        "Box size:     {} bytes",
        mem::size_of::<Option<Box<Node>>>()
    );

    println!(
        "List size:    {} bytes",
        mem::size_of::<SinglyLinkedList>()
    );
}

//==============================================================
// 27. MAIN FUNCTION
//==============================================================

fn main()
{
    println!("\n01. CREATE AND DISPLAY");
    create_and_display();

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

    println!("\n10. SEARCH ELEMENT");
    search_element();

    println!("\n11. FIND ELEMENT");
    find_element();

    println!("\n12. REVERSE");
    reverse_list();

    println!("\n13. LENGTH");
    check_length();

    println!("\n14. EMPTY");
    check_empty();

    println!("\n15. SUM");
    calculate_sum();

    println!("\n16. MINIMUM");
    find_minimum();

    println!("\n17. MAXIMUM");
    find_maximum();

    println!("\n18. APPEND");
    append_lists();

    println!("\n19. CLONE");
    clone_list();

    println!("\n20. ITERATOR");
    iterate_list();

    println!("\n21. CLEAR");
    clear_list();

    println!("\n22. MEMORY SIZE");
    memory_size();
}

/*
//---------------------------------------------------
Time complexity:

Operation - Time complexity

Insert at front     - O(1)
Remove from front   - O(1)
Insert at back      - O(n)
Remove from back    - O(n)
Search              - O(n)
Access by index     - O(n)
Reverse             - O(n)
Get length          - O(1)

//---------------------------------------------------


//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261107_ds_SinglyLinkedList_01.rs -o singlylinkedlist

./singlylinkedlist

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

01. CREATE AND DISPLAY
HEAD -> 10 -> 20 -> 30 -> NULL

02. INSERT FRONT
HEAD -> 10 -> 20 -> 30 -> NULL

03. INSERT BACK
HEAD -> 10 -> 20 -> 30 -> NULL

04. REMOVE FRONT
Removed:      Some(10)
HEAD -> 20 -> 30 -> NULL

05. REMOVE BACK
Removed:      Some(30)
HEAD -> 10 -> 20 -> NULL

06. INSERT AT INDEX
HEAD -> 10 -> 20 -> 30 -> 40 -> NULL

07. REMOVE AT INDEX
Removed:      Some(30)
HEAD -> 10 -> 20 -> 40 -> NULL

08. ACCESS ELEMENT
Element:      Some(20)

09. MODIFY ELEMENT
HEAD -> 10 -> 200 -> 30 -> NULL

10. SEARCH ELEMENT
Contains 20:  true
Contains 90:  false

11. FIND ELEMENT
Index:        Some(1)

12. REVERSE
HEAD -> 30 -> 20 -> 10 -> NULL

13. LENGTH
Length:       3

14. EMPTY
Is empty:     true

15. SUM
Sum:          60

16. MINIMUM
Minimum:      Some(10)

17. MAXIMUM
Maximum:      Some(30)

18. APPEND
First list:
HEAD -> 10 -> 20 -> 30 -> 40 -> NULL
Second list:
HEAD -> NULL

19. CLONE
Original:
HEAD -> 10 -> 20 -> 30 -> NULL
Copied:
HEAD -> 10 -> 20 -> 30 -> NULL

20. ITERATOR
Value:        10
Value:        20
Value:        30

21. CLEAR
Is empty:     true
Length:       0

22. MEMORY SIZE
Node size:    16 bytes
Box size:     8 bytes
List size:    16 bytes

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/


