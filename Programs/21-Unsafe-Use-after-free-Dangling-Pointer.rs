use std::ptr::drop_in_place;

fn main(){
    let val: *mut String = Box::into_raw(Box::new(String::from("Rust")));

    unsafe {
        println!("{}", *val);
        drop_in_place(val);
        println!("{}", *val);
    }
}