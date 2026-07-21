/*

--------------------------------------------------
built-in types:

i32, i64
f32, f64
bool
char
tuple
array
slice

------------------------------

 Primitive Types
│     i32
│     f64
│     bool
│     char
│
├── Compound Types
│     Array
│     Tuple
│     Slice
│
└── Collections (std::collections and std)
      Vec
      String
      HashMap
      HashSet
      VecDeque
      BinaryHeap
      LinkedList
      BTreeMap
      BTreeSet

--------------------------------------------------
vector: (standard library type)

let mut v = Vec::new(); 
v.push(10);	
❌ No (inferred as Vec<i32>)

let v = Vec::new();	
✅ Yes, unless later usage provides the type

let v: Vec<i32> = Vec::new();
✅ Explicit

let v = Vec::<i32>::new();
✅ Explicit

let v = vec![1,2,3];
❌ No (inferred)

------------------------------
let mut v = Vec::new();

v.push(10);
v.push(20);
v.pop();

--------------------------------------------------


--------------------------------------------------


--------------------------------------------------


--------------------------------------------------


--------------------------------------------------


--------------------------------------------------


--------------------------------------------------


--------------------------------------------------


--------------------------------------------------


--------------------------------------------------


--------------------------------------------------

*/
