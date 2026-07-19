use std::io;

fn main(){
   let mut input = String::new();
   io::stdin().read_line(&mut input).unwrap();
   let count: i32 = input.trim().parse().unwrap();
   
   let mut sum: i32 = 0;
   // let c: i32 = count+1;

   for _ in 0..count {
      let mut input = String::new();
      io::stdin().read_line(&mut input).unwrap(); 
      let i: i32 = input.trim().parse().unwrap();
      sum += i;	   
   }   

   println!("{}",sum);  
}


