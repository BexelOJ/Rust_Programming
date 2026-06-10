fn main(){
    let arr:[i32;4] = [10,20,30,40];
    println!("\narray is {:?}\n",arr);
    println!("array size is :{}\n",arr.len());

    for index in 0..4 {
        println!("index is: {} & value is : {}",index,arr[index]);
    }

    println!("\narray size is :{}\n",arr.len());

    for val in arr.iter(){
        println!("value is :{}",val);
    }



}

