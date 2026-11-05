//---------------------------------------------------
// BTreeMap - Rust Data Structures
// Date: 20261123
//---------------------------------------------------

use std::collections::BTreeMap;
use std::mem::size_of;

//--------------------------------------------------
// 01. Employee structure
//--------------------------------------------------

#[derive(Debug, Clone)]
struct Employee {
    id: u32,
    name: String,
    salary: f64,
}

//--------------------------------------------------
// 02. Display BTreeMap
//--------------------------------------------------

fn display_map<K, V>(map: &BTreeMap<K, V>)
where
    K: std::fmt::Debug + Ord,
    V: std::fmt::Debug,
{
    for (key, value) in map {
        println!("{:?} => {:?}", key, value);
    }
}

//--------------------------------------------------
// 03. Main function
//--------------------------------------------------

fn main() {

    //--------------------------------------------------
    // 04. Create an empty BTreeMap
    //--------------------------------------------------

    println!("\n04. Create BTreeMap");

    let mut map: BTreeMap<i32, String> = BTreeMap::new();

    println!("Is empty: {}", map.is_empty());
    println!("Length  : {}", map.len());

    //--------------------------------------------------
    // 05. Insert key-value pairs
    //--------------------------------------------------

    println!("\n05. Insert elements");

    map.insert(103, String::from("Charan"));
    map.insert(101, String::from("Arun"));
    map.insert(102, String::from("Bala"));
    map.insert(104, String::from("Deepak"));

    display_map(&map);

    // Keys are automatically maintained in ascending order.

    //--------------------------------------------------
    // 06. Access value using get()
    //--------------------------------------------------

    println!("\n06. Access value");

    match map.get(&102) {
        Some(name) => println!("Employee: {}", name),
        None => println!("Employee not found"),
    }

    //--------------------------------------------------
    // 07. Access using indexing
    //--------------------------------------------------

    println!("\n07. Access using index");

    println!("Employee 101: {}", map[&101]);

    // Indexing panics if the key does not exist.
    // Prefer get() when a key may be absent.

    //--------------------------------------------------
    // 08. Update an existing value
    //--------------------------------------------------

    println!("\n08. Update value");

    map.insert(101, String::from("Arun Kumar"));

    println!("Updated: {:?}", map.get(&101));

    //--------------------------------------------------
    // 09. Check whether a key exists
    //--------------------------------------------------

    println!("\n09. contains_key()");

    println!("Key 101 exists: {}", map.contains_key(&101));
    println!("Key 999 exists: {}", map.contains_key(&999));

    //--------------------------------------------------
    // 10. Remove an element
    //--------------------------------------------------

    println!("\n10. Remove element");

    match map.remove(&102) {
        Some(name) => println!("Removed: {}", name),
        None => println!("Key not found"),
    }

    display_map(&map);

    //--------------------------------------------------
    // 11. Get length and empty status
    //--------------------------------------------------

    println!("\n11. Length and empty status");

    println!("Length  : {}", map.len());
    println!("Is empty: {}", map.is_empty());

    //--------------------------------------------------
    // 12. Iterate over key-value pairs
    //--------------------------------------------------

    println!("\n12. Iterate over BTreeMap");

    for (key, value) in &map {
        println!("Key: {}, Value: {}", key, value);
    }

    //--------------------------------------------------
    // 13. Iterate over keys
    //--------------------------------------------------

    println!("\n13. Iterate over keys");

    for key in map.keys() {
        println!("Key: {}", key);
    }

    //--------------------------------------------------
    // 14. Iterate over values
    //--------------------------------------------------

    println!("\n14. Iterate over values");

    for value in map.values() {
        println!("Value: {}", value);
    }

    //--------------------------------------------------
    // 15. Modify values using values_mut()
    //--------------------------------------------------

    println!("\n15. Modify values");

    for value in map.values_mut() {
        value.push_str(" - Employee");
    }

    display_map(&map);

    //--------------------------------------------------
    // 16. Insert only if key does not exist
    //--------------------------------------------------

    println!("\n16. entry().or_insert()");

    map.entry(105)
        .or_insert(String::from("Elango"));

    map.entry(105)
        .or_insert(String::from("Another Name"));

    display_map(&map);

    //--------------------------------------------------
    // 17. Modify using entry()
    //--------------------------------------------------

    println!("\n17. entry().and_modify()");

    map.entry(105)
        .and_modify(|name| name.push_str(" Updated"))
        .or_insert(String::from("New Employee"));

    display_map(&map);

    //--------------------------------------------------
    // 18. Count word frequency
    //--------------------------------------------------

    println!("\n18. Word frequency");

    let sentence = "rust is fast rust is safe rust is powerful";

    let mut frequency: BTreeMap<&str, u32> = BTreeMap::new();

    for word in sentence.split_whitespace() {
        *frequency.entry(word).or_insert(0) += 1;
    }

    display_map(&frequency);

    // Words appear in alphabetical order.

    //--------------------------------------------------
    // 19. Count character frequency
    //--------------------------------------------------

    println!("\n19. Character frequency");

    let text = "programming";

    let mut char_frequency: BTreeMap<char, u32> = BTreeMap::new();

    for ch in text.chars() {
        *char_frequency.entry(ch).or_insert(0) += 1;
    }

    display_map(&char_frequency);

    //--------------------------------------------------
    // 20. BTreeMap with integer keys and values
    //--------------------------------------------------

    println!("\n20. Integer BTreeMap");

    let mut numbers: BTreeMap<i32, i32> = BTreeMap::new();

    numbers.insert(3, 300);
    numbers.insert(1, 100);
    numbers.insert(2, 200);

    display_map(&numbers);

    //--------------------------------------------------
    // 21. BTreeMap with String keys and values
    //--------------------------------------------------

    println!("\n21. String BTreeMap");

    let mut countries: BTreeMap<String, String> = BTreeMap::new();

    countries.insert(
        String::from("India"),
        String::from("New Delhi"),
    );

    countries.insert(
        String::from("Japan"),
        String::from("Tokyo"),
    );

    countries.insert(
        String::from("France"),
        String::from("Paris"),
    );

    display_map(&countries);

    //--------------------------------------------------
    // 22. BTreeMap with tuple values
    //--------------------------------------------------

    println!("\n22. Tuple values");

    let mut coordinates: BTreeMap<String, (i32, i32)> =
        BTreeMap::new();

    coordinates.insert(String::from("Point C"), (30, 40));
    coordinates.insert(String::from("Point A"), (10, 20));
    coordinates.insert(String::from("Point B"), (20, 30));

    display_map(&coordinates);

    //--------------------------------------------------
    // 23. BTreeMap with struct values
    //--------------------------------------------------

    println!("\n23. Struct values");

    let mut employees: BTreeMap<u32, Employee> = BTreeMap::new();

    employees.insert(
        103,
        Employee {
            id: 103,
            name: String::from("Charan"),
            salary: 70000.0,
        },
    );

    employees.insert(
        101,
        Employee {
            id: 101,
            name: String::from("Arun"),
            salary: 50000.0,
        },
    );

    employees.insert(
        102,
        Employee {
            id: 102,
            name: String::from("Bala"),
            salary: 60000.0,
        },
    );

    for (id, employee) in &employees {
        println!(
            "Key: {}, ID: {}, Name: {}, Salary: {}",
            id,
            employee.id,
            employee.name,
            employee.salary
        );
    }

    //--------------------------------------------------
    // 24. Modify a value using get_mut()
    //--------------------------------------------------

    println!("\n24. Modify using get_mut()");

    if let Some(employee) = employees.get_mut(&101) {
        employee.salary += 5000.0;
    }

    println!(
        "Updated salary: {}",
        employees.get(&101).unwrap().salary
    );

    //--------------------------------------------------
    // 25. Clone BTreeMap
    //--------------------------------------------------

    println!("\n25. Clone BTreeMap");

    let cloned_map = map.clone();

    println!("Original:");
    display_map(&map);

    println!("Cloned:");
    display_map(&cloned_map);

    //--------------------------------------------------
    // 26. Extend BTreeMap
    //--------------------------------------------------

    println!("\n26. Extend BTreeMap");

    let mut first: BTreeMap<i32, &str> = BTreeMap::new();

    first.insert(1, "One");
    first.insert(2, "Two");

    let second = BTreeMap::from([
        (3, "Three"),
        (4, "Four"),
    ]);

    first.extend(second);

    display_map(&first);

    //--------------------------------------------------
    // 27. Create BTreeMap from an array
    //--------------------------------------------------

    println!("\n27. BTreeMap from array");

    let array_map: BTreeMap<i32, &str> = BTreeMap::from([
        (30, "Thirty"),
        (10, "Ten"),
        (20, "Twenty"),
    ]);

    display_map(&array_map);

    //--------------------------------------------------
    // 28. Get first key-value pair
    //--------------------------------------------------

    println!("\n28. First entry");

    match map.first_key_value() {
        Some((key, value)) => {
            println!("First: {} => {}", key, value);
        }
        None => println!("Map is empty"),
    }

    //--------------------------------------------------
    // 29. Get last key-value pair
    //--------------------------------------------------

    println!("\n29. Last entry");

    match map.last_key_value() {
        Some((key, value)) => {
            println!("Last: {} => {}", key, value);
        }
        None => println!("Map is empty"),
    }

    //--------------------------------------------------
    // 30. Remove first entry
    //--------------------------------------------------

    println!("\n30. Pop first entry");

    let mut pop_map = BTreeMap::from([
        (30, "Thirty"),
        (10, "Ten"),
        (20, "Twenty"),
    ]);

    match pop_map.pop_first() {
        Some((key, value)) => {
            println!("Removed: {} => {}", key, value);
        }
        None => println!("Map is empty"),
    }

    display_map(&pop_map);

    //--------------------------------------------------
    // 31. Remove last entry
    //--------------------------------------------------

    println!("\n31. Pop last entry");

    match pop_map.pop_last() {
        Some((key, value)) => {
            println!("Removed: {} => {}", key, value);
        }
        None => println!("Map is empty"),
    }

    display_map(&pop_map);

    //--------------------------------------------------
    // 32. Range-based lookup
    //--------------------------------------------------

    println!("\n32. Range lookup");

    let range_map = BTreeMap::from([
        (10, "Ten"),
        (20, "Twenty"),
        (30, "Thirty"),
        (40, "Forty"),
        (50, "Fifty"),
        (60, "Sixty"),
    ]);

    for (key, value) in range_map.range(20..=50) {
        println!("{} => {}", key, value);
    }

    //--------------------------------------------------
    // 33. Find keys below a value
    //--------------------------------------------------

    println!("\n33. Range below a value");

    for (key, value) in range_map.range(..30) {
        println!("{} => {}", key, value);
    }

    //--------------------------------------------------
    // 34. Find keys above a value
    //--------------------------------------------------

    println!("\n34. Range above a value");

    for (key, value) in range_map.range(30..) {
        println!("{} => {}", key, value);
    }

    //--------------------------------------------------
    // 35. Reverse iteration
    //--------------------------------------------------

    println!("\n35. Reverse iteration");

    for (key, value) in range_map.iter().rev() {
        println!("{} => {}", key, value);
    }

    //--------------------------------------------------
    // 36. BTreeMap with IP addresses
    //--------------------------------------------------

    println!("\n36. IP address mapping");

    let mut ip_table: BTreeMap<String, String> = BTreeMap::new();

    ip_table.insert(
        String::from("192.168.1.30"),
        String::from("Raspberry Pi"),
    );

    ip_table.insert(
        String::from("192.168.1.10"),
        String::from("Server"),
    );

    ip_table.insert(
        String::from("192.168.1.20"),
        String::from("Desktop"),
    );

    display_map(&ip_table);

    //--------------------------------------------------
    // 37. Retain selected entries
    //--------------------------------------------------

    println!("\n37. Retain");

    let mut retain_map = BTreeMap::from([
        (1, 10),
        (2, 20),
        (3, 30),
        (4, 40),
        (5, 50),
    ]);

    retain_map.retain(|key, _value| *key % 2 == 0);

    display_map(&retain_map);

    //--------------------------------------------------
    // 38. Consume BTreeMap using into_iter()
    //--------------------------------------------------

    println!("\n38. Consume BTreeMap");

    let consume_map = BTreeMap::from([
        (3, "Three"),
        (1, "One"),
        (2, "Two"),
    ]);

    for (key, value) in consume_map {
        println!("{} => {}", key, value);
    }

    //--------------------------------------------------
    // 39. BTreeMap equality
    //--------------------------------------------------

    println!("\n39. BTreeMap equality");

    let map1 = BTreeMap::from([
        (1, 100),
        (2, 200),
    ]);

    let map2 = BTreeMap::from([
        (2, 200),
        (1, 100),
    ]);

    println!("Maps equal: {}", map1 == map2);

    //--------------------------------------------------
    // 40. Default value using unwrap_or()
    //--------------------------------------------------

    println!("\n40. Default value");

    let value = map.get(&999).map(String::as_str).unwrap_or("Unknown");

    println!("Lookup result: {}", value);

    //--------------------------------------------------
    // 41. BTreeMap memory descriptor size
    //--------------------------------------------------

    println!("\n41. Memory size");

    println!(
        "BTreeMap<i32, i32> descriptor: {} bytes",
        size_of::<BTreeMap<i32, i32>>()
    );

    println!("i32 size: {} bytes", size_of::<i32>());

    // Descriptor size excludes allocated tree nodes.

    //--------------------------------------------------
    // 42. Clear BTreeMap
    //--------------------------------------------------

    println!("\n42. Clear BTreeMap");

    let mut clear_map = BTreeMap::from([
        (1, "One"),
        (2, "Two"),
    ]);

    clear_map.clear();

    println!("Length: {}", clear_map.len());
    println!("Is empty: {}", clear_map.is_empty());

    //--------------------------------------------------
    // 43. Final status
    //--------------------------------------------------

    println!("\n43. BTreeMap program completed successfully.");
}


