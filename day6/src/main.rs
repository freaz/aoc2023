use std::usize;

use shared::{debug_print, load_file};

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}
impl Race {
    fn number_of_ways_to_beat_record(&self) -> usize {
        let mut ways = 0;
        for charge_time in 0..self.time {
            if charge_time * (self.time - charge_time) > self.distance {
                ways += 1;
            }
        }
        ways
    }
}

fn parse_line(line: &str) -> Vec<usize> {
    line.split_whitespace()
        .map(|t| t.trim().parse::<usize>().unwrap())
        .collect()
}

fn main() {
    let data = load_file("input.txt");
    let lines: Vec<&str> = data.lines().collect();

    let times: Vec<usize> = parse_line(&lines[0].replace("Time:", ""));
    debug_print!("times: {:?}", times);
    let distances: Vec<usize> = parse_line(&lines[1].replace("Distance:", ""));
    debug_print!("distances: {:?}", distances);

    let mut races: Vec<Race> = Vec::new();
    for i in 0..times.len() {
        races.push(Race {
            time: times[i],
            distance: distances[i],
        });
    }
    debug_print!("races: {:?}", races);

    let mut ways = 1;
    for race in &races {
        let race_ways = race.number_of_ways_to_beat_record();
        debug_print!("Race ways: {}", race_ways);
        ways *= race_ways;
    }
    println!("Result one: {}", ways);

    let time = lines[0]
        .replace("Time:", "")
        .replace(" ", "")
        .parse()
        .unwrap();
    let distance = lines[1]
        .replace("Distance: ", "")
        .replace(" ", "")
        .parse()
        .unwrap();
    let race = Race { time, distance };
    debug_print!("Race: {:?}", race);

    println!("Result two: {}", race.number_of_ways_to_beat_record());
}
