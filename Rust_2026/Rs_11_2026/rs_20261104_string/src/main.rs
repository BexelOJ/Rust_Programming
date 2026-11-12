fn main() {

    let mut empty_string:String = String::new();
    let mut str:String = String::from("Bexel");
    let mut new_string:String = String::with_capacity(100);

    println!("Str : {}",str);

    str.push_str(" OJ");
    println!("New Str : {}",str);

    str.push('!');
    println!("Latest Str : {}", str);

    println!("length : {}",str.len());
    
    str.trim();
    println!("Length, after trim : {}",str.len());
    
    str.clear();
    println!("Length, after clear : {}",str.len());

    println!("Capacity Length : {}", new_string.len());
}


/*
Associated functions	Methods

String::new()	name.len()
String::from()	name.push_str()
String::with_capacity()	name.push()
String::from_utf8()	name.clear()
String::from_utf16()	name.trim()*

*/
