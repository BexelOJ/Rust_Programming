fn main(){
    let v = vec![1,2,3];
    println!("v = {:?}",v);
    //let v2 = v;
    let v2 = v;
    println!("v2 = {:?}",v2);
    //println!("{:?}",v);
    let res = display(v2);
    println!("res = {:?}",res);

}

fn display(v3:Vec<i32>)->Vec<i32>{
    println!("v3 = {:?}",v3);
    return v3;
}