/*
//---------------------------------------------------
Important BTreeMap methods

Method	Purpose

new()	
- Create map

insert()
- Insert/update pair

get()
- Retrieve value

get_mut()
- Mutable access

contains_key()
- Check key

remove()
- Remove key

keys()
- Iterate keys

values()
- Iterate values

entry()
- Conditional insertion/update

first_key_value()
- Smallest key

last_key_value()
- Largest key

pop_first()
- Remove smallest key

pop_last()
- Remove largest key

range()
- Retrieve a key range

retain()
- Keep matching entries

clear()
- Remove everything

//---------------------------------------------------
HashMap vs BTreeMap

Feature - HashMap - BTreeMap

Internal structure	Hash table	B-tree
Key order	Unspecified	Sorted
Search	Average O(1)	O(log n)
Insert	Average O(1)	O(log n)
Remove	Average O(1)	O(log n)
Range queries	Not directly supported	Supported
Key requirement	Eq + Hash	Ord

//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261123_ds_BTreeMap_01.rs -o btreemap_example

./btreemap_example

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

04. Create BTreeMap
Is empty: true
Length  : 0

05. Insert elements
101 => "Arun"
102 => "Bala"
103 => "Charan"
104 => "Deepak"

06. Access value
Employee: Bala

07. Access using index
Employee 101: Arun

08. Update value
Updated: Some("Arun Kumar")

09. contains_key()
Key 101 exists: true
Key 999 exists: false

10. Remove element
Removed: Bala
101 => "Arun Kumar"
103 => "Charan"
104 => "Deepak"

11. Length and empty status
Length  : 3
Is empty: false

12. Iterate over BTreeMap
Key: 101, Value: Arun Kumar
Key: 103, Value: Charan
Key: 104, Value: Deepak

13. Iterate over keys
Key: 101
Key: 103
Key: 104

14. Iterate over values
Value: Arun Kumar
Value: Charan
Value: Deepak

15. Modify values
101 => "Arun Kumar - Employee"
103 => "Charan - Employee"
104 => "Deepak - Employee"

16. entry().or_insert()
101 => "Arun Kumar - Employee"
103 => "Charan - Employee"
104 => "Deepak - Employee"
105 => "Elango"

17. entry().and_modify()
101 => "Arun Kumar - Employee"
103 => "Charan - Employee"
104 => "Deepak - Employee"
105 => "Elango Updated"

18. Word frequency
"fast" => 1
"is" => 3
"powerful" => 1
"rust" => 3
"safe" => 1

19. Character frequency
'a' => 1
'g' => 2
'i' => 1
'm' => 2
'n' => 1
'o' => 1
'p' => 1
'r' => 2

20. Integer BTreeMap
1 => 100
2 => 200
3 => 300

21. String BTreeMap
"France" => "Paris"
"India" => "New Delhi"
"Japan" => "Tokyo"

22. Tuple values
"Point A" => (10, 20)
"Point B" => (20, 30)
"Point C" => (30, 40)

23. Struct values
Key: 101, ID: 101, Name: Arun, Salary: 50000
Key: 102, ID: 102, Name: Bala, Salary: 60000
Key: 103, ID: 103, Name: Charan, Salary: 70000

24. Modify using get_mut()
Updated salary: 55000

25. Clone BTreeMap
Original:
101 => "Arun Kumar - Employee"
103 => "Charan - Employee"
104 => "Deepak - Employee"
105 => "Elango Updated"
Cloned:
101 => "Arun Kumar - Employee"
103 => "Charan - Employee"
104 => "Deepak - Employee"
105 => "Elango Updated"

26. Extend BTreeMap
1 => "One"
2 => "Two"
3 => "Three"
4 => "Four"

27. BTreeMap from array
10 => "Ten"
20 => "Twenty"
30 => "Thirty"

28. First entry
First: 101 => Arun Kumar - Employee

29. Last entry
Last: 105 => Elango Updated

30. Pop first entry
Removed: 10 => Ten
20 => "Twenty"
30 => "Thirty"

31. Pop last entry
Removed: 30 => Thirty
20 => "Twenty"

32. Range lookup
20 => Twenty
30 => Thirty
40 => Forty
50 => Fifty

33. Range below a value
10 => Ten
20 => Twenty

34. Range above a value
30 => Thirty
40 => Forty
50 => Fifty
60 => Sixty

35. Reverse iteration
60 => Sixty
50 => Fifty
40 => Forty
30 => Thirty
20 => Twenty
10 => Ten

36. IP address mapping
"192.168.1.10" => "Server"
"192.168.1.20" => "Desktop"
"192.168.1.30" => "Raspberry Pi"

37. Retain
2 => 20
4 => 40

38. Consume BTreeMap
1 => One
2 => Two
3 => Three

39. BTreeMap equality
Maps equal: true

40. Default value
Lookup result: Unknown

41. Memory size
BTreeMap<i32, i32> descriptor: 24 bytes
i32 size: 4 bytes

42. Clear BTreeMap
Length: 0
Is empty: true

43. BTreeMap program completed successfully.

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/



