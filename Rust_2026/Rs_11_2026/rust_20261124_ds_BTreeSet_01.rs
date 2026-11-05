//---------------------------------------------------
// BTreeSet - Rust Data Structures
// Date: 20261124
//---------------------------------------------------

use std::collections::BTreeSet;
use std::mem::size_of;

//--------------------------------------------------
// 01. Employee structure
//--------------------------------------------------

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Employee {
    id: u32,
    name: String,
}

//--------------------------------------------------
// 02. Display BTreeSet
//--------------------------------------------------

fn display_set<T>(set: &BTreeSet<T>)
where
    T: std::fmt::Debug + Ord,
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
    // 04. Create an empty BTreeSet
    //--------------------------------------------------

    println!("\n04. Create BTreeSet");

    let mut set: BTreeSet<i32> = BTreeSet::new();

    println!("Is empty: {}", set.is_empty());
    println!("Length  : {}", set.len());

    //--------------------------------------------------
    // 05. Insert elements
    //--------------------------------------------------

    println!("\n05. Insert elements");

    set.insert(40);
    set.insert(10);
    set.insert(30);
    set.insert(20);
    set.insert(50);

    display_set(&set);

    // Elements are automatically sorted.

    //--------------------------------------------------
    // 06. Insert duplicate elements
    //--------------------------------------------------

    println!("\n06. Insert duplicates");

    let inserted = set.insert(20);
    println!("Was 20 inserted? {}", inserted);

    let inserted = set.insert(60);
    println!("Was 60 inserted? {}", inserted);

    println!("Length: {}", set.len());

    display_set(&set);

    //--------------------------------------------------
    // 07. Check whether an element exists
    //--------------------------------------------------

    println!("\n07. contains()");

    println!("20 exists: {}", set.contains(&20));
    println!("99 exists: {}", set.contains(&99));

    //--------------------------------------------------
    // 08. Remove an element
    //--------------------------------------------------

    println!("\n08. Remove element");

    println!("Removed 30: {}", set.remove(&30));
    println!("Removed 99: {}", set.remove(&99));

    display_set(&set);

    //--------------------------------------------------
    // 09. Get length and empty status
    //--------------------------------------------------

    println!("\n09. Length and empty status");

    println!("Length  : {}", set.len());
    println!("Is empty: {}", set.is_empty());

    //--------------------------------------------------
    // 10. Iterate over BTreeSet
    //--------------------------------------------------

    println!("\n10. Iterate");

    for value in &set {
        println!("Value: {}", value);
    }

    //--------------------------------------------------
    // 11. Create BTreeSet from an array
    //--------------------------------------------------

    println!("\n11. Create from array");

    let array_set: BTreeSet<i32> =
        BTreeSet::from([50, 10, 30, 20, 40, 10]);

    display_set(&array_set);

    //--------------------------------------------------
    // 12. Create BTreeSet from a vector
    //--------------------------------------------------

    println!("\n12. Create from Vec");

    let numbers = vec![5, 3, 1, 4, 2, 3, 1];

    let vector_set: BTreeSet<i32> =
        numbers.into_iter().collect();

    display_set(&vector_set);

    //--------------------------------------------------
    // 13. Convert BTreeSet into Vec
    //--------------------------------------------------

    println!("\n13. Convert into Vec");

    let result: Vec<i32> = vector_set.into_iter().collect();

    println!("Sorted vector: {:?}", result);

    //--------------------------------------------------
    // 14. Clone BTreeSet
    //--------------------------------------------------

    println!("\n14. Clone BTreeSet");

    let original: BTreeSet<i32> =
        BTreeSet::from([10, 20, 30]);

    let cloned = original.clone();

    println!("Original: {:?}", original);
    println!("Cloned  : {:?}", cloned);

    //--------------------------------------------------
    // 15. Extend BTreeSet
    //--------------------------------------------------

    println!("\n15. Extend BTreeSet");

    let mut first: BTreeSet<i32> =
        BTreeSet::from([1, 2, 3]);

    let second: BTreeSet<i32> =
        BTreeSet::from([3, 4, 5]);

    first.extend(second);

    println!("Extended set: {:?}", first);

    //--------------------------------------------------
    // 16. Union of two sets
    //--------------------------------------------------

    println!("\n16. Union");

    let set_a: BTreeSet<i32> =
        BTreeSet::from([1, 2, 3, 4]);

    let set_b: BTreeSet<i32> =
        BTreeSet::from([3, 4, 5, 6]);

    let union: BTreeSet<i32> =
        set_a.union(&set_b).copied().collect();

    println!("Set A: {:?}", set_a);
    println!("Set B: {:?}", set_b);
    println!("Union: {:?}", union);

    //--------------------------------------------------
    // 17. Intersection of two sets
    //--------------------------------------------------

    println!("\n17. Intersection");

    let intersection: BTreeSet<i32> =
        set_a.intersection(&set_b).copied().collect();

    println!("Intersection: {:?}", intersection);

    //--------------------------------------------------
    // 18. Difference of two sets
    //--------------------------------------------------

    println!("\n18. Difference");

    let difference: BTreeSet<i32> =
        set_a.difference(&set_b).copied().collect();

    println!("A - B: {:?}", difference);

    let reverse_difference: BTreeSet<i32> =
        set_b.difference(&set_a).copied().collect();

    println!("B - A: {:?}", reverse_difference);

    //--------------------------------------------------
    // 19. Symmetric difference
    //--------------------------------------------------

    println!("\n19. Symmetric difference");

    let symmetric: BTreeSet<i32> =
        set_a.symmetric_difference(&set_b).copied().collect();

    println!("Symmetric difference: {:?}", symmetric);

    //--------------------------------------------------
    // 20. Check subset
    //--------------------------------------------------

    println!("\n20. Subset");

    let small_set: BTreeSet<i32> =
        BTreeSet::from([1, 2]);

    let large_set: BTreeSet<i32> =
        BTreeSet::from([1, 2, 3, 4]);

    println!(
        "Small is subset of large: {}",
        small_set.is_subset(&large_set)
    );

    //--------------------------------------------------
    // 21. Check superset
    //--------------------------------------------------

    println!("\n21. Superset");

    println!(
        "Large is superset of small: {}",
        large_set.is_superset(&small_set)
    );

    //--------------------------------------------------
    // 22. Check disjoint sets
    //--------------------------------------------------

    println!("\n22. Disjoint");

    let group_a: BTreeSet<i32> =
        BTreeSet::from([1, 2, 3]);

    let group_b: BTreeSet<i32> =
        BTreeSet::from([7, 8, 9]);

    let group_c: BTreeSet<i32> =
        BTreeSet::from([3, 4, 5]);

    println!("A and B disjoint: {}", group_a.is_disjoint(&group_b));
    println!("A and C disjoint: {}", group_a.is_disjoint(&group_c));

    //--------------------------------------------------
    // 23. First element
    //--------------------------------------------------

    println!("\n23. First element");

    match set.first() {
        Some(value) => println!("Smallest: {}", value),
        None => println!("Set is empty"),
    }

    //--------------------------------------------------
    // 24. Last element
    //--------------------------------------------------

    println!("\n24. Last element");

    match set.last() {
        Some(value) => println!("Largest: {}", value),
        None => println!("Set is empty"),
    }

    //--------------------------------------------------
    // 25. Pop first element
    //--------------------------------------------------

    println!("\n25. Pop first element");

    let mut pop_set: BTreeSet<i32> =
        BTreeSet::from([10, 20, 30, 40, 50]);

    match pop_set.pop_first() {
        Some(value) => println!("Removed smallest: {}", value),
        None => println!("Set is empty"),
    }

    println!("Remaining: {:?}", pop_set);

    //--------------------------------------------------
    // 26. Pop last element
    //--------------------------------------------------

    println!("\n26. Pop last element");

    match pop_set.pop_last() {
        Some(value) => println!("Removed largest: {}", value),
        None => println!("Set is empty"),
    }

    println!("Remaining: {:?}", pop_set);

    //--------------------------------------------------
    // 27. Range-based lookup
    //--------------------------------------------------

    println!("\n27. Range lookup");

    let range_set: BTreeSet<i32> =
        BTreeSet::from([10, 20, 30, 40, 50, 60, 70]);

    for value in range_set.range(20..=50) {
        println!("{}", value);
    }

    //--------------------------------------------------
    // 28. Range below a value
    //--------------------------------------------------

    println!("\n28. Range below 40");

    for value in range_set.range(..40) {
        println!("{}", value);
    }

    //--------------------------------------------------
    // 29. Range above a value
    //--------------------------------------------------

    println!("\n29. Range from 40");

    for value in range_set.range(40..) {
        println!("{}", value);
    }

    //--------------------------------------------------
    // 30. Reverse iteration
    //--------------------------------------------------

    println!("\n30. Reverse iteration");

    for value in range_set.iter().rev() {
        println!("{}", value);
    }

    //--------------------------------------------------
    // 31. Retain selected elements
    //--------------------------------------------------

    println!("\n31. Retain");

    let mut retain_set: BTreeSet<i32> =
        BTreeSet::from([1, 2, 3, 4, 5, 6, 7, 8]);

    retain_set.retain(|value| value % 2 == 0);

    println!("Even elements: {:?}", retain_set);

    //--------------------------------------------------
    // 32. BTreeSet with String
    //--------------------------------------------------

    println!("\n32. String BTreeSet");

    let mut languages: BTreeSet<String> = BTreeSet::new();

    languages.insert(String::from("Rust"));
    languages.insert(String::from("C"));
    languages.insert(String::from("Python"));
    languages.insert(String::from("C++"));
    languages.insert(String::from("Rust"));

    display_set(&languages);

    //--------------------------------------------------
    // 33. BTreeSet with characters
    //--------------------------------------------------

    println!("\n33. Character BTreeSet");

    let mut characters: BTreeSet<char> = BTreeSet::new();

    for ch in "programming".chars() {
        characters.insert(ch);
    }

    display_set(&characters);

    println!("Unique characters: {}", characters.len());

    //--------------------------------------------------
    // 34. BTreeSet with tuples
    //--------------------------------------------------

    println!("\n34. Tuple BTreeSet");

    let mut coordinates: BTreeSet<(i32, i32)> =
        BTreeSet::new();

    coordinates.insert((30, 40));
    coordinates.insert((10, 20));
    coordinates.insert((20, 30));
    coordinates.insert((10, 20));

    display_set(&coordinates);

    // Tuples are ordered lexicographically.

    //--------------------------------------------------
    // 35. BTreeSet with structures
    //--------------------------------------------------

    println!("\n35. Struct BTreeSet");

    let mut employees: BTreeSet<Employee> =
        BTreeSet::new();

    employees.insert(Employee {
        id: 103,
        name: String::from("Charan"),
    });

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

    // Employee ordering follows id first, then name.

    //--------------------------------------------------
    // 36. Remove duplicates from a vector
    //--------------------------------------------------

    println!("\n36. Remove duplicates");

    let values = vec![10, 20, 10, 30, 20, 40, 30];

    let unique: BTreeSet<i32> =
        values.into_iter().collect();

    println!("Unique sorted values: {:?}", unique);

    //--------------------------------------------------
    // 37. Find common elements
    //--------------------------------------------------

    println!("\n37. Find common elements");

    let students_a: BTreeSet<&str> =
        BTreeSet::from(["Arun", "Bala", "Charan"]);

    let students_b: BTreeSet<&str> =
        BTreeSet::from(["Bala", "Deepak", "Arun"]);

    let common: BTreeSet<&str> =
        students_a.intersection(&students_b).copied().collect();

    println!("Common students: {:?}", common);

    //--------------------------------------------------
    // 38. Find unique elements
    //--------------------------------------------------

    println!("\n38. Find unique elements");

    let unique_students: BTreeSet<&str> =
        students_a.symmetric_difference(&students_b).copied().collect();

    println!("Unique students: {:?}", unique_students);

    //--------------------------------------------------
    // 39. BTreeSet equality
    //--------------------------------------------------

    println!("\n39. BTreeSet equality");

    let first_set: BTreeSet<i32> =
        BTreeSet::from([1, 2, 3]);

    let second_set: BTreeSet<i32> =
        BTreeSet::from([3, 2, 1]);

    println!("Sets equal: {}", first_set == second_set);

    //--------------------------------------------------
    // 40. Consume BTreeSet using into_iter()
    //--------------------------------------------------

    println!("\n40. Consume BTreeSet");

    let consume_set: BTreeSet<i32> =
        BTreeSet::from([30, 10, 20]);

    for value in consume_set {
        println!("Value: {}", value);
    }

    // Values are consumed in ascending order.

    //--------------------------------------------------
    // 41. BTreeSet with IP addresses
    //--------------------------------------------------

    println!("\n41. IP address BTreeSet");

    let mut ip_addresses: BTreeSet<String> =
        BTreeSet::new();

    ip_addresses.insert(String::from("192.168.1.30"));
    ip_addresses.insert(String::from("192.168.1.10"));
    ip_addresses.insert(String::from("192.168.1.20"));
    ip_addresses.insert(String::from("192.168.1.10"));

    display_set(&ip_addresses);

    println!(
        "Pi IP exists: {}",
        ip_addresses.contains("192.168.1.30")
    );

    //--------------------------------------------------
    // 42. Clear BTreeSet
    //--------------------------------------------------

    println!("\n42. Clear BTreeSet");

    let mut clear_set: BTreeSet<i32> =
        BTreeSet::from([10, 20, 30]);

    clear_set.clear();

    println!("Length: {}", clear_set.len());
    println!("Is empty: {}", clear_set.is_empty());

    //--------------------------------------------------
    // 43. BTreeSet memory descriptor size
    //--------------------------------------------------

    println!("\n43. Memory size");

    println!(
        "BTreeSet<i32> descriptor: {} bytes",
        size_of::<BTreeSet<i32>>()
    );

    println!(
        "i32 element size: {} bytes",
        size_of::<i32>()
    );

    // Descriptor size excludes allocated tree nodes.

    //--------------------------------------------------
    // 44. Final status
    //--------------------------------------------------

    println!("\n44. BTreeSet program completed successfully.");
}


