use std::{
    env,
    fs::File,
    io::{BufReader, Read},
};

use itertools::Itertools;

#[derive(Debug)]
struct Game {
    id: u32,
    takes: Vec<GameTake>,
}

#[derive(Debug)]
struct GameTake {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let games = load_games();

    let red_cubes: u32 = 12;
    let green_cubes: u32 = 13;
    let blue_cubers: u32 = 14;

    let mut valid_games: Vec<u32> = vec![];

    for game in &games {
        let all_takes_valid = game.takes.iter().all(|take| {
            take.red <= red_cubes && take.green <= green_cubes && take.blue <= blue_cubers
        });

        if all_takes_valid {
            valid_games.push(game.id)
        }
    }

    println!(
        "Result one: {}",
        valid_games.iter().fold(0, |acc, id| acc + id)
    );

    let mut take_values: Vec<u32> = vec![];

    for game in &games {
        let mut reds: u32 = 0;
        let mut greens: u32 = 0;
        let mut blues: u32 = 0;

        for take in &game.takes {
            if reds < take.red {
                reds = take.red
            }

            if greens < take.green {
                greens = take.green
            }

            if blues < take.blue {
                blues = take.blue
            }
        }

        take_values.push(reds * greens * blues);
    }

    println!(
        "Result two: {}",
        take_values.iter().fold(0, |acc, id| acc + id)
    );
}

fn load_games() -> Vec<Game> {
    let mut path = env::current_dir().unwrap();
    path.push("input_data.txt");

    let file = File::open(path).expect("Cannot open file");
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader
        .read_to_string(&mut data)
        .expect("Cannot read the file");

    let mut games: Vec<Game> = vec![];

    for line in data.lines() {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

        let line_split: Vec<&str> = line.split(':').collect();
        let game_header = line_split[0];
        let takes = line_split[1];

        let game_id: u32 = game_header.split_whitespace().collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();

        let mut game = Game {
            id: game_id,
            takes: vec![],
        };

        for take in takes.split(';').map(|t| t.trim()) {
            let mut game_take = GameTake {
                red: 0,
                green: 0,
                blue: 0,
            };

            for cubes in take.split(',').map(|s| s.trim()) {
                let (count, color) = cubes.split_whitespace().collect_tuple().unwrap();
                let count: u32 = count.parse().unwrap();
                match color {
                    "red" => game_take.red = count,
                    "green" => game_take.green = count,
                    "blue" => game_take.blue = count,
                    _ => {}
                }
            }

            game.takes.push(game_take);
        }
        games.push(game);
    }

    games
}
