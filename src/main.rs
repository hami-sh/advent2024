use std::io::BufRead;

fn main() {
    // get int from command line arg
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let input: i32 = args[1].parse().unwrap();
        match input {
            1 => one(),
            _ => println!("Invalid input"),
        }
        return;
    }
}

fn one() {
    // read line by line from file inputs/1
    let file = std::fs::File::open("src/inputs/1").unwrap();
    let reader = std::io::BufReader::new(file);
    let mut first_array: Vec<i32> = Vec::new();
    let mut second_array: Vec<i32> = Vec::new();
    reader.lines().for_each(|line| {
        let line = line.unwrap();
        println!("{}", line);
        let words = line.split("   ").collect::<Vec<&str>>();
        let first: i32 = words[0].parse().unwrap();
        let second: i32 = words[1].parse().unwrap();
        first_array.push(first);
        second_array.push(second);
    });

    // sort both arrays
    first_array.sort();
    second_array.sort();

    // loop over both arrays 
    let mut total_distance = 0;
    for i in 0..first_array.len() {
        let distance = (first_array[i] - second_array[i]).abs();
        total_distance += distance;
    }
    println!("{}", total_distance);
}