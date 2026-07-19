use std::io;


fn main(){
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  let score: i32 = input.trim().parse().unwrap();
  
  let grade = match score{
    90..=100 => "A",
    80..=89 => "B",
    70..=79 => "C",
    60..=69 => "D", 
    0..=59 => "F",
    _ => "Invalid Score",  
};
  println!("{}",grade);
}


