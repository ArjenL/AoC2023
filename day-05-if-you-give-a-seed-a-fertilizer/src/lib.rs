use std::ops::{Range, RangeBounds};

use runner::*;

pub fn start(ctx: &mut Ctx) {
    let input = ctx.input();

    let almanac = parse(input);
    ctx.update_timer(Ctx::PARSING);

    part1(ctx, &almanac);
    ctx.update_timer(Ctx::PART1);

    part2(ctx, &almanac);
    ctx.update_timer(Ctx::PART2);
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    mappings: Vec<Mapping>,
}

impl Almanac {
    fn destination(&self, source: u64) -> u64 {
        let mut source = source;
        for m in &self.mappings {
            source = m.destination(source);
        }
        source
    }
}

#[derive(Debug)]
struct Mapping {
    source: Vec<u64>,
    destination: Vec<u64>,
    range: Vec<u64>,
    map: Vec<std::ops::Range<u64>>,
}

impl Mapping {
    fn new() -> Self {
        Self {
            source: vec![],
            destination: vec![],
            range: vec![],
            map: vec![],
        }
    }

    fn add_map(&mut self, source: u64, destination: u64, range: u64) {
        self.source.push(source);
        self.destination.push(destination);
        self.range.push(range);
        self.map.push(std::ops::Range {
            start: source,
            end: source + range,
        });
    }

    pub fn destination(&self, source: u64) -> u64 {
        for (i, m) in self.map.iter().enumerate() {
            if m.contains(&source) {
                return self.destination[i] + source - self.source[i];
            }
        }
        source
    }
}

fn parse(input: &str) -> Almanac {
    let chunks = input.split_terminator("\n\n").collect::<Vec<_>>();

    // First line contains the seeds
    let seeds: Vec<_> = chunks[0]
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|n| n.parse::<u64>().ok())
        .collect();

    let mut mappings = Vec::new();
    // Each next chunk is a mapping
    for c in chunks.iter().skip(1) {
        let mut mapping = Mapping::new();
        for line in c.lines().skip(1) {
            let nums = line
                .split_ascii_whitespace()
                .flat_map(|n| n.parse::<u64>().ok())
                .collect::<Vec<_>>();
            mapping.add_map(nums[1], nums[0], nums[2]);
        }
        mappings.push(mapping);
    }

    let almanac = Almanac { seeds, mappings };

    almanac
}

fn part1(ctx: &mut Ctx, almanac: &Almanac) {
    let nearest_location =
        almanac
            .seeds
            .iter()
            .map(|&s| almanac.destination(s))
            .min()
            .unwrap();

    outputln!(ctx, "part 1: {}", nearest_location);
}

fn part2(ctx: &mut Ctx, almanac: &Almanac) {
    let mut locations: Vec<u64> = vec![];
    for c in almanac.seeds.chunks(2) {
            locations.push(
                (c[0]..c[0] + c[1])
                    .map(|v| almanac.destination(v))
                    .min()
                    .unwrap(),
            );
    }
    let nearest_location = locations.iter().min().unwrap();

    outputln!(ctx, "part 2: {}", nearest_location);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mapping() {
        let mut m = Mapping::new();
        m.add_map(50, 52, 48);
        assert_eq!(m.destination(50), 52);
        assert_eq!(m.destination(60), 62);
        assert_eq!(m.destination(98), 98);
        assert_eq!(m.destination(97), 99);
    }
}
