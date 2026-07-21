use std::io;

fn main(){
  let mut input_1 = String::new();
  io::stdin().read_line(&mut input_1).unwrap();  

  let mut input_2 = String::new();
  io::stdin().read_line(&mut input_2).unwrap();  

  let mut swap = String::new();
  swap = input_2;
  input_2 = input_1;
  input_1 = swap;
  
  println!("{}",input_1);
  println!("{}",input_2);

}


