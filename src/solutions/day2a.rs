use std::io::BufRead;

pub fn main() {
    // read line by line from file inputs/2
    let file = std::fs::File::open("src/inputs/02.txt").unwrap();
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