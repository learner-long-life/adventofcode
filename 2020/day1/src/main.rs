use std::env;
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let filename = &args[1];

    println!("in file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text\n{}", contents);

    let number_strings: Vec<&str> = contents
        .split("\n")
        .collect();

    let numbers: Vec<i32> = number_strings
        .iter()
        .map(|&x| {
            if x.len() > 0 {
                return  x.parse::<i32>().unwrap();
            } else {
                return 0;
            }
        })
        .collect::<Vec<i32>>();

    // store whether we've found the bipartite complement in 2020
    let mut set : HashSet<i32> = HashSet::new();

    // Find a bi-partite product
    for i in numbers.iter() {
        if set.contains(i) {
            // we found the first past of our pair
            println!("Found {} and its complement {}",  i, (2020 - i));
            println!("Their product is {}", i * (2020-i));
        } else {
            set.insert(2020 - i);
        }
    }

    // Key is third number of the tuple (first, second) in the value
    let mut three_set : HashMap<i32,(i32,i32)> = HashMap::new();

    // Find a tri-partite product
    for i in 0..set.len() {
        let first = numbers[i];

        let result = three_set.get(&first);

        match result {
            Option::None => {
                for j in (i+1)..set.len() {
                    let second = numbers[j];
                    if first + second < 2020 {
                        three_set.insert(2020 - first - second, (first, second));
                    }
                }
        
            },
            Option::Some((x, y)) => {
                println!("Found triple {} {} {}", x, y, (2020 - x - y));
                println!("Their product is {}", x * y * (2020 - x - y));    
            }
        }

    }
}