use lib1::home::kitchen::*;
use lib1::home::bed_room::*;


fn main() {
    let tuple:(i32,f64,u8) = (-325,4.9,22);
    println!("{:?}\n",tuple);

    println!("integer is :{:?}\n",tuple.0);
    println!("float is :{:?}\n",tuple.1);
    println!("unsigned int is :{:?}\n",tuple.2);

    extra_tuple_1();
    cook();    // From kitchen.rs
    sleep();   // From bedroom.rs
}



