use std::io::BufRead;

pub fn main() {
    let file = std::fs::File::open("src/inputs/01.txt").unwrap();
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

    let mut map = std::collections::HashMap::new();
    for i in 0..second_array.len() {
        // create new key if doesnt exist or add 1 to value of existing key
        let count = map.entry(second_array[i]).or_insert(0);
        *count += 1;
    }

    let mut result = 0;
    for i in 0..first_array.len() {
        let key = first_array[i];
        if map.contains_key(&key) {
            let value = map.get(&key).unwrap();
            let score = key * value;
            result += score;
            continue;
        }
    }

    println!("{}", result);

    
}