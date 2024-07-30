fn main(){
    let mut val1 = 5_i8;
    let mut val2: String = "Hello".to_string();

    //call generic functions
    val1 = get_value::<i8>(val1);
    val2 = get_value::<String>(val2);

    println!("{} {}",val1, val2)
}

//define generic function
fn get_value<T: std::fmt::Display>(value: T) -> T{
    println!("{}", value);
    return value;
}