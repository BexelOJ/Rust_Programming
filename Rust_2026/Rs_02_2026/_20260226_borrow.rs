fn main(){
    let mut v = vec![1,2,3];
    let len = v.len();
    
    println!("Length of this Vector : {}",len);

    for i in 0..len{
        println!("Printing the Initial value from main() v[{}] = {}", i, v[i]);
    }
    println!();
    print_vector(&mut v);

}

fn print_vector(x:&mut Vec<i32>){
    let l = x.len();
    for u in 0..l{
        println!("Printing the value outside_function, index {} = {}",u, x[u]);
    }

    println!();
    println!("Length of this Vector : {}",l);
     
    for i in 0..l{
       x[i] = x[i] + 1;
    }
     
    for i in 0..l{
        println!("Printing the updated value at index {} = {}",i,x[i]);
    }

}

