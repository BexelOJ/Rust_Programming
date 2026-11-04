fn main()
{
    //---------------------------------------------------
    // Array - Rust Data Structures
    // Date: 20261101

    //---------------------------------------------------
    // 01. Array Declaration and Initialization
    //---------------------------------------------------

    let arr1: [i32; 5] = [10, 20, 30, 40, 50];

    println!("01. Array Declaration");
    println!("arr1 = {:?}", arr1);

    //---------------------------------------------------
    // 02. Array Initialization with Same Value
    //---------------------------------------------------

    let arr2: [i32; 5] = [100; 5];

    println!("\n02. Same Value Initialization");
    println!("arr2 = {:?}", arr2);

    //---------------------------------------------------
    // 03. Mutable Array
    //---------------------------------------------------

    let mut arr3: [i32; 5] = [1, 2, 3, 4, 5];

    arr3[0] = 100;
    arr3[2] = 300;

    println!("\n03. Mutable Array");
    println!("arr3 = {:?}", arr3);

    //---------------------------------------------------
    // 04. Accessing Array Elements
    //---------------------------------------------------

    println!("\n04. Accessing Elements");

    println!("Index 0 = {}", arr1[0]);
    println!("Index 1 = {}", arr1[1]);
    println!("Index 2 = {}", arr1[2]);
    println!("Index 3 = {}", arr1[3]);
    println!("Index 4 = {}", arr1[4]);

    //---------------------------------------------------
    // 05. Array Length
    //---------------------------------------------------

    println!("\n05. Array Length");

    println!("Length = {}", arr1.len());

    //---------------------------------------------------
    // 06. Array Iteration Using for Loop
    //---------------------------------------------------

    println!("\n06. Iteration Using for Loop");

    for i in 0..arr1.len()
    {
        println!("arr1[{}] = {}", i, arr1[i]);
    }

    //---------------------------------------------------
    // 07. Iteration Using IntoIterator
    //---------------------------------------------------

    println!("\n07. Iteration Using for-in");

    for value in arr1
    {
        println!("Value = {}", value);
    }

    //---------------------------------------------------
    // 08. Iteration Using iter()
    //---------------------------------------------------

    println!("\n08. Iteration Using iter()");

    for value in arr1.iter()
    {
        println!("Value = {}", value);
    }

    //---------------------------------------------------
    // 09. Iteration Using iter_mut()
    //---------------------------------------------------

    println!("\n09. Iteration Using iter_mut()");

    let mut arr4: [i32; 5] = [10, 20, 30, 40, 50];

    for value in arr4.iter_mut()
    {
        *value += 10;
    }

    println!("Modified arr4 = {:?}", arr4);

    //---------------------------------------------------
    // 10. Array Copy
    //---------------------------------------------------

    println!("\n10. Array Copy");

    let arr5 = [1, 2, 3, 4, 5];

    let arr6 = arr5;

    println!("arr5 = {:?}", arr5);
    println!("arr6 = {:?}", arr6);

    //---------------------------------------------------
    // 11. Array Clone
    //---------------------------------------------------

    println!("\n11. Array Clone");

    let arr7 = [10, 20, 30, 40, 50];

    let arr8 = arr7.clone();

    println!("arr7 = {:?}", arr7);
    println!("arr8 = {:?}", arr8);

    //---------------------------------------------------
    // 12. Array Destructuring
    //---------------------------------------------------

    println!("\n12. Array Destructuring");

    let arr9 = [11, 22, 33];

    let [a, b, c] = arr9;

    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);

    //---------------------------------------------------
    // 13. Passing Array to Function
    //---------------------------------------------------

    println!("\n13. Passing Array to Function");

    let arr10 = [5, 10, 15, 20, 25];

    display_array(&arr10);

    //---------------------------------------------------
    // 14. Passing Mutable Array to Function
    //---------------------------------------------------

    println!("\n14. Mutable Array Function");

    let mut arr11 = [1, 2, 3, 4, 5];

    modify_array(&mut arr11);

    println!("Modified array = {:?}", arr11);

    //---------------------------------------------------
    // 15. Returning Array from Function
    //---------------------------------------------------

    println!("\n15. Returning Array from Function");

    let arr12 = create_array();

    println!("Returned array = {:?}", arr12);

    //---------------------------------------------------
    // 16. Multidimensional Array
    //---------------------------------------------------

    println!("\n16. Two Dimensional Array");

    let matrix: [[i32; 3]; 2] =
    [
        [1, 2, 3],
        [4, 5, 6]
    ];

    for row in 0..2
    {
        for col in 0..3
        {
            print!("{} ", matrix[row][col]);
        }

        println!();
    }

    //---------------------------------------------------
    // 17. Array Sum
    //---------------------------------------------------

    println!("\n17. Array Sum");

    let arr13 = [10, 20, 30, 40, 50];

    let sum: i32 = arr13.iter().sum();

    println!("Sum = {}", sum);

    //---------------------------------------------------
    // 18. Array Minimum and Maximum
    //---------------------------------------------------

    println!("\n18. Minimum and Maximum");

    println!("Minimum = {:?}", arr13.iter().min());
    println!("Maximum = {:?}", arr13.iter().max());

    //---------------------------------------------------
    // 19. Array Contains
    //---------------------------------------------------

    println!("\n19. Contains");

    println!("Contains 30 = {}", arr13.contains(&30));
    println!("Contains 99 = {}", arr13.contains(&99));

    //---------------------------------------------------
    // 20. Array Reverse
    //---------------------------------------------------

    println!("\n20. Reverse");

    let mut arr14 = [1, 2, 3, 4, 5];

    arr14.reverse();

    println!("Reversed array = {:?}", arr14);

    //---------------------------------------------------
    // 21. Array Sorting
    //---------------------------------------------------

    println!("\n21. Sorting");

    let mut arr15 = [50, 10, 40, 20, 30];

    arr15.sort();

    println!("Sorted array = {:?}", arr15);

    //---------------------------------------------------
    // 22. Array Memory Size
    //---------------------------------------------------

    println!("\n22. Memory Size");

    println!("Array size   = {} bytes",
             std::mem::size_of_val(&arr1));

    println!("Element size = {} bytes",
             std::mem::size_of::<i32>());

    //---------------------------------------------------
    // 23. Array Swap
    //---------------------------------------------------

    println!("\n23. Swap Elements");

    let mut arr16 = [10, 20, 30, 40, 50];

    arr16.swap(0, 4);

    println!("After swap = {:?}", arr16);

    //---------------------------------------------------
    // 24. Array Fill
    //---------------------------------------------------

    println!("\n24. Fill Array");

    let mut arr17 = [0; 5];

    arr17.fill(100);

    println!("Filled array = {:?}", arr17);

    //---------------------------------------------------
    // 25. Array Split
    //---------------------------------------------------

    println!("\n25. Split Array");

    let arr18 = [10, 20, 30, 40, 50];

    let (left, right) = arr18.split_at(2);

    println!("Left  = {:?}", left);
    println!("Right = {:?}", right);

    //---------------------------------------------------
    // 26. Array First and Last
    //---------------------------------------------------

    println!("\n26. First and Last");

    println!("First = {:?}", arr18.first());
    println!("Last  = {:?}", arr18.last());

    //---------------------------------------------------
    // 27. Safe Element Access Using get()
    //---------------------------------------------------

    println!("\n27. Safe Access");

    println!("Index 2 = {:?}", arr18.get(2));
    println!("Index 8 = {:?}", arr18.get(8));

    //---------------------------------------------------
    // 28. Array Enumerate
    //---------------------------------------------------

    println!("\n28. Enumerate");

    for (index, value) in arr18.iter().enumerate()
    {
        println!("Index = {}, Value = {}", index, value);
    }
}

