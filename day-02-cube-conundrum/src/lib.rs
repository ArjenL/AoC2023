use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete,
        complete::{multispace0, space1},
    },
    combinator::map,
    error::Error,
    multi::{many0, separated_list1},
    sequence::{delimited, pair, preceded, terminated},
    Finish,
    IResult,
};
use runner::*;
use std::cmp::max;

#[derive(Debug)]
enum CubeColor {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Clone, Debug, Default)]
struct GameSet {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Clone, Debug)]
struct Game {
    id: u32,
    sets: Vec<GameSet>,
}

impl Game {
    fn power(&self) -> u32 {
        let (r, g, b) = self.sets.iter().fold((0, 0, 0), |s, acc| {
            (max(s.0, acc.red), max(s.1, acc.green), max(s.2, acc.blue))
        });

        r * g * b
    }
}

#[derive(Debug)]
struct Games(Vec<Game>);

impl Games {
    fn from_str(input: &str) -> Self {
        let games = match parse_games(input) {
            Ok((_, g)) => g,
            Err(_) => Self(vec![]),
        };
        games
    }

    fn possibles(
        &self,
        red: u32,
        green: u32,
        blue: u32,
    ) -> Box<dyn Iterator<Item = Game> + '_> {
        Box::new(
            self.0
                .iter()
                .filter(move |g| {
                    !g.sets
                        .iter()
                        .any(|s| s.red > red || s.green > green || s.blue > blue)
                })
                .cloned(),
        )
    }

    fn powersum(&self) -> u32 { self.0.iter().map(|g| g.power()).sum() }
}

fn parse_games(input: &str) -> IResult<&str, Games> {
    let (input, games) = many0(terminated(parse_game, multispace0))(input)?;
    Ok((input, Games(games))).finish()
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) =
        delimited(tag("Game "), complete::u32::<&str, Error<_>>, tag(": "))(input)?;
    let (input, raw_sets) = separated_list1(
        tag("; "),
        separated_list1(
            tag(", "),
            map(
                pair(
                    complete::u32::<&str, Error<_>>,
                    preceded(space1, alt((tag("red"), tag("green"), tag("blue")))),
                ),
                |(count, color)| {
                    let c = match color {
                        "red" => CubeColor::Red(count),
                        "green" => CubeColor::Green(count),
                        "blue" => CubeColor::Blue(count),
                        _ => panic!("Invalid color"),
                    };
                    c
                },
            ),
        ),
    )(input)?;

    let sets = raw_sets
        .iter()
        .map(|rs| {
            rs.iter().fold(GameSet::default(), |g, e| match e {
                CubeColor::Red(n) => GameSet {
                    red: g.red + n,
                    green: g.green,
                    blue: g.blue,
                },
                CubeColor::Green(n) => GameSet {
                    red: g.red,
                    green: g.green + n,
                    blue: g.blue,
                },
                CubeColor::Blue(n) => GameSet {
                    red: g.red,
                    green: g.green,
                    blue: g.blue + n,
                },
            })
        })
        .collect();
    Ok((input, Game { id, sets }))
}

pub fn start(ctx: &mut Ctx) {
    let input = ctx.input();

    part1(ctx, input);
    ctx.update_timer(Ctx::PART1);

    part2(ctx, input);
    ctx.update_timer(Ctx::PART2);
}

fn part1(ctx: &mut Ctx, input: &str) {
    let g = Games::from_str(&input);
    let answer = g.possibles(12, 13, 14).map(|game| game.id).sum::<u32>();
    outputln!(ctx, "part 1: {}", answer);
}

fn part2(ctx: &mut Ctx, input: &str) {
    let g = Games::from_str(&input);
    let answer = g.powersum();
    outputln!(ctx, "part 2: {}", answer);
}
