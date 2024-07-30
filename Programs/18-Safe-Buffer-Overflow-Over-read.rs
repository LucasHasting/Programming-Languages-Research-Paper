fn main(){
    let mut buff: [u16; 6] = [3, 2, 7, 4, 11, 13];

    //buffer over-read
    println!("{}",buff[6]);

    //buffer over-flow
    buff[6] = 5;
}