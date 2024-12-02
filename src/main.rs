use std::io::BufRead;

fn main() {
    // get int from command line arg
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let input: i32 = args[1].parse().unwrap();
        match input {
            1 => one(),
            2 => two(),
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

fn two() {
    // read line by line from file inputs/2
    let file = std::fs::File::open("src/inputs/2").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut array: Vec<Vec<i32>> = Vec::new();
    reader.lines().for_each(|line| {
        let line = line.unwrap();
        let mut row: Vec<i32> = Vec::new();
        line.split(" ").for_each(|word| {
            let number: i32 = word.parse().unwrap();
            row.push(number);
        });
        array.push(row);
    });

    let mut num_safe = 0;
    array.into_iter().for_each(|row| {
        println!("{:?}", row);
        let decreasing: bool;

        if row[0] < row[1] {
            decreasing = false;
        } else {
            decreasing = true;
        }

        let mut safe = true;
        for i in 1..row.len() {
            let current = row[i];
            let previous = row[i - 1];
            let diff = current - previous;

            println!(
                "Current: {}, Previous: {}, Diff: {}",
                current, previous, diff
            );

            if diff < 0 && !decreasing {
                println!("Not safe -- increasing, diff negative \n");
                safe = false;
                break;
            } else if diff > 0 && decreasing {
                println!("Not safe -- decreasing, diff positive \n");
                safe = false;
                break;
            }
            if diff.abs() < 1 || diff.abs() > 3 {
                println!("Not safe -- diff outside range 1-3 \n");
                safe = false;
                break;
            }
        }
        if safe {
            println!("Safe \n");
            num_safe += 1;
        }
    });

    println!("{}", num_safe);
}
