use std::any::type_name;

fn print_type<T>(_: &T) {
    println!("{}", type_name::<T>());
}

fn main() {
    let x = 42;
    print_type(&x);     // "i32"

   let y = 3.14;
   print_type(&y);     // "f64"

}


