fn add_one(e:&mut i32){
    *e += 1;
    println!("{}",e);
    println!("{}",*e);
}

fn main(){
    let mut i = 3;
    add_one(&mut i);
    println!("{}",i);
}

