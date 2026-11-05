//---------------------------------------------------
// HashSet - Rust Data Structures
// Date: 20261122
//---------------------------------------------------

use std::collections::HashSet;
use std::mem::size_of;

//--------------------------------------------------
// 01. Employee structure
//--------------------------------------------------

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Employee {
    id: u32,
    name: String,
}

//--------------------------------------------------
// 02. Display HashSet
//--------------------------------------------------

fn display_set<T>(set: &HashSet<T>)
where
    T: std::fmt::Debug + Eq + std::hash::Hash,
{
    for value in set {
        println!("{:?}", value);
    }
}

//--------------------------------------------------
// 03. Main function
//--------------------------------------------------

fn main() {

    //--------------------------------------------------
    // 04. Create an empty HashSet
    //--------------------------------------------------

    println!("\n04. Create HashSet");

    let mut set: HashSet<i32> = HashSet::new();

    println!("Is empty: {}", set.is_empty());
    println!("Length  : {}", set.len());

    //--------------------------------------------------
    // 05. Insert elements
    //--------------------------------------------------

    println!("\n05. Insert elements");

    set.insert(10);
    set.insert(20);
    set.insert(30);
    set.insert(40);

    display_set(&set);

    //--------------------------------------------------
    // 06. Insert duplicate elements
    //--------------------------------------------------

    println!("\n06. Insert duplicates");

    set.insert(10);
    set.insert(20);
    set.insert(10);

    println!("Length after duplicate insertion: {}", set.len());

    display_set(&set);

    // HashSet stores each unique value only once.

    //--------------------------------------------------
    // 07. Check whether an element exists
    //--------------------------------------------------

    println!("\n07. contains()");

    println!("10 exists: {}", set.contains(&10));
    println!("99 exists: {}", set.contains(&99));

    //--------------------------------------------------
    // 08. Remove an element
    //--------------------------------------------------

    println!("\n08. Remove element");

    println!("Removed 20: {}", set.remove(&20));
    println!("Removed 99: {}", set.remove(&99));

    display_set(&set);

    //--------------------------------------------------
    // 09. Get length and empty status
    //--------------------------------------------------

    println!("\n09. Length and empty status");

    println!("Length  : {}", set.len());
    println!("Is empty: {}", set.is_empty());

    //--------------------------------------------------
    // 10. Iterate over HashSet
    //--------------------------------------------------

    println!("\n10. Iterate");

    for value in &set {
        println!("Value: {}", value);
    }

    // Iteration order is not guaranteed.

    //--------------------------------------------------
    // 11. Create HashSet with capacity
    //--------------------------------------------------

    println!("\n11. HashSet with capacity");

    let mut capacity_set: HashSet<i32> =
        HashSet::with_capacity(20);

    capacity_set.insert(100);
    capacity_set.insert(200);

    println!("Capacity: {}", capacity_set.capacity());

    //--------------------------------------------------
    // 12. Reserve additional capacity
    //--------------------------------------------------

    println!("\n12. Reserve capacity");

    capacity_set.reserve(50);

    println!("Capacity: {}", capacity_set.capacity());

    //--------------------------------------------------
    // 13. Shrink capacity
    //--------------------------------------------------

    println!("\n13. Shrink capacity");

    capacity_set.shrink_to_fit();

    println!("Capacity: {}", capacity_set.capacity());

    //--------------------------------------------------
    // 14. Clear HashSet
    //--------------------------------------------------

    println!("\n14. Clear HashSet");

    capacity_set.clear();

    println!("Length: {}", capacity_set.len());
    println!("Is empty: {}", capacity_set.is_empty());

    //--------------------------------------------------
    // 15. HashSet from an array
    //--------------------------------------------------

    println!("\n15. Create from array");

    let array_set: HashSet<i32> =
        HashSet::from([10, 20, 30, 20, 40, 10]);

    display_set(&array_set);

    println!("Unique elements: {}", array_set.len());

    //--------------------------------------------------
    // 16. HashSet from a vector
    //--------------------------------------------------

    println!("\n16. Create from Vec");

    let numbers = vec![1, 2, 3, 2, 4, 3, 5];

    let vector_set: HashSet<i32> =
        numbers.into_iter().collect();

    display_set(&vector_set);

    //--------------------------------------------------
    // 17. Convert HashSet into Vec
    //--------------------------------------------------

    println!("\n17. Convert into Vec");

    let result: Vec<i32> = vector_set.into_iter().collect();

    println!("Vector: {:?}", result);

    //--------------------------------------------------
    // 18. Clone HashSet
    //--------------------------------------------------

    println!("\n18. Clone HashSet");

    let original: HashSet<i32> =
        HashSet::from([10, 20, 30]);

    let cloned = original.clone();

    println!("Original: {:?}", original);
    println!("Cloned  : {:?}", cloned);

    //--------------------------------------------------
    // 19. Extend HashSet
    //--------------------------------------------------

    println!("\n19. Extend HashSet");

    let mut first: HashSet<i32> =
        HashSet::from([1, 2, 3]);

    let second: HashSet<i32> =
        HashSet::from([3, 4, 5]);

    first.extend(second);

    display_set(&first);

    //--------------------------------------------------
    // 20. Union of two sets
    //--------------------------------------------------

    println!("\n20. Union");

    let set_a: HashSet<i32> =
        HashSet::from([1, 2, 3, 4]);

    let set_b: HashSet<i32> =
        HashSet::from([3, 4, 5, 6]);

    let union: HashSet<i32> =
        set_a.union(&set_b).copied().collect();

    println!("Set A: {:?}", set_a);
    println!("Set B: {:?}", set_b);
    println!("Union: {:?}", union);

    //--------------------------------------------------
    // 21. Intersection of two sets
    //--------------------------------------------------

    println!("\n21. Intersection");

    let intersection: HashSet<i32> =
        set_a.intersection(&set_b).copied().collect();

    println!("Intersection: {:?}", intersection);

    //--------------------------------------------------
    // 22. Difference of two sets
    //--------------------------------------------------

    println!("\n22. Difference");

    let difference: HashSet<i32> =
        set_a.difference(&set_b).copied().collect();

    println!("A - B: {:?}", difference);

    //--------------------------------------------------
    // 23. Symmetric difference
    //--------------------------------------------------

    println!("\n23. Symmetric difference");

    let symmetric: HashSet<i32> =
        set_a.symmetric_difference(&set_b).copied().collect();

    println!("Symmetric difference: {:?}", symmetric);

    //--------------------------------------------------
    // 24. Check subset
    //--------------------------------------------------

    println!("\n24. Subset");

    let small_set: HashSet<i32> =
        HashSet::from([1, 2]);

    let large_set: HashSet<i32> =
        HashSet::from([1, 2, 3, 4]);

    println!(
        "Small is subset of large: {}",
        small_set.is_subset(&large_set)
    );

    //--------------------------------------------------
    // 25. Check superset
    //--------------------------------------------------

    println!("\n25. Superset");

    println!(
        "Large is superset of small: {}",
        large_set.is_superset(&small_set)
    );

    //--------------------------------------------------
    // 26. Check disjoint sets
    //--------------------------------------------------

    println!("\n26. Disjoint");

    let group_a: HashSet<i32> =
        HashSet::from([1, 2, 3]);

    let group_b: HashSet<i32> =
        HashSet::from([7, 8, 9]);

    let group_c: HashSet<i32> =
        HashSet::from([3, 4, 5]);

    println!("A and B disjoint: {}", group_a.is_disjoint(&group_b));
    println!("A and C disjoint: {}", group_a.is_disjoint(&group_c));

    //--------------------------------------------------
    // 27. Retain selected elements
    //--------------------------------------------------

    println!("\n27. Retain");

    let mut retain_set: HashSet<i32> =
        HashSet::from([1, 2, 3, 4, 5, 6, 7, 8]);

    retain_set.retain(|value| value % 2 == 0);

    println!("Even elements: {:?}", retain_set);

    //--------------------------------------------------
    // 28. HashSet with String
    //--------------------------------------------------

    println!("\n28. String HashSet");

    let mut languages: HashSet<String> = HashSet::new();

    languages.insert(String::from("C"));
    languages.insert(String::from("C++"));
    languages.insert(String::from("Rust"));
    languages.insert(String::from("Python"));
    languages.insert(String::from("Rust"));

    display_set(&languages);

    //--------------------------------------------------
    // 29. HashSet with characters
    //--------------------------------------------------

    println!("\n29. Character HashSet");

    let mut characters: HashSet<char> = HashSet::new();

    for ch in "programming".chars() {
        characters.insert(ch);
    }

    display_set(&characters);

    println!("Unique characters: {}", characters.len());

    //--------------------------------------------------
    // 30. HashSet with tuples
    //--------------------------------------------------

    println!("\n30. Tuple HashSet");

    let mut coordinates: HashSet<(i32, i32)> =
        HashSet::new();

    coordinates.insert((10, 20));
    coordinates.insert((30, 40));
    coordinates.insert((10, 20));

    display_set(&coordinates);

    //--------------------------------------------------
    // 31. HashSet with structures
    //--------------------------------------------------

    println!("\n31. Struct HashSet");

    let mut employees: HashSet<Employee> =
        HashSet::new();

    employees.insert(Employee {
        id: 101,
        name: String::from("Arun"),
    });

    employees.insert(Employee {
        id: 102,
        name: String::from("Bala"),
    });

    employees.insert(Employee {
        id: 101,
        name: String::from("Arun"),
    });

    display_set(&employees);

    println!("Unique employees: {}", employees.len());

    //--------------------------------------------------
    // 32. HashSet of IP addresses
    //--------------------------------------------------

    println!("\n32. IP address HashSet");

    let mut ip_addresses: HashSet<&str> =
        HashSet::new();

    ip_addresses.insert("192.168.1.10");
    ip_addresses.insert("192.168.1.20");
    ip_addresses.insert("192.168.1.30");
    ip_addresses.insert("192.168.1.10");

    println!("Unique IP addresses: {}", ip_addresses.len());

    println!(
        "Pi IP exists: {}",
        ip_addresses.contains("192.168.1.30")
    );

    //--------------------------------------------------
    // 33. Remove duplicates from a vector
    //--------------------------------------------------

    println!("\n33. Remove duplicates");

    let values = vec![10, 20, 10, 30, 20, 40, 30];

    let unique: HashSet<i32> =
        values.into_iter().collect();

    println!("Unique values: {:?}", unique);

    //--------------------------------------------------
    // 34. Find common elements
    //--------------------------------------------------

    println!("\n34. Find common elements");

    let students_a: HashSet<&str> =
        HashSet::from(["Arun", "Bala", "Charan"]);

    let students_b: HashSet<&str> =
        HashSet::from(["Bala", "Deepak", "Arun"]);

    let common: HashSet<&str> =
        students_a.intersection(&students_b).copied().collect();

    println!("Common students: {:?}", common);

    //--------------------------------------------------
    // 35. Find unique elements
    //--------------------------------------------------

    println!("\n35. Find unique elements");

    let unique_students: HashSet<&str> =
        students_a.symmetric_difference(&students_b).copied().collect();

    println!("Unique students: {:?}", unique_students);

    //--------------------------------------------------
    // 36. HashSet equality
    //--------------------------------------------------

    println!("\n36. HashSet equality");

    let first_set: HashSet<i32> =
        HashSet::from([1, 2, 3]);

    let second_set: HashSet<i32> =
        HashSet::from([3, 2, 1]);

    println!("Sets equal: {}", first_set == second_set);

    // Equality does not depend on iteration order.

    //--------------------------------------------------
    // 37. Remove an element using take()
    //--------------------------------------------------

    println!("\n37. take()");

    let mut take_set: HashSet<String> =
        HashSet::from([
            String::from("Rust"),
            String::from("C"),
        ]);

    match take_set.take("Rust") {
        Some(value) => println!("Taken value: {}", value),
        None => println!("Value not found"),
    }

    println!("Remaining: {:?}", take_set);

    //--------------------------------------------------
    // 38. HashSet memory descriptor size
    //--------------------------------------------------

    println!("\n38. Memory size");

    println!(
        "HashSet<i32> descriptor: {} bytes",
        size_of::<HashSet<i32>>()
    );

    println!(
        "i32 element size: {} bytes",
        size_of::<i32>()
    );

    // Descriptor size excludes dynamically allocated table storage.

    //--------------------------------------------------
    // 39. Clear and rebuild a set
    //--------------------------------------------------

    println!("\n39. Clear and rebuild");

    let mut rebuild_set: HashSet<i32> =
        HashSet::from([10, 20, 30]);

    rebuild_set.clear();

    rebuild_set.extend([100, 200, 300]);

    println!("Rebuilt set: {:?}", rebuild_set);

    //--------------------------------------------------
    // 40. Final status
    //--------------------------------------------------

    println!("\n40. HashSet program completed successfully.");
}


