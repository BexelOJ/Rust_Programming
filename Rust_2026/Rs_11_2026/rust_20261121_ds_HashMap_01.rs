//---------------------------------------------------
// HashMap - Rust Data Structures
// Date: 20261121
//---------------------------------------------------


use std::collections::HashMap;
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
// 02. Display a HashMap
//--------------------------------------------------

fn display_map<K, V>(map: &HashMap<K, V>)
where
    K: std::fmt::Debug + Eq + std::hash::Hash,
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
    // 04. Create an empty HashMap
    //--------------------------------------------------

    println!("\n04. Create HashMap");

    let mut map: HashMap<i32, String> = HashMap::new();

    println!("Is empty: {}", map.is_empty());
    println!("Length  : {}", map.len());

    //--------------------------------------------------
    // 05. Insert key-value pairs
    //--------------------------------------------------

    println!("\n05. Insert elements");

    map.insert(101, String::from("Arun"));
    map.insert(102, String::from("Bala"));
    map.insert(103, String::from("Charan"));

    display_map(&map);

    //--------------------------------------------------
    // 06. Access value using get()
    //--------------------------------------------------

    println!("\n06. Access value");

    match map.get(&101) {
        Some(name) => println!("Employee: {}", name),
        None => println!("Employee not found"),
    }

    //--------------------------------------------------
    // 07. Access using indexing
    //--------------------------------------------------

    println!("\n07. Access using index");

    println!("Employee 102: {}", map[&102]);

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

    println!("\n12. Iterate over HashMap");

    for (key, value) in &map {
        println!("ID: {}, Name: {}", key, value);
    }

    // HashMap iteration order is not guaranteed.

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
    // 15. Mutable iteration over values
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

    map.entry(104)
        .or_insert(String::from("Deepak"));

    map.entry(104)
        .or_insert(String::from("Another Name"));

    display_map(&map);

    // The second insertion does not replace Deepak.

    //--------------------------------------------------
    // 17. Update using entry()
    //--------------------------------------------------

    println!("\n17. entry().and_modify()");

    map.entry(104)
        .and_modify(|name| name.push_str(" Updated"))
        .or_insert(String::from("New Employee"));

    display_map(&map);

    //--------------------------------------------------
    // 18. Count word frequency
    //--------------------------------------------------

    println!("\n18. Word frequency");

    let sentence = "rust is fast rust is safe rust is powerful";

    let mut frequency: HashMap<&str, u32> = HashMap::new();

    for word in sentence.split_whitespace() {
        let count = frequency.entry(word).or_insert(0);
        *count += 1;
    }

    display_map(&frequency);

    //--------------------------------------------------
    // 19. Count character frequency
    //--------------------------------------------------

    println!("\n19. Character frequency");

    let text = "programming";

    let mut char_frequency: HashMap<char, u32> = HashMap::new();

    for ch in text.chars() {
        *char_frequency.entry(ch).or_insert(0) += 1;
    }

    display_map(&char_frequency);

    //--------------------------------------------------
    // 20. HashMap with integer keys and values
    //--------------------------------------------------

    println!("\n20. Integer HashMap");

    let mut numbers: HashMap<i32, i32> = HashMap::new();

    numbers.insert(1, 100);
    numbers.insert(2, 200);
    numbers.insert(3, 300);

    display_map(&numbers);

    //--------------------------------------------------
    // 21. HashMap with String keys and values
    //--------------------------------------------------

    println!("\n21. String HashMap");

    let mut countries: HashMap<String, String> = HashMap::new();

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
    // 22. HashMap with tuple values
    //--------------------------------------------------

    println!("\n22. Tuple values");

    let mut coordinates: HashMap<String, (i32, i32)> =
        HashMap::new();

    coordinates.insert(String::from("Point A"), (10, 20));
    coordinates.insert(String::from("Point B"), (30, 40));

    display_map(&coordinates);

    //--------------------------------------------------
    // 23. HashMap with struct values
    //--------------------------------------------------

    println!("\n23. Struct values");

    let mut employees: HashMap<u32, Employee> = HashMap::new();

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
    // 25. Clone a HashMap
    //--------------------------------------------------

    println!("\n25. Clone HashMap");

    let cloned_map = map.clone();

    println!("Original:");
    display_map(&map);

    println!("Cloned:");
    display_map(&cloned_map);

    //--------------------------------------------------
    // 26. Extend HashMap
    //--------------------------------------------------

    println!("\n26. Extend HashMap");

    let mut first: HashMap<i32, &str> = HashMap::new();

    first.insert(1, "One");
    first.insert(2, "Two");

    let second = HashMap::from([
        (3, "Three"),
        (4, "Four"),
    ]);

    first.extend(second);

    display_map(&first);

    //--------------------------------------------------
    // 27. Build HashMap from an array
    //--------------------------------------------------

    println!("\n27. HashMap from array");

    let array_map: HashMap<i32, &str> = HashMap::from([
        (10, "Ten"),
        (20, "Twenty"),
        (30, "Thirty"),
    ]);

    display_map(&array_map);

    //--------------------------------------------------
    // 28. Reserve capacity
    //--------------------------------------------------

    println!("\n28. Reserve capacity");

    let mut reserve_map: HashMap<i32, i32> = HashMap::new();

    reserve_map.reserve(100);

    println!("Capacity: {}", reserve_map.capacity());

    //--------------------------------------------------
    // 29. Shrink capacity
    //--------------------------------------------------

    println!("\n29. Shrink capacity");

    reserve_map.shrink_to_fit();

    println!("Capacity: {}", reserve_map.capacity());

    //--------------------------------------------------
    // 30. Clear HashMap
    //--------------------------------------------------

    println!("\n30. Clear HashMap");

    reserve_map.insert(1, 100);
    reserve_map.insert(2, 200);

    reserve_map.clear();

    println!("Length: {}", reserve_map.len());
    println!("Empty : {}", reserve_map.is_empty());

    //--------------------------------------------------
    // 31. Remove an entry using entry()
    //--------------------------------------------------

    println!("\n31. Remove using retain()");

    let mut retain_map = HashMap::from([
        (1, 10),
        (2, 20),
        (3, 30),
        (4, 40),
    ]);

    retain_map.retain(|key, _value| *key % 2 == 0);

    display_map(&retain_map);

    //--------------------------------------------------
    // 32. Consume HashMap using into_iter()
    //--------------------------------------------------

    println!("\n32. Consume HashMap");

    let consume_map = HashMap::from([
        (1, "Apple"),
        (2, "Banana"),
        (3, "Orange"),
    ]);

    for (key, value) in consume_map {
        println!("{} => {}", key, value);
    }

    // consume_map is no longer available after this loop.

    //--------------------------------------------------
    // 33. HashMap with vector values
    //--------------------------------------------------

    println!("\n33. Vector values");

    let mut subjects: HashMap<String, Vec<&str>> = HashMap::new();

    subjects.insert(
        String::from("Arun"),
        vec!["C", "Rust", "Linux"],
    );

    subjects.insert(
        String::from("Bala"),
        vec!["C++", "Python", "Networking"],
    );

    display_map(&subjects);

    //--------------------------------------------------
    // 34. HashMap as a simple cache
    //--------------------------------------------------

    println!("\n34. Simple cache");

    let mut cache: HashMap<String, i32> = HashMap::new();

    cache.insert(String::from("temperature"), 28);
    cache.insert(String::from("humidity"), 65);

    match cache.get("temperature") {
        Some(value) => println!("Cached temperature: {}", value),
        None => println!("Cache miss"),
    }

    //--------------------------------------------------
    // 35. HashMap for IP address lookup
    //--------------------------------------------------

    println!("\n35. IP address lookup");

    let mut ip_table: HashMap<&str, &str> = HashMap::new();

    ip_table.insert("192.168.1.10", "Server");
    ip_table.insert("192.168.1.20", "Desktop");
    ip_table.insert("192.168.1.30", "Raspberry Pi");

    match ip_table.get("192.168.1.30") {
        Some(device) => println!("Device: {}", device),
        None => println!("Unknown device"),
    }

    //--------------------------------------------------
    // 36. HashMap with boolean values
    //--------------------------------------------------

    println!("\n36. Boolean values");

    let mut status: HashMap<&str, bool> = HashMap::new();

    status.insert("Network", true);
    status.insert("Database", false);
    status.insert("MQTT", true);

    display_map(&status);

    //--------------------------------------------------
    // 37. HashMap equality
    //--------------------------------------------------

    println!("\n37. HashMap equality");

    let map1 = HashMap::from([
        (1, 100),
        (2, 200),
    ]);

    let map2 = HashMap::from([
        (2, 200),
        (1, 100),
    ]);

    println!("Maps equal: {}", map1 == map2);

    // HashMap equality does not depend on iteration order.

    //--------------------------------------------------
    // 38. Default value with unwrap_or()
    //--------------------------------------------------

    println!("\n38. Default value");

    let value = map.get(&999).map(String::as_str).unwrap_or("Unknown");

    println!("Lookup result: {}", value);

    //--------------------------------------------------
    // 39. HashMap memory descriptor size
    //--------------------------------------------------

    println!("\n39. Memory size");

    println!(
        "HashMap<i32, i32> descriptor: {} bytes",
        size_of::<HashMap<i32, i32>>()
    );

    println!("i32 size: {} bytes", size_of::<i32>());

    // Descriptor size excludes dynamically allocated table storage.

    //--------------------------------------------------
    // 40. Final status
    //--------------------------------------------------

    println!("\n40. HashMap program completed successfully.");
}


