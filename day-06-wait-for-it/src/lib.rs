use runner::*;
use std::iter::zip;

pub fn start(ctx: &mut Ctx) {
    let input = ctx.input();
    let boatraces = parse(input);
    part1(ctx, boatraces);
    ctx.update_timer(Ctx::PART1);

    part2(ctx);
    ctx.update_timer(Ctx::PART2);
}

#[derive(Debug)]
struct BoatRace {
    time: u32,
    distance: u32,
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
                .flat_map(|t| t.parse::<u32>().ok()),
            lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .skip(1)
                .flat_map(|t| t.parse::<u32>().ok()),
        )
        .map(|(time, distance)| BoatRace { time, distance })
        .collect(),
    )
}

fn part1(ctx: &mut Ctx, boatraces: BoatRaces) {
    let answer = 42;

    outputln!(ctx, "part 1: {}", answer);

}

fn part2(ctx: &mut Ctx) {
    let answer = 42;
    outputln!(ctx, "part 2: {}", answer);
}
