fn main(){
    let b:(i32,bool,f64) = (110,true,10.9);
    user_defined_print(b);
}
//pass the tuple as a parameter
fn user_defined_print(x:(i32,bool,f64)){
    println!("Inside print method");
    println!("{:?}",x);
}

