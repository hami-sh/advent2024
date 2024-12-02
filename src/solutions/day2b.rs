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
    for row in array {
        println!("{:?}", row);

        let (safe, reached_index) = is_row_safe(&row);
        if safe {
            println!("Safe \n");
            num_safe += 1;
        } else {
            // Try removing each number one at a time to see if it makes the sequence safe
            if (0..row.len()).any(|i| {
                let mut modified = row.clone();
                modified.remove(i);
                is_row_safe(&modified).0
            }) {
                num_safe += 1;
            }
        }
    }

    println!("{}", num_safe);
}

fn is_row_safe(row: &Vec<i32>) -> (bool, usize) {
    let decreasing: bool;
    if row[0] < row[row.len() - 1] {
        decreasing = false;
    } else {
        decreasing = true;
    }

    let mut safe = true;
    let mut reached_index = 0;
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
            reached_index = i;
            break;
        } else if diff > 0 && decreasing {
            println!("Not safe -- decreasing, diff positive \n");
            safe = false;
            reached_index = i;
            break;
        }
        if diff.abs() < 1 || diff.abs() > 3 {
            println!("Not safe -- diff outside range 1-3 \n");
            safe = false;
            break;
        }
    }
    (safe, reached_index)
}
