use std::io::{BufRead, Read};

pub fn main() {
    let file = std::fs::File::open("src/inputs/03.txt").unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();
    // println!("{}", buf);

    let mut allow = false;

    let mut result = 0;
    for i in 0..buf.len() {
        // println!("i: {}", i);
        // get next 4 characters
        let mut first_four_char = buf.chars().skip(i).take(4).collect::<String>();
        if first_four_char != "mul(" {
            // println!("ignore {}", first_four_char);
            continue;
        }
        println!("found mul( -- rest of line: {}", buf.chars().skip(i).take(20).collect::<String>());
        let mut j = i + 4;
        // read char until ","
        let mut first_num = String::new();
        while buf.chars().nth(j).unwrap() != ',' {
            first_num.push(buf.chars().nth(j).unwrap());
            j += 1;
        }
        // println!("first_num: {}", first_num);
        // read char until ")"
        j += 1;
        let mut second_num = String::new();
        while buf.chars().nth(j).unwrap() != ')' {
            second_num.push(buf.chars().nth(j).unwrap());
            j += 1;
        }
        println!("second_num: {}", second_num);

        // convert to integers, continue if failed
        let maybe_first: Result<i32, _> = first_num.parse();
        if maybe_first.is_err() {
            println!("failed to parse first_num, {first_num}");
            continue;
        }
        let first = maybe_first.unwrap();
        let maybe_second: Result<i32, _> = second_num.parse();
        if maybe_second.is_err() {
            println!("failed to parse second_num, {second_num}");
            continue;
        }
        let second = maybe_second.unwrap();
        result += first * second;
        println!("{} * {} = {}", first, second, first * second);
    }
    println!("{}", result);
}