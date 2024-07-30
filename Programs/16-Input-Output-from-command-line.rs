use std::io;

fn main() {
    //set empty string
    let mut input = String::new();

    //read input
    io::stdin().read_line(&mut input);

    //trim input
    println!("You typed: {}", input.trim());
}