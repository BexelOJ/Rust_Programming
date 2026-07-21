use std::io;

fn main(){
 let n: i32 = 10;   
 let mut sum: i32 = 0;
 
 for i in 1..n {
   sum += i;
   println!("Sum as of index {} is {}",i, sum);
 }

println!("Fianl Sum: {}", sum);
}

