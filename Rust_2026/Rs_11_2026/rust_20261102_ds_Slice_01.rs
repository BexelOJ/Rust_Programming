fn main()
{
    //---------------------------------------------------
    // Slice - Rust Data Structures
    // Date: 20261102

    //---------------------------------------------------
    // 01. Creating a Slice from an Array
    //---------------------------------------------------

    let arr: [i32; 5] = [10, 20, 30, 40, 50];

    let slice: &[i32] = &arr;

    println!("01. Slice from Array");
    println!("Array = {:?}", arr);
    println!("Slice = {:?}", slice);

    //---------------------------------------------------
    // 02. Creating a Partial Slice
    //---------------------------------------------------

    let slice2 = &arr[1..4];

    println!("\n02. Partial Slice");
    println!("Slice = {:?}", slice2);

    //---------------------------------------------------
    // 03. Slice Range Operations
    //---------------------------------------------------

    println!("\n03. Slice Range Operations");

    println!("arr[0..3] = {:?}", &arr[0..3]);
    println!("arr[1..4] = {:?}", &arr[1..4]);
    println!("arr[2..]  = {:?}", &arr[2..]);
    println!("arr[..3]  = {:?}", &arr[..3]);
    println!("arr[..]   = {:?}", &arr[..]);

    //---------------------------------------------------
    // 04. Slice Length
    //---------------------------------------------------

    println!("\n04. Slice Length");

    println!("Length = {}", slice.len());
    println!("Empty  = {}", slice.is_empty());

    //---------------------------------------------------
    // 05. Accessing Slice Elements
    //---------------------------------------------------

    println!("\n05. Accessing Elements");

    println!("Index 0 = {}", slice[0]);
    println!("Index 1 = {}", slice[1]);
    println!("Index 2 = {}", slice[2]);

    //---------------------------------------------------
    // 06. Safe Access Using get()
    //---------------------------------------------------

    println!("\n06. Safe Access");

    println!("Index 2 = {:?}", slice.get(2));
    println!("Index 8 = {:?}", slice.get(8));

    //---------------------------------------------------
    // 07. Iterating Through Slice
    //---------------------------------------------------

    println!("\n07. Iteration");

    for value in slice.iter()
    {
        println!("Value = {}", value);
    }

    //---------------------------------------------------
    // 08. Iteration with Index
    //---------------------------------------------------

    println!("\n08. Enumerate");

    for (index, value) in slice.iter().enumerate()
    {
        println!("Index = {}, Value = {}", index, value);
    }

    //---------------------------------------------------
    // 09. Mutable Slice
    //---------------------------------------------------

    println!("\n09. Mutable Slice");

    let mut arr2 = [10, 20, 30, 40, 50];

    {
        let slice3: &mut [i32] = &mut arr2[1..4];

        slice3[0] = 200;
        slice3[1] = 300;
        slice3[2] = 400;

        println!("Modified Slice = {:?}", slice3);
    }

    println!("Modified Array = {:?}", arr2);

    //---------------------------------------------------
    // 10. Mutable Iteration
    //---------------------------------------------------

    println!("\n10. Mutable Iteration");

    let mut arr3 = [1, 2, 3, 4, 5];

    for value in arr3.iter_mut()
    {
        *value *= 10;
    }

    println!("Modified Array = {:?}", arr3);

    //---------------------------------------------------
    // 11. Split Slice
    //---------------------------------------------------

    println!("\n11. Split Slice");

    let (left, right) = slice.split_at(2);

    println!("Left  = {:?}", left);
    println!("Right = {:?}", right);

    //---------------------------------------------------
    // 12. Split Mutable Slice
    //---------------------------------------------------

    println!("\n12. Split Mutable Slice");

    let mut arr4 = [10, 20, 30, 40, 50];

    let (left, right) = arr4.split_at_mut(2);

    left[0] = 100;
    right[0] = 300;

    println!("Left  = {:?}", left);
    println!("Right = {:?}", right);
    println!("Array = {:?}", arr4);

    //---------------------------------------------------
    // 13. First and Last Element
    //---------------------------------------------------

    println!("\n13. First and Last");

    println!("First = {:?}", slice.first());
    println!("Last  = {:?}", slice.last());

    //---------------------------------------------------
    // 14. Contains
    //---------------------------------------------------

    println!("\n14. Contains");

    println!("Contains 30 = {}", slice.contains(&30));
    println!("Contains 99 = {}", slice.contains(&99));

    //---------------------------------------------------
    // 15. Reverse Slice
    //---------------------------------------------------

    println!("\n15. Reverse");

    let mut arr5 = [1, 2, 3, 4, 5];

    arr5[1..4].reverse();

    println!("Reversed Part = {:?}", arr5);

    //---------------------------------------------------
    // 16. Sort Slice
    //---------------------------------------------------

    println!("\n16. Sorting");

    let mut arr6 = [50, 10, 40, 20, 30];

    arr6.sort();

    println!("Sorted = {:?}", arr6);

    //---------------------------------------------------
    // 17. Fill Slice
    //---------------------------------------------------

    println!("\n17. Fill");

    let mut arr7 = [0; 5];

    arr7[1..4].fill(100);

    println!("Filled = {:?}", arr7);

    //---------------------------------------------------
    // 18. Copy Within Slice
    //---------------------------------------------------

    println!("\n18. Copy Within");

    let mut arr8 = [1, 2, 3, 4, 5];

    arr8.copy_within(0..3, 2);

    println!("Result = {:?}", arr8);

    //---------------------------------------------------
    // 19. Swap Slice Elements
    //---------------------------------------------------

    println!("\n19. Swap");

    let mut arr9 = [10, 20, 30, 40, 50];

    arr9.swap(0, 4);

    println!("Result = {:?}", arr9);

    //---------------------------------------------------
    // 20. Reverse Iterator
    //---------------------------------------------------

    println!("\n20. Reverse Iterator");

    for value in slice.iter().rev()
    {
        println!("Value = {}", value);
    }

    //---------------------------------------------------
    // 21. Skip and Take
    //---------------------------------------------------

    println!("\n21. Skip and Take");

    for value in slice.iter().skip(1).take(3)
    {
        println!("Value = {}", value);
    }

    //---------------------------------------------------
    // 22. Sum and Average
    //---------------------------------------------------

    println!("\n22. Sum and Average");

    let sum: i32 = slice.iter().sum();

    let average: f64 = sum as f64 / slice.len() as f64;

    println!("Sum     = {}", sum);
    println!("Average = {}", average);

    //---------------------------------------------------
    // 23. Split into Chunks
    //---------------------------------------------------

    println!("\n23. Chunks");

    for chunk in slice.chunks(2)
    {
        println!("Chunk = {:?}", chunk);
    }

    //---------------------------------------------------
    // 24. Windows
    //---------------------------------------------------

    println!("\n24. Windows");

    for window in slice.windows(3)
    {
        println!("Window = {:?}", window);
    }

    //---------------------------------------------------
    // 25. Passing Slice to Function
    //---------------------------------------------------

    println!("\n25. Function with Slice");

    display_slice(&arr);

    //---------------------------------------------------
    // 26. Passing Mutable Slice to Function
    //---------------------------------------------------

    println!("\n26. Mutable Slice Function");

    let mut arr10 = [1, 2, 3, 4, 5];

    modify_slice(&mut arr10);

    println!("Modified = {:?}", arr10);

    //---------------------------------------------------
    // 27. Slice from Vector
    //---------------------------------------------------

    println!("\n27. Slice from Vector");

    let vec = vec![100, 200, 300, 400, 500];

    let vec_slice: &[i32] = &vec[1..4];

    println!("Vector = {:?}", vec);
    println!("Slice  = {:?}", vec_slice);

    //---------------------------------------------------
    // 28. Empty Slice
    //---------------------------------------------------

    println!("\n28. Empty Slice");

    let empty: &[i32] = &arr[0..0];

    println!("Empty Slice = {:?}", empty);
    println!("Length      = {}", empty.len());

    //---------------------------------------------------
    // 29. Slice Memory Size
    //---------------------------------------------------

    println!("\n29. Slice Memory Size");

    println!("Array size       = {} bytes",
             std::mem::size_of_val(&arr));

    println!("Slice data size  = {} bytes",
             std::mem::size_of_val(slice));

    println!("Slice reference  = {} bytes",
             std::mem::size_of::<&[i32]>());
}

