fn main(){
    println!("{}", drop());
}

fn drop() -> &'static String{
    let val = String::from("Rust");
    return &val;
}