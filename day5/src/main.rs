use shared::load_file;

#[derive(Debug)]
struct Mapping {
    from: String,
    to: String,
    maps: Vec<Map>,
}
impl Mapping {
    fn map(&self, range: &Range) -> Range {
        for map in &self.maps {
            if let Some(r) = map.map(range) {
                return r;
            }
        }

        range.to_owned()
    }
}

#[derive(Debug)]
struct Map {
    src: Range,
    dest: Range,
}
impl Map {
    fn map(&self, range: &Range) -> Option<Range> {
        // ok this isn't enough and ranges need to be splitted to what is in mapped, before and after mapped range
        // unfrotunatelly I am out of time
        if self.src.overlap(range) {
            let range = self.src.overlap_range(range);
            Some(Range {
                start: range.start - self.src.start + self.dest.start,
                end: range.end - self.src.start + self.dest.start,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Range {
    start: usize,
    end: usize,
}
impl Range {
    fn overlap(&self, other: &Range) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    fn overlap_range(&self, other: &Range) -> Range {
        Range {
            start: std::cmp::max(self.start, other.start),
            end: std::cmp::min(self.end, other.end),
        }
    }
}

fn main() {
    let data = load_file("example.txt");

    let mut seeds: Vec<Range> = Vec::new();
    let mut mappings: Vec<Mapping> = Vec::new();

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("seeds") {
            seeds = line
                .replace("seeds: ", "")
                .split_whitespace()
                .map(|ch| {
                    let num = ch.parse().expect("Valid number");
                    Range {
                        start: num,
                        end: num,
                    }
                })
                .collect();
            continue;
        }

        if line.contains("map") {
            let head = line.replace(" map:", "");
            let parts: Vec<String> = head.split("-").map(|s| s.to_string()).collect();
            let from = parts[0].to_owned();
            let to = parts[2].to_owned();

            mappings.push(Mapping {
                from,
                to,
                maps: Vec::new(),
            });

            continue;
        }

        let parts: Vec<usize> = line
            .split_whitespace()
            .map(|ch| ch.parse().expect("Valid nuber"))
            .collect();

        let dest = parts[0];
        let src = parts[1];
        let len = parts[2];

        if let Some(current_mapping) = mappings.last_mut() {
            current_mapping.maps.push(Map {
                src: Range {
                    start: src,
                    end: src + len - 1,
                },
                dest: Range {
                    start: dest,
                    end: dest + len - 1,
                },
            });
        }
    }

    println!("Result one: {}", find_min_location(&seeds, &mappings));

    let seed_ranges = seeds
        .clone()
        .chunks(2)
        .map(|r| Range {
            start: r[0].start,
            end: r[0].start + r[1].start - 1,
        })
        .collect();
    println!("Result two: {}", find_min_location(&seed_ranges, &mappings));
}

fn find_min_location(seeds: &Vec<Range>, mappings: &Vec<Mapping>) -> usize {
    let mut locations: Vec<Range> = Vec::new();
    for seed in seeds {
        let mut mapping = Some(mappings.iter().find(|m| m.from == "seed").unwrap());
        let mut value: Range = seed.clone();

        while mapping.is_some() {
            value = mapping.unwrap().map(&mut value);
            mapping = mappings.iter().find(|m| m.from == mapping.unwrap().to);
        }

        locations.push(value);
    }
    locations.iter().map(|r| r.start).min().unwrap()
}
