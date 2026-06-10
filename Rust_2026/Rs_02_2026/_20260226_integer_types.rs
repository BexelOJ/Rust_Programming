use std::mem;

fn main(){
    
    let a1: i8 = 10;
    let a2: u8 = 15;

    let b1: i16 = 20;
    let b2: u16 = 25;
    
    let c1: i32 = 30;
    let c2: u32 = 35;
    
    let d1: i64 = 40;
    let d2: u64 = 45;

    let e1: i128 = 50;
    let e2: u128 = 55;

    let s:String = String::from("India");

    println!("a1 = {}",a1);
    println!("a2 = {}",a2);
    println!("size of a1 = {} byte",mem::size_of::<i8>());
    println!("size of a2 = {} byte\n",mem::size_of::<u8>());

    println!("b1 = {}",b1);
    println!("b1 = {}",b2);
    println!("size of a1 = {} byte",mem::size_of::<i16>());
    println!("size of a2 = {} byte\n",mem::size_of::<u16>());

    println!("c1 = {}",c1);
    println!("c1 = {}",c2);
    println!("size of a1 = {} byte",mem::size_of::<i32>());
    println!("size of a2 = {} byte\n",mem::size_of::<u32>());

    println!("d1 = {}",d1);
    println!("d1 = {}",d2);
    println!("size of a1 = {} byte",mem::size_of::<i64>());
    println!("size of a2 = {} byte\n",mem::size_of::<u64>());

    println!("e1 = {}",e1);
    println!("e1 = {}",e2);
    println!("size of a1 = {} byte",mem::size_of::<i128>());
    println!("size of a2 = {} byte\n",mem::size_of::<u128>());

}


