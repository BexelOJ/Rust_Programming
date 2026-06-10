fn main(){
    // Method 1: From string literal (most common)
    let _s1: String = String::from("hello");
    
    // Method 2: Using to_string()
    let _s2: String = "hello".to_string();
    
    // Method 3: Empty string
    let mut _s3: String = String::new();
    _s3.push_str("India");

    // Method 4: With capacity (optimized)
    let _s4: String = String::with_capacity(100);

    println!("{}\n", _s1);
    println!("{}\n", _s2);
    println!("{}\n", _s3);
//    println!("{}\n", _s4);

    let str_len = _s1.len();
    println!("{}\n", str_len);
    
    /*
    for i in 0..str_len{
        println!("{}\n", _s3);        
    }
    */

    let reversed: String = _s1.chars().rev().collect();
    println!("{}", reversed);  // "olleh"

    let reverse_str: String = _s3.chars().rev().collect();
    println!("{}",reverse_str);

}

