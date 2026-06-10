fn main(){
    let mut data = [10,20,30,40,50];
    use_slice_1(&data[3..5]);
    use_slice_2(&mut data[0..3]);

}

fn use_slice_1(slice:&[i32]){
    println!("Length of slice : {}",slice.len());
    println!("{:?}",slice);
}

fn use_slice_2(slice:&mut [i32]){
    println!("Length of slice : {}",slice.len());
    println!("{:?}",slice);
    for i in 0..slice.len(){
        slice[i] = slice[i] + 1;
    }
    println!("{:?}",slice);
}

