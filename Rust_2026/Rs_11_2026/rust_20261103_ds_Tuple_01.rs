fn main()
{
    //---------------------------------------------------
    // Tuple - Rust Data Structures
    // Date: 20261103

    //---------------------------------------------------
    // 01. Tuple Declaration and Initialization
    //---------------------------------------------------

    let tuple1: (i32, f64, char) = (100, 25.5, 'A');

    println!("01. Tuple Declaration");
    println!("tuple1 = {:?}", tuple1);

    //---------------------------------------------------
    // 02. Tuple Without Explicit Data Types
    //---------------------------------------------------

    let tuple2 = (10, 20.5, 'B', true);

    println!("\n02. Type Inference");
    println!("tuple2 = {:?}", tuple2);

    //---------------------------------------------------
    // 03. Accessing Tuple Elements
    //---------------------------------------------------

    println!("\n03. Accessing Elements");

    println!("Element 0 = {}", tuple1.0);
    println!("Element 1 = {}", tuple1.1);
    println!("Element 2 = {}", tuple1.2);

    //---------------------------------------------------
    // 04. Mutable Tuple
    //---------------------------------------------------

    println!("\n04. Mutable Tuple");

    let mut tuple3 = (10, 20.5, 'C');

    tuple3.0 = 100;
    tuple3.1 = 50.5;
    tuple3.2 = 'D';

    println!("Modified Tuple = {:?}", tuple3);

    //---------------------------------------------------
    // 05. Tuple Destructuring
    //---------------------------------------------------

    println!("\n05. Tuple Destructuring");

    let tuple4 = (10, 20.5, 'E');

    let (a, b, c) = tuple4;

    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);

    //---------------------------------------------------
    // 06. Partial Destructuring
    //---------------------------------------------------

    println!("\n06. Partial Destructuring");

    let tuple5 = (100, 200, 300, 400);

    let (x, y, ..) = tuple5;

    println!("x = {}", x);
    println!("y = {}", y);

    //---------------------------------------------------
    // 07. Ignoring Elements
    //---------------------------------------------------

    println!("\n07. Ignoring Elements");

    let tuple6 = (10, 20, 30);

    let (first, _, last) = tuple6;

    println!("First = {}", first);
    println!("Last  = {}", last);

    //---------------------------------------------------
    // 08. Nested Tuple
    //---------------------------------------------------

    println!("\n08. Nested Tuple");

    let tuple7 = (100, (20, 30), 'F');

    println!("Nested Tuple = {:?}", tuple7);

    println!("First        = {}", tuple7.0);
    println!("Nested First = {}", tuple7.1.0);
    println!("Nested Last  = {}", tuple7.1.1);

    //---------------------------------------------------
    // 09. Single Element Tuple
    //---------------------------------------------------

    println!("\n09. Single Element Tuple");

    let tuple8 = (100,);

    println!("Single Tuple = {:?}", tuple8);
    println!("Value        = {}", tuple8.0);

    //---------------------------------------------------
    // 10. Unit Tuple
    //---------------------------------------------------

    println!("\n10. Unit Tuple");

    let tuple9 = ();

    println!("Unit Tuple = {:?}", tuple9);

    //---------------------------------------------------
    // 11. Tuple of Strings
    //---------------------------------------------------

    println!("\n11. Tuple of Strings");

    let tuple10 = ("Bexel", "Rust", "Data Structures");

    println!("Tuple = {:?}", tuple10);

    //---------------------------------------------------
    // 12. Mutable String Inside Tuple
    //---------------------------------------------------

    println!("\n12. Mutable String Inside Tuple");

    let mut tuple11 = (String::from("Rust"), 2026);

    tuple11.0.push_str(" Programming");
    tuple11.1 = 2027;

    println!("Tuple = {:?}", tuple11);

    //---------------------------------------------------
    // 13. Tuple Copy
    //---------------------------------------------------

    println!("\n13. Tuple Copy");

    let tuple12 = (10, 20, 30);

    let tuple13 = tuple12;

    println!("Original = {:?}", tuple12);
    println!("Copy     = {:?}", tuple13);

    //---------------------------------------------------
    // 14. Tuple Clone
    //---------------------------------------------------

    println!("\n14. Tuple Clone");

    let tuple14 = (String::from("Rust"), 100);

    let tuple15 = tuple14.clone();

    println!("Original = {:?}", tuple14);
    println!("Clone    = {:?}", tuple15);

    //---------------------------------------------------
    // 15. Tuple Comparison
    //---------------------------------------------------

    println!("\n15. Tuple Comparison");

    let tuple16 = (10, 20);
    let tuple17 = (10, 20);
    let tuple18 = (20, 10);

    println!("tuple16 == tuple17 : {}", tuple16 == tuple17);
    println!("tuple16 == tuple18 : {}", tuple16 == tuple18);

    //---------------------------------------------------
    // 16. Tuple Ordering
    //---------------------------------------------------

    println!("\n16. Tuple Ordering");

    let tuple19 = (10, 20);
    let tuple20 = (10, 30);

    println!("tuple19 < tuple20 : {}", tuple19 < tuple20);
    println!("tuple19 > tuple20 : {}", tuple19 > tuple20);

    //---------------------------------------------------
    // 17. Tuple Length Using Destructuring
    //---------------------------------------------------

    println!("\n17. Tuple Element Count");

    let tuple21 = (10, 20, 30, 40, 50);

    let (_, _, _, _, _) = tuple21;

    println!("Tuple = {:?}", tuple21);
    println!("Elements = 5");

    //---------------------------------------------------
    // 18. Passing Tuple to Function
    //---------------------------------------------------

    println!("\n18. Passing Tuple to Function");

    let tuple22 = (100, 200);

    display_tuple(tuple22);

    //---------------------------------------------------
    // 19. Passing Tuple by Reference
    //---------------------------------------------------

    println!("\n19. Tuple by Reference");

    let tuple23 = (10, 20.5, 'A');

    display_tuple_ref(&tuple23);

    //---------------------------------------------------
    // 20. Returning Tuple from Function
    //---------------------------------------------------

    println!("\n20. Returning Tuple");

    let result = create_tuple();

    println!("Returned Tuple = {:?}", result);

    //---------------------------------------------------
    // 21. Multiple Return Values
    //---------------------------------------------------

    println!("\n21. Multiple Return Values");

    let (sum, difference, product) = calculate(20, 10);

    println!("Sum        = {}", sum);
    println!("Difference = {}", difference);
    println!("Product    = {}", product);

    //---------------------------------------------------
    // 22. Tuple Inside Array
    //---------------------------------------------------

    println!("\n22. Array of Tuples");

    let arr: [(i32, char); 3] =
    [
        (1, 'A'),
        (2, 'B'),
        (3, 'C')
    ];

    println!("Array = {:?}", arr);

    for item in arr
    {
        println!("ID = {}, Grade = {}", item.0, item.1);
    }

    //---------------------------------------------------
    // 23. Tuple Inside Vector
    //---------------------------------------------------

    println!("\n23. Vector of Tuples");

    let vec = vec!
    [
        ("CPU", 4),
        ("RAM", 8),
        ("Storage", 256)
    ];

    for (name, value) in vec
    {
        println!("Name = {}, Value = {}", name, value);
    }

    //---------------------------------------------------
    // 24. Tuple Memory Size
    //---------------------------------------------------

    println!("\n24. Tuple Memory Size");

    let tuple24: (i32, f64, char) = (10, 20.5, 'A');

    println!("Tuple size = {} bytes",
             std::mem::size_of_val(&tuple24));

    println!("i32 size   = {} bytes",
             std::mem::size_of::<i32>());

    println!("f64 size   = {} bytes",
             std::mem::size_of::<f64>());

    println!("char size  = {} bytes",
             std::mem::size_of::<char>());

    //---------------------------------------------------
    // 25. Tuple with Option
    //---------------------------------------------------

    println!("\n25. Tuple with Option");

    let sensor_data: (u32, Option<f64>) = (101, Some(35.5));

    println!("Sensor ID   = {}", sensor_data.0);
    println!("Temperature = {:?}", sensor_data.1);

    //---------------------------------------------------
    // 26. Tuple with Result
    //---------------------------------------------------

    println!("\n26. Tuple with Result");

    let operation: (i32, Result<i32, &str>) = (10, Ok(100));

    println!("Input  = {}", operation.0);
    println!("Result = {:?}", operation.1);

    //---------------------------------------------------
    // 27. Tuple Array Destructuring
    //---------------------------------------------------

    println!("\n27. Tuple Array Destructuring");

    let data = [(10, 20), (30, 40), (50, 60)];

    for (first, second) in data
    {
        println!("First = {}, Second = {}", first, second);
    }
}

