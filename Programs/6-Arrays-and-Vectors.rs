fn main(){
    //create and edit an array
    let mut arr1: [u16; 6] = [3, 2, 7, 4, 11, 13];
    arr1[0] = 4;

    //set all elements a specific value, t has 10000 0s 
    let _arr2 = [0; 10000];

    //initialize the vector with values
    let vec1 = vec![3, 2, 7, 4, 11, 13];

    //create empty vector and push values onto it
    let mut vec2 = Vec::new();
    vec2.push(3);
    vec2.push(4);
    vec2.push(5);

    println!("ARRAY1 1:");
    //print array
    for i in arr1 {
        println!("{}", i);
    }

    println!("Vector 1:");
    //print vector
    for i in vec1 {
        println!("{}", i);
    }

    println!("Vector 2:");
    //another way to print 
    for i in 0..vec2.len() {
       println!("{}", vec2[i]);
    }
}