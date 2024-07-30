fn main() {

    //both loops display 1, 3, 5

    let mut count = 0_i8;

    while(count != 6){
        count += 1;
        if(count % 2 == 0){
            continue;
        }

        println!("{}", count);
    }

    count = 0_i8;
    
    loop{
        if(count == 6){
            break;
        }

        count += 1;

        if(count % 2 == 0){
            continue;
        }

        println!("{}", count);
    }

}