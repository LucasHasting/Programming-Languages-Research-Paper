use std::ptr::drop_in_place;

fn main(){
    unsafe {
        let val: *mut String = Box::into_raw(Box::new(String::from("Rust")));
        println!("{}", *val);
        drop_in_place(val);
        drop_in_place(val);
        println!("finished");
    }
}