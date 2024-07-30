fn main() {
    //create enum
    let mut light : Light = Light::RED;

    //use the pattern
    println!("{}",pattern(light));

    //change the state
    light = Light::YELLOW;
    
    println!("{}",pattern(light));
}

//define enum
enum Light {
    RED,
    YELLOW,
    GREEN
}
 
fn pattern(light: Light) -> String{
    //define pattern
    match light {
        Light::RED => String::from("RED"),
        Light::YELLOW => String::from("YELLOW"),
        Light::GREEN => String::from("GREEN")
    }
}