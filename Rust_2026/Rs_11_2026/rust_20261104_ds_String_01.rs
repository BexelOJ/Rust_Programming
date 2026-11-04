fn main()
{
    //---------------------------------------------------
    // String - Rust Data Structures
    // Date: 20261104

    //---------------------------------------------------
    // 01. String Declaration
    //---------------------------------------------------

    let s1: String = String::from("Hello Rust");

    println!("01. String Declaration");
    println!("s1 = {}", s1);

    //---------------------------------------------------
    // 02. String Creation Using new()
    //---------------------------------------------------

    let s2 = String::new();

    println!("\n02. Empty String");
    println!("s2 = '{}'", s2);

    //---------------------------------------------------
    // 03. String Creation Using to_string()
    //---------------------------------------------------

    let s3 = "Embedded Systems".to_string();

    println!("\n03. to_string()");
    println!("s3 = {}", s3);

    //---------------------------------------------------
    // 04. String Mutability
    //---------------------------------------------------

    println!("\n04. Mutable String");

    let mut s4 = String::from("Rust");

    s4.push_str(" Programming");

    println!("s4 = {}", s4);

    //---------------------------------------------------
    // 05. push_str()
    //---------------------------------------------------

    println!("\n05. push_str()");

    let mut s5 = String::from("Linux");

    s5.push_str(" Kernel");

    println!("s5 = {}", s5);

    //---------------------------------------------------
    // 06. push()
    //---------------------------------------------------

    println!("\n06. push()");

    let mut s6 = String::from("ABC");

    s6.push('D');

    println!("s6 = {}", s6);

    //---------------------------------------------------
    // 07. String Length
    //---------------------------------------------------

    println!("\n07. String Length");

    let s7 = String::from("Rust");

    println!("Length = {}", s7.len());

    //---------------------------------------------------
    // 08. String Capacity
    //---------------------------------------------------

    println!("\n08. String Capacity");

    let mut s8 = String::with_capacity(50);

    s8.push_str("Rust");

    println!("Length   = {}", s8.len());
    println!("Capacity = {}", s8.capacity());

    //---------------------------------------------------
    // 09. String is_empty()
    //---------------------------------------------------

    println!("\n09. is_empty()");

    let s9 = String::new();

    println!("Empty = {}", s9.is_empty());

    //---------------------------------------------------
    // 10. String clear()
    //---------------------------------------------------

    println!("\n10. clear()");

    let mut s10 = String::from("Hello Rust");

    s10.clear();

    println!("After clear = '{}'", s10);

    //---------------------------------------------------
    // 11. String Concatenation Using +
    //---------------------------------------------------

    println!("\n11. String Concatenation");

    let s11 = String::from("Hello ");
    let s12 = String::from("Rust");

    let s13 = s11 + &s12;

    println!("Result = {}", s13);

    //---------------------------------------------------
    // 12. Concatenation Using format!()
    //---------------------------------------------------

    println!("\n12. format!()");

    let s14 = String::from("Hello");
    let s15 = String::from("World");

    let s16 = format!("{} {}", s14, s15);

    println!("Result = {}", s16);
    println!("s14 = {}", s14);
    println!("s15 = {}", s15);

    //---------------------------------------------------
    // 13. String Clone
    //---------------------------------------------------

    println!("\n13. Clone");

    let s17 = String::from("Rust");

    let s18 = s17.clone();

    println!("Original = {}", s17);
    println!("Clone    = {}", s18);

    //---------------------------------------------------
    // 14. String Move
    //---------------------------------------------------

    println!("\n14. Ownership Move");

    let s19 = String::from("Ownership");

    let s20 = s19;

    println!("Moved String = {}", s20);

    //---------------------------------------------------
    // 15. String Comparison
    //---------------------------------------------------

    println!("\n15. String Comparison");

    let s21 = String::from("Rust");
    let s22 = String::from("Rust");
    let s23 = String::from("C");

    println!("s21 == s22 : {}", s21 == s22);
    println!("s21 == s23 : {}", s21 == s23);

    //---------------------------------------------------
    // 16. String Contains
    //---------------------------------------------------

    println!("\n16. Contains");

    let s24 = String::from("Rust Programming Language");

    println!("Contains Rust = {}", s24.contains("Rust"));
    println!("Contains Java = {}", s24.contains("Java"));

    //---------------------------------------------------
    // 17. String Starts With and Ends With
    //---------------------------------------------------

    println!("\n17. Starts With and Ends With");

    println!("Starts with Rust = {}", s24.starts_with("Rust"));
    println!("Ends with Language = {}", s24.ends_with("Language"));

    //---------------------------------------------------
    // 18. String Replace
    //---------------------------------------------------

    println!("\n18. Replace");

    let s25 = String::from("Rust is powerful");

    let s26 = s25.replace("powerful", "fast");

    println!("Original = {}", s25);
    println!("Modified = {}", s26);

    //---------------------------------------------------
    // 19. String Trim
    //---------------------------------------------------

    println!("\n19. Trim");

    let s27 = String::from("   Hello Rust   ");

    println!("Original = '{}'", s27);
    println!("Trimmed  = '{}'", s27.trim());

    //---------------------------------------------------
    // 20. String Split Whitespace
    //---------------------------------------------------

    println!("\n20. Split Whitespace");

    let s28 = "Rust is a systems programming language";

    for word in s28.split_whitespace()
    {
        println!("Word = {}", word);
    }

    //---------------------------------------------------
    // 21. String Split
    //---------------------------------------------------

    println!("\n21. Split");

    let s29 = "C,C++,Rust,Python";

    for language in s29.split(',')
    {
        println!("Language = {}", language);
    }

    //---------------------------------------------------
    // 22. String Lines
    //---------------------------------------------------

    println!("\n22. Lines");

    let s30 = "Line 1\nLine 2\nLine 3";

    for line in s30.lines()
    {
        println!("{}", line);
    }

    //---------------------------------------------------
    // 23. String Bytes
    //---------------------------------------------------

    println!("\n23. Bytes");

    let s31 = "Rust";

    for byte in s31.bytes()
    {
        print!("{} ", byte);
    }

    println!();

    //---------------------------------------------------
    // 24. String Characters
    //---------------------------------------------------

    println!("\n24. Characters");

    let s32 = "Rust";

    for ch in s32.chars()
    {
        println!("Character = {}", ch);
    }

    //---------------------------------------------------
    // 25. UTF-8 String
    //---------------------------------------------------

    println!("\n25. UTF-8");

    let s33 = "नमस्ते";

    println!("String       = {}", s33);
    println!("Byte Length  = {}", s33.len());
    println!("Char Count   = {}", s33.chars().count());

    //---------------------------------------------------
    // 26. Safe Character Access
    //---------------------------------------------------

    println!("\n26. Character Access");

    let s34 = "Rust";

    println!("First  = {:?}", s34.chars().nth(0));
    println!("Second = {:?}", s34.chars().nth(1));
    println!("Third  = {:?}", s34.chars().nth(2));

    //---------------------------------------------------
    // 27. String Slice
    //---------------------------------------------------

    println!("\n27. String Slice");

    let s35 = "Hello Rust";

    let part = &s35[0..5];

    println!("Original = {}", s35);
    println!("Slice    = {}", part);

    //---------------------------------------------------
    // 28. String to Uppercase
    //---------------------------------------------------

    println!("\n28. Uppercase");

    let s36 = "rust programming";

    println!("{}", s36.to_uppercase());

    //---------------------------------------------------
    // 29. String to Lowercase
    //---------------------------------------------------

    println!("\n29. Lowercase");

    let s37 = "RUST PROGRAMMING";

    println!("{}", s37.to_lowercase());

    //---------------------------------------------------
    // 30. String Reverse Characters
    //---------------------------------------------------

    println!("\n30. Reverse Characters");

    let s38 = "Rust";

    let reversed: String = s38.chars().rev().collect();

    println!("Original = {}", s38);
    println!("Reversed = {}", reversed);

    //---------------------------------------------------
    // 31. String Insert
    //---------------------------------------------------

    println!("\n31. Insert");

    let mut s39 = String::from("Rust");

    s39.insert(0, 'A');

    println!("Result = {}", s39);

    //---------------------------------------------------
    // 32. String Insert String
    //---------------------------------------------------

    println!("\n32. Insert String");

    let mut s40 = String::from("Programming");

    s40.insert_str(0, "Rust ");

    println!("Result = {}", s40);

    //---------------------------------------------------
    // 33. String Remove
    //---------------------------------------------------

    println!("\n33. Remove");

    let mut s41 = String::from("Rust");

    let removed = s41.remove(0);

    println!("Removed  = {}", removed);
    println!("Remaining = {}", s41);

    //---------------------------------------------------
    // 34. String Pop
    //---------------------------------------------------

    println!("\n34. Pop");

    let mut s42 = String::from("Rust");

    println!("Popped = {:?}", s42.pop());
    println!("String = {}", s42);

    //---------------------------------------------------
    // 35. String Truncate
    //---------------------------------------------------

    println!("\n35. Truncate");

    let mut s43 = String::from("Rust Programming");

    s43.truncate(4);

    println!("Result = {}", s43);

    //---------------------------------------------------
    // 36. String Retain
    //---------------------------------------------------

    println!("\n36. Retain");

    let mut s44 = String::from("a1b2c3d4");

    s44.retain(|ch| ch.is_alphabetic());

    println!("Result = {}", s44);

    //---------------------------------------------------
    // 37. String Repeat
    //---------------------------------------------------

    println!("\n37. Repeat");

    let s45 = "Rust ".repeat(3);

    println!("Result = {}", s45);

    //---------------------------------------------------
    // 38. String Find
    //---------------------------------------------------

    println!("\n38. Find");

    let s46 = "Rust Programming Language";

    println!("Position = {:?}", s46.find("Programming"));

    //---------------------------------------------------
    // 39. String Match Indices
    //---------------------------------------------------

    println!("\n39. Match Indices");

    let s47 = "Rust is fast and Rust is safe";

    for (index, value) in s47.match_indices("Rust")
    {
        println!("Index = {}, Match = {}", index, value);
    }

    //---------------------------------------------------
    // 40. String Capacity Management
    //---------------------------------------------------

    println!("\n40. Capacity Management");

    let mut s48 = String::with_capacity(10);

    s48.push_str("Rust");

    println!("Length   = {}", s48.len());
    println!("Capacity = {}", s48.capacity());

    s48.shrink_to_fit();

    println!("After shrink_to_fit = {}", s48.capacity());

    //---------------------------------------------------
    // 41. String Memory Size
    //---------------------------------------------------

    println!("\n41. String Memory Size");

    let s49 = String::from("Hello");

    println!("String object size = {} bytes",
             std::mem::size_of_val(&s49));

    println!("String length      = {} bytes", s49.len());

    println!("String capacity    = {} bytes", s49.capacity());

    //---------------------------------------------------
    // 42. String Slice from String
    //---------------------------------------------------

    println!("\n42. String Slice Reference");

    let s50 = String::from("Embedded Rust");

    let slice: &str = &s50[9..13];

    println!("Original = {}", s50);
    println!("Slice    = {}", slice);

    //---------------------------------------------------
    // 43. Passing String to Function
    //---------------------------------------------------

    println!("\n43. Passing String");

    let s51 = String::from("Rust Programming");

    display_string(&s51);

    //---------------------------------------------------
    // 44. Passing Mutable String
    //---------------------------------------------------

    println!("\n44. Mutable String Function");

    let mut s52 = String::from("Hello");

    modify_string(&mut s52);

    println!("Modified = {}", s52);

    //---------------------------------------------------
    // 45. Returning String from Function
    //---------------------------------------------------

    println!("\n45. Returning String");

    let s53 = create_string();

    println!("Returned = {}", s53);

    //---------------------------------------------------
    // 46. String and str
    //---------------------------------------------------

    println!("\n46. String and str");

    let owned: String = String::from("Rust");
    let borrowed: &str = "Programming";

    println!("String = {}", owned);
    println!("str    = {}", borrowed);

    //---------------------------------------------------
    // 47. String Concatenation Using push_str()
    //---------------------------------------------------

    println!("\n47. Concatenation");

    let mut s54 = String::from("Linux");

    s54.push_str(" Device Driver");

    println!("Result = {}", s54);

    //---------------------------------------------------
    // 48. String Equality Ignoring Case
    //---------------------------------------------------

    println!("\n48. Case Insensitive Comparison");

    let s55 = "RUST";
    let s56 = "rust";

    println!("Equal = {}", s55.eq_ignore_ascii_case(s56));

    //---------------------------------------------------
    // 49. String Split At
    //---------------------------------------------------

    println!("\n49. Split At");

    let s57 = "RustProgramming";

    let (left, right) = s57.split_at(4);

    println!("Left  = {}", left);
    println!("Right = {}", right);

    //---------------------------------------------------
    // 50. String Join
    //---------------------------------------------------

    println!("\n50. Join");

    let languages = ["C", "C++", "Rust", "Python"];

    let joined = languages.join(" | ");

    println!("Result = {}", joined);
}

