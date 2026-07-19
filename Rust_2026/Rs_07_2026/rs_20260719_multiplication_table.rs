use std::io;

fn main(){
let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
let f: i32 = input.trim().parse().unwrap();

//let mut mul = f;
for x in 1..11{	
  //mul = f * x;
  println!("{} x {}: = {}",f,x,(f*x));

}
}