/*
//---------------------------------------------------
A HashSet<T> is a collection that stores unique values without key-value pairs.

Unlike a HashMap, a HashSet does not store a separate value for each key.
- Duplicate values are automatically ignored.
- Elements are not stored in sorted order.
- Fast average insertion, lookup and removal.
- Supports mathematical set operations such as union, intersection and difference.

//---------------------------------------------------
Important HashSet methods:

Method - Purpose
new() 
- Create empty set

insert()	
- Insert element

contains()	
- Check existence

remove()	
- Delete element

take()	
- Remove and return stored element

len()
- Number of elements

is_empty()	
- Check whether empty

clear()	
- Remove all elements

union()	
- Combine sets

intersection()	
- Find common elements

difference()	
- Find elements in one set only

symmetric_difference()	
- Find elements exclusive to either set

is_subset()	
- Check subset

is_superset()	
- Check superset

is_disjoint()	
- Check for no common elements

retain()	
- Keep matching elements

//---------------------------------------------------
Important Rust concept: 

Elements stored in a HashSet must implement Eq + Hash. If you create your own struct, derive or implement these traits consistently.

HashMap vs HashSet

Feature - HashMap - HashSet

Storage - Key-value pairs - Unique values
Duplicate keys - Not allowed - Not applicable
Duplicate values - Allowed - Not allowed
Lookup - By key - By element
Main use - Mapping - Membership checking

//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261122_ds_HashSet_01.rs -o hashset_example

./hashset_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

04. Create HashSet
Is empty: true
Length  : 0

05. Insert elements
20
40
10
30

06. Insert duplicates
Length after duplicate insertion: 4
20
40
10
30

07. contains()
10 exists: true
99 exists: false

08. Remove element
Removed 20: true
Removed 99: false
40
10
30

09. Length and empty status
Length  : 3
Is empty: false

10. Iterate
Value: 40
Value: 10
Value: 30

11. HashSet with capacity
Capacity: 28

12. Reserve capacity
Capacity: 56

13. Shrink capacity
Capacity: 3

14. Clear HashSet
Length: 0
Is empty: true

15. Create from array
10
30
20
40
Unique elements: 4

16. Create from Vec
5
3
2
4
1

17. Convert into Vec
Vector: [5, 3, 2, 4, 1]

18. Clone HashSet
Original: {10, 20, 30}
Cloned  : {10, 20, 30}

19. Extend HashSet
2
4
1
5
3

20. Union
Set A: {2, 3, 1, 4}
Set B: {6, 3, 5, 4}
Union: {5, 2, 1, 4, 3, 6}

21. Intersection
Intersection: {3, 4}

22. Difference
A - B: {1, 2}

23. Symmetric difference
Symmetric difference: {1, 2, 6, 5}

24. Subset
Small is subset of large: true

25. Superset
Large is superset of small: true

26. Disjoint
A and B disjoint: true
A and C disjoint: false

27. Retain
Even elements: {6, 4, 8, 2}

28. String HashSet
"C++"
"Python"
"Rust"
"C"

29. Character HashSet
'a'
'p'
'r'
'n'
'g'
'o'
'i'
'm'
Unique characters: 8

30. Tuple HashSet
(30, 40)
(10, 20)

31. Struct HashSet
Employee { id: 102, name: "Bala" }
Employee { id: 101, name: "Arun" }
Unique employees: 2

32. IP address HashSet
Unique IP addresses: 3
Pi IP exists: true

33. Remove duplicates
Unique values: {40, 20, 30, 10}

34. Find common elements
Common students: {"Bala", "Arun"}

35. Find unique elements
Unique students: {"Charan", "Deepak"}

36. HashSet equality
Sets equal: true

37. take()
Taken value: Rust
Remaining: {"C"}

38. Memory size
HashSet<i32> descriptor: 48 bytes
i32 element size: 4 bytes

39. Clear and rebuild
Rebuilt set: {100, 200, 300}

40. HashSet program completed successfully.

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/