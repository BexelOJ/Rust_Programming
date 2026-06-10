fn main(){
let s: String = String::from("India");

println!("Country is {}", s);

let l = s.len();
println!("Length is {}", l);

for j in (0..l).rev(){
    let _byte = s.as_bytes()[j];
    print!("{:?} ", _byte); 
 }  
 
 println!();

}

