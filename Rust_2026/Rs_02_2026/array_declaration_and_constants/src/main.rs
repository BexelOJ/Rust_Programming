fn main() {

    const N: usize = 20; // pointer sized
    let arr = [0; N];
    print!("{}\n",arr[10])
}

/*
This will throw and Error:

let N: usize = 20;
let arr = [0; N]; //Error: non-constant used with constant

*/