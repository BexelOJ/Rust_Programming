struct Animal {

}

struct Dog {
    animal : Animal
}

struct Cat {
    animal: Animal
}

trait Sound {
    fn sound(&self);
}

impl Sound for Dog {
    fn sound(&self) {
	println!("Dog Barkk!");
    }
}

impl Sound for Cat {
    fn sound(&self){
	println!("Cat Meoww!");
    }
}

fn main(){
    let dog = Dog {animal: Animal{}};
    dog.sound();
    
    let cat = Cat{animal: Animal{}};
    cat.sound();
}