//---------------------------------------------------
// 30. Display Slice Function
//---------------------------------------------------

fn display_slice(slice: &[i32])
{
    println!("Received Slice = {:?}", slice);
}

//---------------------------------------------------
// 31. Modify Slice Function
//---------------------------------------------------

fn modify_slice(slice: &mut [i32])
{
    for value in slice.iter_mut()
    {
        *value += 10;
    }
}


/*
//---------------------------------------------------
Important concepts:

Array versus Slice

Feature - Array - Slice

Syntax\ - [i32; 5] - &[i32]
Size - Fixed - Dynamically sized
Ownership - Owns elements - Borrows elements
Memory - Contiguous - References contiguous data
Modification - With mut - Through &mut
Resizing - Not possible - Not applicable

//---------------------------------------------------


//---------------------------------------------------
Compile and execute:

rustc rust_20261102_ds_Slice_01.rs -o slice

./slice

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

01. Slice from Array
Array = [10, 20, 30, 40, 50]
Slice = [10, 20, 30, 40, 50]

02. Partial Slice
Slice = [20, 30, 40]

03. Slice Range Operations
arr[0..3] = [10, 20, 30]
arr[1..4] = [20, 30, 40]
arr[2..]  = [30, 40, 50]
arr[..3]  = [10, 20, 30]
arr[..]   = [10, 20, 30, 40, 50]

04. Slice Length
Length = 5
Empty  = false

05. Accessing Elements
Index 0 = 10
Index 1 = 20
Index 2 = 30

06. Safe Access
Index 2 = Some(30)
Index 8 = None

07. Iteration
Value = 10
Value = 20
Value = 30
Value = 40
Value = 50

08. Enumerate
Index = 0, Value = 10
Index = 1, Value = 20
Index = 2, Value = 30
Index = 3, Value = 40
Index = 4, Value = 50

09. Mutable Slice
Modified Slice = [200, 300, 400]
Modified Array = [10, 200, 300, 400, 50]

10. Mutable Iteration
Modified Array = [10, 20, 30, 40, 50]

11. Split Slice
Left  = [10, 20]
Right = [30, 40, 50]

12. Split Mutable Slice
Left  = [100, 20]
Right = [300, 40, 50]
Array = [100, 20, 300, 40, 50]

13. First and Last
First = Some(10)
Last  = Some(50)

14. Contains
Contains 30 = true
Contains 99 = false

15. Reverse
Reversed Part = [1, 4, 3, 2, 5]

16. Sorting
Sorted = [10, 20, 30, 40, 50]

17. Fill
Filled = [0, 100, 100, 100, 0]

18. Copy Within
Result = [1, 2, 1, 2, 3]

19. Swap
Result = [50, 20, 30, 40, 10]

20. Reverse Iterator
Value = 50
Value = 40
Value = 30
Value = 20
Value = 10

21. Skip and Take
Value = 20
Value = 30
Value = 40

22. Sum and Average
Sum     = 150
Average = 30

23. Chunks
Chunk = [10, 20]
Chunk = [30, 40]
Chunk = [50]

24. Windows
Window = [10, 20, 30]
Window = [20, 30, 40]
Window = [30, 40, 50]

25. Function with Slice
Received Slice = [10, 20, 30, 40, 50]

26. Mutable Slice Function
Modified = [11, 12, 13, 14, 15]

27. Slice from Vector
Vector = [100, 200, 300, 400, 500]
Slice  = [200, 300, 400]

28. Empty Slice
Empty Slice = []
Length      = 0

29. Slice Memory Size
Array size       = 20 bytes
Slice data size  = 20 bytes
Slice reference  = 16 bytes

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/


