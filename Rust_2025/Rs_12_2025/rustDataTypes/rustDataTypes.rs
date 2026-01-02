mod integerTypes;
mod floatTypes;
mod stringType;
mod boolType;

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

let result = intergerTypes::integer();
println!("Final Result : {}",result);
}


