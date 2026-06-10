fn main() {
    let b:(i32,bool,f64) = (30,true,7.9);
    user_defined_print(b);
}

fn user_defined_print(x:(i32,bool,f64)){
    println!("Inside user defined print method");
    let (age,is_male,cgpa) = x;
    println!("Age : {}\nMale : {}\nCGPA : {}",age,is_male, cgpa);
}

