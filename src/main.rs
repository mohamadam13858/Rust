fn add(num_one: i32, num_two: i32) -> i32 {
    num_one + num_two
}

fn main() {
    let add_number = add(10, 16);

    if add_number > 50 {
        println!("you are ready for free delivery")
    } else if add_number > 20 {
        println!("mohamad")
    }else {
        println!("else")
    }
}
