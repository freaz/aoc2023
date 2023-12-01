use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufReader, Read},
};

fn replace_words_for_digits(data: &str) -> String {
    // xeightwo => x82

    let mut map = HashMap::new();
    map.insert("one", "o1ne");
    map.insert("two", "t2wo");
    map.insert("three", "t3hree");
    map.insert("four", "f4our");
    map.insert("five", "f5ive");
    map.insert("six", "s6ix");
    map.insert("seven", "s7even");
    map.insert("eight", "e8ight");
    map.insert("nine", "n9ine");

    let mut result = data.to_string();

    for (w, d) in map {
        result = result.replace(w, d);
    }

    result
}

fn calibration_values(data: &str) -> Vec<u32> {
    let mut two_digits: Vec<u32> = vec![];

    for line in data.lines() {
        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        two_digits.push(
            format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
                .parse()
                .unwrap(),
        );
    }

    two_digits
}

fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("calibration_data.txt");
    //path.push("example_data.txt");

    let file = File::open(path).expect("Cannot open file");
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader
        .read_to_string(&mut data)
        .expect("Cannot read the file");

    println!(
        "Result part one: {}",
        calibration_values(&data)
            .iter()
            .fold(0, |acc, &num| acc + num)
    );

    let data2 = replace_words_for_digits(&data);
    println!(
        "Result part two: {}",
        calibration_values(&data2)
            .iter()
            .fold(0, |acc, &num| acc + num)
    );
}
