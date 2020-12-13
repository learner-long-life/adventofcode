use std::env;
use std::fs;
use regex::Regex;

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let filename = &args[1];

    println!("in file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text\n{}", contents);

    let password_lines: Vec<&str> = contents
        .split("\n")
        .collect();

    // 1-3 x: abcdef\n

    struct PasswordLine {
        minCount: i32,
        maxCount: i32,
        passChar: char,
        passWord: String,
    }

    let PASSWORD_LINE_RE = Regex::new(r"(\d)-(\d) (\w): (\w*)").unwrap();
    let PASSCHAR_RE = Regex::new(r"(\w)").unwrap();

    let caps = PASSWORD_LINE_RE.captures_iter(&contents);

    let mut invalid_count;

    for cap in caps {
        let min_count = cap[1].parse::<i32>().unwrap();
        let max_count = cap[2].parse::<i32>().unwrap();
        let pass_char = cap[3].parse::<char>().unwrap();
        let pass_word = String::from(&cap[4]);

        return Option::Some(PasswordLine {
            minCount: min_count,
            maxCount: max_count,
            passChar: pass_char,
            passWord: pass_word,
        });

    }
let numbers: Vec<Option<PasswordLine>> = password_lines
        .iter()
        .map(|&x| {
            
            let caps = PASSWORD_LINE_RE.captures_iter(x);

            for cap in caps {
                let min_count = cap[1].parse::<i32>().unwrap();
                let max_count = cap[2].parse::<i32>().unwrap();
                let pass_char = cap[3].parse::<char>().unwrap();
                let pass_word = String::from(&cap[4]);
                return Option::Some(PasswordLine {
                    minCount: min_count,
                    maxCount: max_count,
                    passChar: pass_char,
                    passWord: pass_word,
                });
    
            }
            return None;

        })
        .collect::<Vec<Option<PasswordLine>>>();
        
        //.collect::<Vec<i32>>();

    println!("Hello, world!");
}
