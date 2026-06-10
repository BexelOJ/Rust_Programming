fn main(){
    let mut name:String = String::from("Accenture");
    println!("The initial name value is : {}",name);
    display(&mut name);
    println!("The value of name after modification is : {}",name);
}

fn display(param_name:&mut String){
    println!("The param_name value is : {}",param_name);
    param_name.push_str(" Rocks");
}

