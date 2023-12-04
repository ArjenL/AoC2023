use runner::*;
use std::collections::HashSet;
use regex::Regex;

pub fn start(ctx: &mut Ctx) {
    let input = ctx.input();

    let data = parse(input);
    ctx.update_timer(Ctx::PARSING);

    part1(ctx, &data);
    ctx.update_timer(Ctx::PART1);

    part2(ctx, &data);
    ctx.update_timer(Ctx::PART2);
}

#[derive(Debug)]
struct Card {
    matches: usize,
}

fn parse(input: &str) -> Vec<Card> {
    let re = Regex::new(
        r"(?m)^Card\s+(?:[0-9]+):\s*((?:\s*[0-9]+\s*)+)\s+\|\s+((?:\s*[0-9]+\s*)+)$",
    )
    .unwrap();
    let mut cards: Vec<Card> = vec![];
    for (_, [winning, scratched]) in re.captures_iter(input).map(|c| c.extract())
    {
        let winning: HashSet<_> = winning
            .split_ascii_whitespace()
            .flat_map(|n| n.parse::<u32>().ok())
            .collect();
        let scratched: HashSet<_> = scratched
            .split_ascii_whitespace()
            .flat_map(|n| n.parse::<u32>().ok())
            .collect();
        let matches = winning.intersection(&scratched).count();
        cards.push(Card {
            matches,
        });
    }

    cards
}

fn part1(ctx: &mut Ctx, cards: &Vec<Card>) {
    let points: u32 = cards
        .iter()
        .map(|&Card { matches, .. }| {
            if matches > 0 {
                1 << (matches - 1)
            }
            else {
                0
            }
        })
        .sum();
    outputln!(ctx, "part 1: {}", points);
}

fn part2(ctx: &mut Ctx, cards: &Vec<Card>) {
    let mut instances: Vec<u32> = vec![1; cards.len()];

    for (i, c) in cards.iter().enumerate() {
        for j in i + 1..=i + c.matches {
            instances[j] += instances[i];
        }
    }
    let sum: u32 = instances.iter().sum();

    outputln!(ctx, "part 2: {}", sum);
}
