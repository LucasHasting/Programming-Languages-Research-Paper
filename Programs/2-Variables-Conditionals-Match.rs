fn main() {
    //initialize with a type
    let val1 : i8 = 6;
    let val3 = 7_i8;

    //type inference
    let val2 = 6;
    
    //conditional
    if (val1 == val3){
        println!("Block 1");
    } else if (val1 == val2){
        println!("Block 2");
    } else {
        println!("Block 3");
    }

    switch(val1);
    switch(val2);
    switch(val3);
}

fn switch(val: i8){
    //match statement
    match val {
        6 => println!("Case 1"),
        7 => println!("Case 2"),
        _ => println!("Default")
    }

    return;
}