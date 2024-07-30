fn main(){
    //initilize the vector with values
    let vec1 = vec![3, 2, 7, 4, 11, 13];

    //create slices, works with arrays and vector
    let slice1: &[u16] = &vec1[1..3];
    let slice2: &[u16] = &vec1[4..];

    //print the slices
    println!("Slice 1:");

    for i in slice1 {
        println!("{}", i);
    }

    println!("Slice 2:");

    for i in slice2 {
        println!("{}", i);
    }
}