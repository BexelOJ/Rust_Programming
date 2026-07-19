use std::io;

fn factorial(num: i64)->i64{
	if num == 1 || num == 0 {
	   return 1;
	}
	return num * factorial(num-1);
	 	
}

fn main(){
   let mut input = String::new();
   io::stdin().read_line(&mut input).unwrap();
   let n: i64 = input.trim().parse().unwrap();	
	
   let res:i64 = factorial(n); 
   println!("{}! = {}",n,res); 
}


