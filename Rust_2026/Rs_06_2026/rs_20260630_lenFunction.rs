fn main(){
   let s = String::from("Hello");
   println!("String : {}", s);
   println!("Length : {}\n", s.len());

   let a = [1, 2, 3, 4, 5];
   println!("Array : {:?}", a);
   println!("Length : {}\n", a.len());

   let v = vec![1, 2, 3, 4, 5];
   println!("Vector : {:?}", v);
   println!("Length : {}\n", v.len());

   for i in a{
      println!("{}",i);
   }
}

