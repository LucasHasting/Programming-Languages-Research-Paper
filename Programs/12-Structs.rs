fn main(){
    //struct object definition
    pub struct Animal<'a> {
        pub name: &'a str,
        action: &'a str
    }

    //struct object implementation
    impl Animal<'_> {
        pub fn talk(&mut self) {
            println!("{}", self.action);
        }
    }

    //create the struct object
    let mut dog = Animal { name: "Joe", action: "Bark"};
    println!("{}",dog.name);
    dog.talk();
}