//---------------------------------------------------
// 51. Display String Function
//---------------------------------------------------

fn display_string(data: &str)
{
    println!("Received = {}", data);
}

//---------------------------------------------------
// 52. Modify String Function
//---------------------------------------------------

fn modify_string(data: &mut String)
{
    data.push_str(" Rust");
}

//---------------------------------------------------
// 53. Create String Function
//---------------------------------------------------

fn create_string() -> String
{
    String::from("Returned from function")
}


/*
//---------------------------------------------------
Important concepts:

String versus &str:

Feature - String - &str

Ownership - Owns data - Borrows data
Storage - Heap-backed - Can reference different storage locations
Growable - Yes - No
Mutable contents - Yes, with mutable access - Only through &mut str
UTF-8 - Yes - Yes
Typical syntax - String::from("Rust") - "Rust"

//---------------------------------------------------


//---------------------------------------------------
Compile and execute:

rustc rust_20261104_ds_String_01.rs -o string

./string

//---------------------------------------------------


//---------------------------------------------------
Output:
//-------------------------

01. String Declaration
s1 = Hello Rust

02. Empty String
s2 = ''

03. to_string()
s3 = Embedded Systems

04. Mutable String
s4 = Rust Programming

05. push_str()
s5 = Linux Kernel

06. push()
s6 = ABCD

07. String Length
Length = 4

08. String Capacity
Length   = 4
Capacity = 50

09. is_empty()
Empty = true

10. clear()
After clear = ''

11. String Concatenation
Result = Hello Rust

12. format!()
Result = Hello World
s14 = Hello
s15 = World

13. Clone
Original = Rust
Clone    = Rust

14. Ownership Move
Moved String = Ownership

15. String Comparison
s21 == s22 : true
s21 == s23 : false

16. Contains
Contains Rust = true
Contains Java = false

17. Starts With and Ends With
Starts with Rust = true
Ends with Language = true

18. Replace
Original = Rust is powerful
Modified = Rust is fast

19. Trim
Original = '   Hello Rust   '
Trimmed  = 'Hello Rust'

20. Split Whitespace
Word = Rust
Word = is
Word = a
Word = systems
Word = programming
Word = language

21. Split
Language = C
Language = C++
Language = Rust
Language = Python

22. Lines
Line 1
Line 2
Line 3

23. Bytes
82 117 115 116

24. Characters
Character = R
Character = u
Character = s
Character = t

25. UTF-8
String       = नमस्ते
Byte Length  = 18
Char Count   = 6

26. Character Access
First  = Some('R')
Second = Some('u')
Third  = Some('s')

27. String Slice
Original = Hello Rust
Slice    = Hello

28. Uppercase
RUST PROGRAMMING

29. Lowercase
rust programming

30. Reverse Characters
Original = Rust
Reversed = tsuR

31. Insert
Result = ARust

32. Insert String
Result = Rust Programming

33. Remove
Removed  = R
Remaining = ust

34. Pop
Popped = Some('t')
String = Rus

35. Truncate
Result = Rust

36. Retain
Result = abcd

37. Repeat
Result = Rust Rust Rust

38. Find
Position = Some(5)

39. Match Indices
Index = 0, Match = Rust
Index = 17, Match = Rust

40. Capacity Management
Length   = 4
Capacity = 10
After shrink_to_fit = 4

41. String Memory Size
String object size = 24 bytes
String length      = 5 bytes
String capacity    = 5 bytes

42. String Slice Reference
Original = Embedded Rust
Slice    = Rust

43. Passing String
Received = Rust Programming

44. Mutable String Function
Modified = Hello Rust

45. Returning String
Returned = Returned from function

46. String and str
String = Rust
str    = Programming

47. Concatenation
Result = Linux Device Driver

48. Case Insensitive Comparison
Equal = true

49. Split At
Left  = Rust
Right = Programming

50. Join
Result = C | C++ | Rust | Python

//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------


//---------------------------------------------------
*/


