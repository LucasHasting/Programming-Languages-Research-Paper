fn main(){
    //printing s causes an error, the value has been moved to k
    let mut s = "John Doe".to_string();
    let k = s;
    //println!("k: {}, s: {}", k, s) 

    //problem fixed
    let s = "John Doe".to_string();
    let k = s.clone();
    println!("k: {}, s: {}", k, s) 
}