
fn add(num1: i32, num2:i32)->i32{
let res = num1 + num2;
println!("Sum: {} ",res);
return res;
}

fn subtract(num1: i32, num2 : i32)->i32{
let res = num1 - num2;
println!("Difference: {} ",res) ;
res
}

fn multiply(num1: i32, num2 : i32)->i32{
let res = num1*num2 ;
println!("Product : {} ",res) ;
res
}

fn divi(num1: i32, num2 : i32)->i32{
let res = num1/num2 ;
println!("Division : {} ",res);
res
}

fn modulus(num1: i32, num2 : i32)->i32{
let res = num1%num2 ;
println!("Modulus : {} ",res);
res
}


fn main(){
    let num1 = 10;
    let num2 = 20;

    let result = add(num1, num2);
    println!("Final addition Result : {}\n",result);

    let result = subtract(num1, num2);
    println!("Final Differnece Result : {}\n",result);

    let result = multiply(num1, num2);
    println!("Final Product Result : {}\n",result);

    let result = divi(num1, num2);
    println!("Final Division Result : {}\n",result);

    let result = modulus(num1, num2);
    println!("Final Modulus Result : {}\n",result);

}


