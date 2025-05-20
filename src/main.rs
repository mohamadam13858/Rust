fn add(num_one: i32, num_two: i32) -> i32 {
    num_one + num_two
}

fn main() {
    let mut add_number = add(10, 15);
    let free_delivery = false;

    if add_number > 50 {
        println!("you are ready for free delivery")
    } else if add_number > 20 {
        println!("mohamad")
    } else {
        println!("else")
    };

    add_number = match free_delivery {
        true => add_number + 0,
        false => add_number + 5,
    };

    match add_number {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("no match found"),
    }

    println!("{:?}", add_number);

    let items: [i32; 5] = [1, 2, 3, 4, 5];

    // println!("{:?}", items[0]);

    let mut my_vector: Vec<i32> = vec![1, 2, 3, 4, 5];

    let mut my_vector2 = Vec::new();

    my_vector2.push(1);
    my_vector2.push(2);
    my_vector2.push(3);
    my_vector2.push(4);
    my_vector2.push(10);

    my_vector.push(6);


    println!("{:?}", my_vector)
}
