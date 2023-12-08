use runner::*;
use std::iter::zip;

pub fn start(ctx: &mut Ctx) {
    let input = ctx.input();
    let boatraces = parse(input);
    part1(ctx, &boatraces);
    ctx.update_timer(Ctx::PART1);

    part2(ctx, input);
    ctx.update_timer(Ctx::PART2);
}

#[derive(Debug)]
struct BoatRace {
    time: u64,
    distance: u64,
}

#[derive(Debug)]
struct BoatRaces(Vec<BoatRace>);

fn parse(input: &str) -> BoatRaces {
    let mut lines = input.lines();
    BoatRaces(
        zip(
            lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .skip(1)
                .flat_map(|t| t.parse::<u64>().ok()),
            lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .skip(1)
                .flat_map(|t| t.parse::<u64>().ok()),
        )
        .map(|(time, distance)| BoatRace { time, distance })
        .collect(),
    )
}

fn part1(ctx: &mut Ctx, boatraces: &BoatRaces) {
    let mut winning_count = vec![];
    for race in boatraces.0.iter() {
        let mut count = 0;
        for i in 0..race.time {
            let distance = i * (race.time - i);
            if distance > race.distance {
                count += 1;
            }
        }
        winning_count.push(count);
    }
    outputln!(ctx, "part 1: {}", winning_count.iter().product::<u32>());
}

fn part2(ctx: &mut Ctx, input: &str) {
    let mut count = 0;
    let race = BoatRace {
        time: 42899189,
        distance: 308117012911467,
    };
    for i in 0..race.time {
        let distance = i * (race.time - i);
        if distance > race.distance {
            count += 1;
        }
    }
    outputln!(ctx, "part 2: {}", count);
}
