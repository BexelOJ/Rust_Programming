fn main(){
    let v = vec![10,20,30];
    print_vector(&v); // passing reference
    println!("Printing the value from main() v[0]={}\n",v[0]);
}

fn print_vector(x:&Vec<i32>){
    println!("\nInside print_vector function {:?}\n",x);
}

