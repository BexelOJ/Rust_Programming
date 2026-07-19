use std::io;
use std::f32::consts::PI;

fn area_rectangle()->f32{
   let mut input = String::new();
   io::stdin().read_line(&mut input).unwrap();
   let l: f32 = input.trim().parse().unwrap();
 
   input.clear(); 
   io::stdin().read_line(&mut input).unwrap();
   let w: f32 = input.trim().parse().unwrap();
   l * w
}

fn area_triangle()->f32{
   let mut input = String::new();
   io::stdin().read_line(&mut input).unwrap();
   let b: f32 = input.trim().parse().unwrap();
  
   input.clear();
   io::stdin().read_line(&mut input).unwrap();
   let h: f32 = input.trim().parse().unwrap();
   0.5 * b * h
}

fn area_circle()->f32{
   let mut input = String::new();
   io::stdin().read_line(&mut input).unwrap();
   let r: f32 = input.trim().parse().unwrap();
   PI * r * r
}

fn main(){
   let mut input = String::new();
   io::stdin().read_line(&mut input).unwrap();
   let shape = input.trim().to_string();

   let res: f32 = match shape.as_str() {
      "rectangle" => area_rectangle(),
      "triangle" => area_triangle(),
      "circle" => area_circle(),     
      _ => 0.00
   }; 

   println!("{:.2}",res);
}


