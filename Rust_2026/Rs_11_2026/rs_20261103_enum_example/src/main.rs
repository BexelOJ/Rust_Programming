#[derive(Debug)]
enum Day {
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY,
}

fn main() {
    println!("Hello, world!");
    println!("First day of the week {:?}",Day::MONDAY);
}


