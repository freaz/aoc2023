use shared::{debug_print, load_file};

#[derive(Debug)]
struct Mapping {
    from: String,
    to: String,
    maps: Vec<Map>,
}
impl Mapping {
    fn map(&self, range: &Range) -> Vec<Range> {
        let mut ranges: Vec<Range> = vec![range.clone()];

        for map in &self.maps {
            let original_ranges = ranges.clone();
            ranges = Vec::new();
            for range in original_ranges {
                ranges.extend(map.map(&range));

                if map.overlaps(&range) {
                    return ranges;
                }
            }
        }

        ranges
    }
}

#[derive(Debug)]
struct Map {
    src: Range,
    dest: Range,
}
impl Map {
    fn overlaps(&self, range: &Range) -> bool {
        self.src.overlaps(range)
    }

    fn map(&self, range: &Range) -> Vec<Range> {
        let mut ranges: Vec<Range> = Vec::new();

        if let Some(before_range) = self.src.before_range(range) {
            ranges.push(before_range);
        }

        if let Some(overlap_range) = self.src.overlap_range(range) {
            ranges.push(Range {
                start: overlap_range.start - self.src.start + self.dest.start,
                end: overlap_range.end - self.src.start + self.dest.start,
            })
        }

        if let Some(after_range) = self.src.after_range(range) {
            ranges.push(after_range);
        }

        ranges
    }
}

#[derive(Debug, Clone)]
struct Range {
    start: usize,
    end: usize,
}
impl Range {
    fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    fn before_range(&self, other: &Range) -> Option<Range> {
        if other.start < self.start {
            Some(Range {
                start: other.start,
                end: std::cmp::min(self.start - 1, other.end),
            })
        } else {
            None
        }
    }

    fn overlap_range(&self, other: &Range) -> Option<Range> {
        if self.start <= other.end && other.start <= self.end {
            Some(Range {
                start: std::cmp::max(self.start, other.start),
                end: std::cmp::min(self.end, other.end),
            })
        } else {
            None
        }
    }

    fn after_range(&self, other: &Range) -> Option<Range> {
        if self.end < other.end {
            Some(Range {
                start: std::cmp::max(self.end + 1, other.start),
                end: other.end,
            })
        } else {
            None
        }
    }
}

fn main() {
    let data = load_file("input.txt");

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

    println!("Result one: {}\n", find_min_location(&seeds, &mappings));

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
        let mut ranges: Vec<Range> = vec![seed.clone()];

        debug_print!("{}: {:?}", mapping.unwrap().from, ranges);

        while mapping.is_some() {
            let original_ranges = ranges.clone();
            ranges = Vec::new();
            for range in &original_ranges {
                ranges = mapping.unwrap().map(&range);
            }
            debug_print!("{}: {:?}", mapping.unwrap().to, ranges);
            mapping = mappings.iter().find(|m| m.from == mapping.unwrap().to);
        }

        locations.extend(ranges);
    }

    debug_print!("Locations: {:?}", locations);
    locations.iter().map(|r| r.start).min().unwrap()
}
