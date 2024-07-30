use std::ptr;

fn main(){
    let mut buff = [3, 2, 7, 4, 11, 13];

    unsafe {
        let ptr: *mut i32 = buff.as_mut_ptr();

        //buffer over-read
        println!("{}",*(ptr.wrapping_add(6)));

        //buffer overflow
        ptr::write(ptr.wrapping_add(6), 5);
    }
}