use std::collections::HashSet;

use shared::load_file;

#[derive(Debug)]
struct Card {
    matching: usize,
    points: usize,
}

impl Card {
    fn new(line: &str) -> Card {
        let parts: Vec<&str> = line.split(':').collect();
        let parts: Vec<&str> = parts[1].split("|").collect();

        let winning: HashSet<usize> = parts[0]
            .split_whitespace()
            .map(|num| num.trim().parse().unwrap())
            .collect();
        let numbers: HashSet<usize> = parts[1]
            .split_whitespace()
            .map(|num| num.trim().parse().unwrap())
            .collect();
        let matching = winning.intersection(&numbers).count();
        let points = if matching <= 1 {
            matching
        } else {
            2usize.pow((matching - 1) as u32)
        };

        Card { matching, points }
    }
}

fn main() {
    let data = load_file("input.txt");

    let mut sum = 0;
    let mut instances: Vec<usize> = vec![1; data.lines().count()];
    for (i, line) in data.lines().enumerate() {
        let card = Card::new(line);
        sum = sum + card.points;

        for j in 1..card.matching + 1 {
            instances[i + j] += instances[i];
        }
    }
    println!("Result one: {}", sum);
    println!("Result two: {}", instances.iter().fold(0, |acc, c| acc + c));
}
