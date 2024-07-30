fn main(){
    //trait definition
    trait AnimalMethods {
        fn talk(&mut self);
    }

    //struct object definition
    pub struct Animal<'a> {
        pub name: &'a str,
        action: &'a str
    }

    //implement the trait for the Animal class
    impl AnimalMethods for Animal<'_> {
        fn talk(&mut self) {
            println!("{}", self.action);
        }
    }

    //create the struct object
    let mut dog = Animal { name: "Joe", action: "Bark"};
    println!("{}",dog.name);

    //call the method implemented by the trait
    dog.talk();
}