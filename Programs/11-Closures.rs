use  std::mem;

fn main() {
    let mut count = 0;

    //closure borrows count
    let mut add_one = || {
        count += 1;
        println!("`count`: {}", count);
    };

    add_one();

    //closure moves count
    let mut count2 = Box::new(0);

    let mut add_one_move = move || {
        *count2 += 1;
        println!("`count2`: {}", *count2);
        mem::drop(count2);
    };

    add_one_move();
}