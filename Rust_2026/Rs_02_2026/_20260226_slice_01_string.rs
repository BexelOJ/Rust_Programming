fn main(){
    let n1 = "Accenture".to_string();
    println!("Length of the Strign is : {}",n1.len());
    println!("Original String : {}",n1);
    let c1 = &n1[4..9];
    println!("{}",c1);
}

