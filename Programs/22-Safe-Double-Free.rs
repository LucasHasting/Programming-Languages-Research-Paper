fn main(){
    let val: String = String::from("Rust");
    std::mem::drop(val);
    std::mem::drop(val);
}