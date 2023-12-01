use runner::*;

use itertools::{
    Itertools,
    MinMaxResult::{MinMax, NoElements, OneElement},
};

pub fn start(ctx: &mut Ctx) {
    let input = ctx.input();

    part1(ctx, input);
    ctx.update_timer(Ctx::PART1);

    part2(ctx, input);
    ctx.update_timer(Ctx::PART2);
}

fn part1(ctx: &mut Ctx, input: &str) {
    let mut answer: u32 = 0;
    for line in input.lines() {
        let digits: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        let num = *digits.first().unwrap() as u32 * 10 + *digits.last().unwrap() as u32;
        answer += num;
    }
    outputln!(ctx, "part 1: {}", answer);
}

fn part2(ctx: &mut Ctx, input: &str) {
    let digits = &[
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9),
    ];

    let answer = input.lines().fold(0, |acc, line| {
        acc + match digits
            .iter()
            .flat_map(|d| line.match_indices(d.0).map(|i| (i.0, d.1)))
            .minmax_by_key(|e| e.0)
        {
            MinMax(first, last) => first.1 * 10 + last.1,
            OneElement(only) => only.1 * 10 + only.1,
            NoElements => 0,
        }
    });

    outputln!(ctx, "part 2: {}", answer);
}
