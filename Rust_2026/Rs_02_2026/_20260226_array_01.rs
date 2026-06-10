fn main(){
    let arr: [i32;3] = [1,2,3];
    let vec = vec![10, 20, 30];
    let slice = &vec[0..2];
    
    println!("{:?}",arr);
    println!("{:?}",vec);
    println!("{:#?}",vec);
    println!("{:?}",slice);

}

