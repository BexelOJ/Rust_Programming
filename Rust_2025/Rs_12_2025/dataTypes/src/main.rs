mod integer_types;
mod float_type;
//mod stringType;
//mod boolType;

fn main(){

let company_name = "Google"; // string type
let year = 1998;             // int type
let rating = 4.8;            // float type
let is_growing = true;       // boolean type
let icon_char = '#';

println!();
println!("Welcom to {}", company_name);
println!("{} tarted in the year: {}",company_name,year);
println!("{} is a good company",company_name);
println!("{} rating is {}",company_name,rating);
println!("It is {} that {} is growing tremendously",is_growing, company_name);
println!();
println!("I {} {}", icon_char,company_name);

let int_result = integer_types::integer();
println!("Final Integer : {}",int_result);

let float_res = float_type::area_circle();
println!("Final Float : {}",float_res);

}


