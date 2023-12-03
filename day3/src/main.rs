use std::{
    env,
    fs::File,
    io::{BufReader, Read},
};

type Schematics = Vec<Vec<char>>;

#[derive(Debug)]
struct PartNumber {
    chars: Vec<char>,
    symbols: Vec<Symbol>,
}
impl PartNumber {
    fn new() -> PartNumber {
        PartNumber {
            chars: Vec::new(),
            symbols: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Symbol {
    value: char,
    x: usize,
    y: usize,
}

fn load_schematics(filename: &str) -> Schematics {
    let mut path = env::current_dir().unwrap();
    path.push(filename);

    let file = File::open(path).expect("Cannot open file");
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader
        .read_to_string(&mut data)
        .expect("Cannot read the file");

    data.lines().map(|l| l.chars().collect()).collect()
}

fn main() {
    let schematics = load_schematics("schematics_data.txt");
    //let schematics = load_schematics("example_data.txt");
    let xm = schematics.len();
    let ym = schematics[0].len();
    let mut part_nums: Vec<PartNumber> = vec![];

    for x in 0..xm {
        let mut part_num = PartNumber::new();
        for y in 0..ym {
            let ch = schematics[x][y];
            if ch.is_digit(10) {
                part_num.chars.push(ch);
                for (nx, ny) in neighbours(x, y, xm, ym) {
                    let nch = schematics[nx][ny];
                    if nch.is_digit(10) == false && nch != '.' {
                        let is_in = part_num
                            .symbols
                            .iter()
                            .filter(|s| s.x == nx || s.y == ny)
                            .count()
                            > 0;

                        if is_in == false {
                            part_num.symbols.push(Symbol {
                                value: nch,
                                x: nx,
                                y: ny,
                            });
                        }
                    }
                }
            } else {
                if part_num.chars.len() > 0 {
                    part_nums.push(part_num);
                }
                part_num = PartNumber::new();
            }
        }
        if part_num.chars.len() > 0 {
            part_nums.push(part_num);
        }
    }

    let result1 = part_nums
        .iter()
        .filter(|n| n.symbols.len() > 0)
        .map(|n| n.chars.iter().collect::<String>().parse::<usize>().unwrap())
        .fold(0, |acc, n| acc + n);

    println!("Result one: {}", result1);

    let mut gears: Vec<Vec<Vec<usize>>> = vec![vec![vec![]; schematics[0].len()]; schematics.len()];

    for part_num in part_nums.iter().filter(|n| n.symbols.len() > 0) {
        for symbol in &part_num.symbols {
            if symbol.value == '*' {
                gears[symbol.x][symbol.y].push(
                    part_num
                        .chars
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap(),
                )
            }
        }
    }

    let result2 = gears
        .into_iter()
        .flat_map(|x| x)
        .filter(|s| s.len() == 2)
        .map(|f| f.iter().fold(1, |acc, g| acc * g))
        .fold(0, |acc, r| acc + r);

    println!("Result two: {}", result2);
}

fn neighbours(x: usize, y: usize, xm: usize, ym: usize) -> Vec<(usize, usize)> {
    let mut n = vec![];

    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < xm as isize && ny >= 0 && ny < ym as isize {
                n.push((nx as usize, ny as usize));
            }
        }
    }

    n
}