//---------------------------------------------------
// 29. Function to Display Array
//---------------------------------------------------

fn display_array(arr: &[i32; 5])
{
    println!("Function received = {:?}", arr);
}

//---------------------------------------------------
// 30. Function to Modify Array
//---------------------------------------------------

fn modify_array(arr: &mut [i32; 5])
{
    for value in arr.iter_mut()
    {
        *value *= 2;
    }
}

//---------------------------------------------------
// 31. Function to Return Array
//---------------------------------------------------

fn create_array() -> [i32; 5]
{
    [100, 200, 300, 400, 500]
}



/*
//---------------------------------------------------
Important Rust concepts demonstrated:

Concept - Rust syntax
Fixed-size array - [i32; 5]
Mutable array - let mut
Length - .len()
Immutable iteration - .iter()
Mutable iteration - .iter_mut()
Copy - let b = a
Clone\ - .clone()
Safe access - .get()
Reverse - .reverse()
Sorting - .sort()
Split - .split_at()
Borrow array - &[i32; 5]
Mutable borrow - &mut [i32; 5]

//---------------------------------------------------


//---------------------------------------------------
Compile and execute:

rustc rust_20261101_ds_Array_01.rs -o array

./array

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

01. Array Declaration
arr1 = [10, 20, 30, 40, 50]

02. Same Value Initialization
arr2 = [100, 100, 100, 100, 100]

03. Mutable Array
arr3 = [100, 2, 300, 4, 5]

04. Accessing Elements
Index 0 = 10
Index 1 = 20
Index 2 = 30
Index 3 = 40
Index 4 = 50

05. Array Length
Length = 5

06. Iteration Using for Loop
arr1[0] = 10
arr1[1] = 20
arr1[2] = 30
arr1[3] = 40
arr1[4] = 50

07. Iteration Using for-in
Value = 10
Value = 20
Value = 30
Value = 40
Value = 50

08. Iteration Using iter()
Value = 10
Value = 20
Value = 30
Value = 40
Value = 50

09. Iteration Using iter_mut()
Modified arr4 = [20, 30, 40, 50, 60]

10. Array Copy
arr5 = [1, 2, 3, 4, 5]
arr6 = [1, 2, 3, 4, 5]

11. Array Clone
arr7 = [10, 20, 30, 40, 50]
arr8 = [10, 20, 30, 40, 50]

12. Array Destructuring
a = 11
b = 22
c = 33

13. Passing Array to Function
Function received = [5, 10, 15, 20, 25]

14. Mutable Array Function
Modified array = [2, 4, 6, 8, 10]

15. Returning Array from Function
Returned array = [100, 200, 300, 400, 500]

16. Two Dimensional Array
1 2 3
4 5 6

17. Array Sum
Sum = 150

18. Minimum and Maximum
Minimum = Some(10)
Maximum = Some(50)

19. Contains
Contains 30 = true
Contains 99 = false

20. Reverse
Reversed array = [5, 4, 3, 2, 1]

21. Sorting
Sorted array = [10, 20, 30, 40, 50]

22. Memory Size
Array size   = 20 bytes
Element size = 4 bytes

23. Swap Elements
After swap = [50, 20, 30, 40, 10]

24. Fill Array
Filled array = [100, 100, 100, 100, 100]

25. Split Array
Left  = [10, 20]
Right = [30, 40, 50]

26. First and Last
First = Some(10)
Last  = Some(50)

27. Safe Access
Index 2 = Some(30)
Index 8 = None

28. Enumerate
Index = 0, Value = 10
Index = 1, Value = 20
Index = 2, Value = 30
Index = 3, Value = 40
Index = 4, Value = 50

//---------------------------------------------------

//---------------------------------------------------

//---------------------------------------------------
*/