//---------------------------------------------------
// 28. Display Tuple Function
//---------------------------------------------------

fn display_tuple(data: (i32, i32))
{
    println!("Received Tuple = {:?}", data);
}

//---------------------------------------------------
// 29. Display Tuple Reference
//---------------------------------------------------

fn display_tuple_ref(data: &(i32, f64, char))
{
    println!("Received Tuple = {:?}", data);
}

//---------------------------------------------------
// 30. Create Tuple Function
//---------------------------------------------------

fn create_tuple() -> (i32, f64, char)
{
    (100, 200.5, 'R')
}

//---------------------------------------------------
// 31. Multiple Return Values
//---------------------------------------------------

fn calculate(a: i32, b: i32) -> (i32, i32, i32)
{
    (a + b, a - b, a * b)
}






/*
//---------------------------------------------------
Tuple versus Array:

Feature - Tuple - Array

Data types - Different - Same
Size - Fixed - Fixed
Access - .0, .1 - [0], [1]
Destructuring - Yes - Yes
Memory - Contiguous value - Contiguous elements
Typical use - Group related fields - Store sequences

//---------------------------------------------------


//---------------------------------------------------
Compile and execute:

rustc rust_20261103_ds_Tuple_01.rs -o tuple

./tuple

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

01. Tuple Declaration
tuple1 = (100, 25.5, 'A')

02. Type Inference
tuple2 = (10, 20.5, 'B', true)

03. Accessing Elements
Element 0 = 100
Element 1 = 25.5
Element 2 = A

04. Mutable Tuple
Modified Tuple = (100, 50.5, 'D')

05. Tuple Destructuring
a = 10
b = 20.5
c = E

06. Partial Destructuring
x = 100
y = 200

07. Ignoring Elements
First = 10
Last  = 30

08. Nested Tuple
Nested Tuple = (100, (20, 30), 'F')
First        = 100
Nested First = 20
Nested Last  = 30

09. Single Element Tuple
Single Tuple = (100,)
Value        = 100

10. Unit Tuple
Unit Tuple = ()

11. Tuple of Strings
Tuple = ("Bexel", "Rust", "Data Structures")

12. Mutable String Inside Tuple
Tuple = ("Rust Programming", 2027)

13. Tuple Copy
Original = (10, 20, 30)
Copy     = (10, 20, 30)

14. Tuple Clone
Original = ("Rust", 100)
Clone    = ("Rust", 100)

15. Tuple Comparison
tuple16 == tuple17 : true
tuple16 == tuple18 : false

16. Tuple Ordering
tuple19 < tuple20 : true
tuple19 > tuple20 : false

17. Tuple Element Count
Tuple = (10, 20, 30, 40, 50)
Elements = 5

18. Passing Tuple to Function
Received Tuple = (100, 200)

19. Tuple by Reference
Received Tuple = (10, 20.5, 'A')

20. Returning Tuple
Returned Tuple = (100, 200.5, 'R')

21. Multiple Return Values
Sum        = 30
Difference = 10
Product    = 200

22. Array of Tuples
Array = [(1, 'A'), (2, 'B'), (3, 'C')]
ID = 1, Grade = A
ID = 2, Grade = B
ID = 3, Grade = C

23. Vector of Tuples
Name = CPU, Value = 4
Name = RAM, Value = 8
Name = Storage, Value = 256

24. Tuple Memory Size
Tuple size = 16 bytes
i32 size   = 4 bytes
f64 size   = 8 bytes
char size  = 4 bytes

25. Tuple with Option
Sensor ID   = 101
Temperature = Some(35.5)

26. Tuple with Result
Input  = 10
Result = Ok(100)

27. Tuple Array Destructuring
First = 10, Second = 20
First = 30, Second = 40
First = 50, Second = 60

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/