/*
//---------------------------------------------------
A HashMap generally works through these steps:

1. You provide a key.
2. A hash function calculates a hash value.
3. The hash value determines the bucket location.
4. The value is stored or retrieved from that bucket.
5. If two keys map to the same bucket, the HashMap handles the collision internally.

//---------------------------------------------------
Important concepts:

Concept	Explanation

Key	Unique identifier
Value	Data associated with a key
Hash function	Converts key into a hash
Bucket	Storage location
Collision	Different keys map to the same bucket
Capacity	Number of elements the table can hold without reallocating
entry()	Convenient API for conditional insertion and modification

//---------------------------------------------------
Compile and execute:

rustc --edition=2021 rust_20261121_ds_HashMap_01.rs -o hashmap_example

./hashmap_example

//---------------------------------------------------
Time complexity:

Operation	Average	Worst case

Insert	O(1)	O(n)
Search	O(1)	O(n)
Remove	O(1)	O(n)
Iterate	O(capacity)	—

//---------------------------------------------------
Output:
//-------------------------

04. Create HashMap
Is empty: true
Length  : 0

05. Insert elements
102 => "Bala"
101 => "Arun"
103 => "Charan"

06. Access value
Employee: Arun

07. Access using index
Employee 102: Bala

08. Update value
Updated: Some("Arun Kumar")

09. contains_key()
Key 101 exists: true
Key 999 exists: false

10. Remove element
Removed: Bala
101 => "Arun Kumar"
103 => "Charan"

11. Length and empty status
Length  : 2
Is empty: false

12. Iterate over HashMap
ID: 101, Name: Arun Kumar
ID: 103, Name: Charan

13. Iterate over keys
Key: 101
Key: 103

14. Iterate over values
Value: Arun Kumar
Value: Charan

15. Modify values
101 => "Arun Kumar - Employee"
103 => "Charan - Employee"

16. entry().or_insert()
104 => "Deepak"
101 => "Arun Kumar - Employee"
103 => "Charan - Employee"

17. entry().and_modify()
104 => "Deepak Updated"
101 => "Arun Kumar - Employee"
103 => "Charan - Employee"

18. Word frequency
"fast" => 1
"is" => 3
"rust" => 3
"safe" => 1
"powerful" => 1

19. Character frequency
'a' => 1
'i' => 1
'r' => 2
'm' => 2
'o' => 1
'n' => 1
'g' => 2
'p' => 1

20. Integer HashMap
1 => 100
2 => 200
3 => 300

21. String HashMap
"India" => "New Delhi"
"France" => "Paris"
"Japan" => "Tokyo"

22. Tuple values
"Point B" => (30, 40)
"Point A" => (10, 20)

23. Struct values
Key: 101, ID: 101, Name: Arun, Salary: 50000
Key: 102, ID: 102, Name: Bala, Salary: 60000

24. Modify using get_mut()
Updated salary: 55000

25. Clone HashMap
Original:
104 => "Deepak Updated"
101 => "Arun Kumar - Employee"
103 => "Charan - Employee"
Cloned:
104 => "Deepak Updated"
101 => "Arun Kumar - Employee"
103 => "Charan - Employee"

26. Extend HashMap
3 => "Three"
2 => "Two"
1 => "One"
4 => "Four"

27. HashMap from array
30 => "Thirty"
10 => "Ten"
20 => "Twenty"

28. Reserve capacity
Capacity: 112

29. Shrink capacity
Capacity: 0

30. Clear HashMap
Length: 0
Empty : true

31. Remove using retain()
2 => 20
4 => 40

32. Consume HashMap
2 => Banana
3 => Orange
1 => Apple

33. Vector values
"Arun" => ["C", "Rust", "Linux"]
"Bala" => ["C++", "Python", "Networking"]

34. Simple cache
Cached temperature: 28

35. IP address lookup
Device: Raspberry Pi

36. Boolean values
"Database" => false
"Network" => true
"MQTT" => true

37. HashMap equality
Maps equal: true

38. Default value
Lookup result: Unknown

39. Memory size
HashMap<i32, i32> descriptor: 48 bytes
i32 size: 4 bytes

40. HashMap program completed successfully.

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/
