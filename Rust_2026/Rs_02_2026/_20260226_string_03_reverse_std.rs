fn main() {
    let original = String::from("India");
    let reversed = original.chars().rev().collect::<String>();
    println!("Original: {}", original);
    println!("Reversed: {}", reversed);  // aidnI
}
