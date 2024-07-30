fn main(){
    //declare mutable variable
    let mut val = 5_i8;

    //pass in reference to the function
    read(&val);

    //pass in mutable reference to the function
    change(&mut val);
    read(&val);

    //allocate value in heap as a smart pointer
    let _v = Box::new(val);

    //raw pointer, unsafe
    unsafe {
        let _bad_val: *mut i8 = &mut val as *mut i8; 
        println!("{}", *_bad_val);
    }
}

//mutable reference to a variabel
fn change(val: &mut i8){
    *val = *val + 1;
}

//Read-only reference to a variable
fn read(val: &i8){
    println!("val is {}", *val);
}