/*
//---------------------------------------------------
Important BTreeSet methods:

Method	Purpose

new()
- Create empty set

insert()
- Insert unique element

contains()
- Check existence

remove()
- Remove element

first()
- Get smallest element

last()
- Get largest element

pop_first()
- Remove smallest element

pop_last()
- Remove largest element

range()
- Retrieve elements in a range

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

clear()
- Remove everything

//---------------------------------------------------
HashSet vs BTreeSet

Feature - HashSet - BTreeSet

Internal structure - Hash table - B-tree
Duplicate elements - Not allowed - Not allowed
Element ordering - Unspecified - Sorted
Search - Average O(1) - O(log n)
Insert - Average O(1) - O(log n)
Remove - Average O(1) - O(log n)
Range queries - Not directly supported - Supported
Element requirement - Eq + Hash - Ord

//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261124_ds_BTreeSet_01.rs -o btreeset_example

./btreeset_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

04. Create BTreeSet
Is empty: true
Length  : 0

05. Insert elements
10
20
30
40
50

06. Insert duplicates
Was 20 inserted? false
Was 60 inserted? true
Length: 6
10
20
30
40
50
60

07. contains()
20 exists: true
99 exists: false

08. Remove element
Removed 30: true
Removed 99: false
10
20
40
50
60

09. Length and empty status
Length  : 5
Is empty: false

10. Iterate
Value: 10
Value: 20
Value: 40
Value: 50
Value: 60

11. Create from array
10
20
30
40
50

12. Create from Vec
1
2
3
4
5

13. Convert into Vec
Sorted vector: [1, 2, 3, 4, 5]

14. Clone BTreeSet
Original: {10, 20, 30}
Cloned  : {10, 20, 30}

15. Extend BTreeSet
Extended set: {1, 2, 3, 4, 5}

16. Union
Set A: {1, 2, 3, 4}
Set B: {3, 4, 5, 6}
Union: {1, 2, 3, 4, 5, 6}

17. Intersection
Intersection: {3, 4}

18. Difference
A - B: {1, 2}
B - A: {5, 6}

19. Symmetric difference
Symmetric difference: {1, 2, 5, 6}

20. Subset
Small is subset of large: true

21. Superset
Large is superset of small: true

22. Disjoint
A and B disjoint: true
A and C disjoint: false

23. First element
Smallest: 10

24. Last element
Largest: 60

25. Pop first element
Removed smallest: 10
Remaining: {20, 30, 40, 50}

26. Pop last element
Removed largest: 50
Remaining: {20, 30, 40}

27. Range lookup
20
30
40
50

28. Range below 40
10
20
30

29. Range from 40
40
50
60
70

30. Reverse iteration
70
60
50
40
30
20
10

31. Retain
Even elements: {2, 4, 6, 8}

32. String BTreeSet
"C"
"C++"
"Python"
"Rust"

33. Character BTreeSet
'a'
'g'
'i'
'm'
'n'
'o'
'p'
'r'
Unique characters: 8

34. Tuple BTreeSet
(10, 20)
(20, 30)
(30, 40)

35. Struct BTreeSet
Employee { id: 101, name: "Arun" }
Employee { id: 102, name: "Bala" }
Employee { id: 103, name: "Charan" }
Unique employees: 3

36. Remove duplicates
Unique sorted values: {10, 20, 30, 40}

37. Find common elements
Common students: {"Arun", "Bala"}

38. Find unique elements
Unique students: {"Charan", "Deepak"}

39. BTreeSet equality
Sets equal: true

40. Consume BTreeSet
Value: 10
Value: 20
Value: 30

41. IP address BTreeSet
"192.168.1.10"
"192.168.1.20"
"192.168.1.30"
Pi IP exists: true

42. Clear BTreeSet
Length: 0
Is empty: true

43. Memory size
BTreeSet<i32> descriptor: 24 bytes
i32 element size: 4 bytes

44. BTreeSet program completed successfully.

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/



