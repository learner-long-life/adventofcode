fn main() {

    let number = 3;

    if number != 0 {
        println!("number was something other than zero")
    }

    let condition = true;
    let number = if condition { 3 } else { 5 };

    println!("The number is {}", number);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }

    };

    println!("The result is {}", result);

    let numbers = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < 5 {
        println!("Index element {}", numbers[index]);
        index += 1;
    }

    for i in numbers.iter() {
        println!("Index element {}", i);
    }

    for i in (1..4).rev() {
        println!("{}", i);
    }

    println!("LIFTOFF!");

}
