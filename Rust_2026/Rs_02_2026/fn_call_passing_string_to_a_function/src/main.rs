fn main(){
    let name:String = String::from("Tutorials Point");
    display(name); //cannot access name after display
}

fn display(param_name:String){
    println!("param_name value is :{}",param_name);
}


