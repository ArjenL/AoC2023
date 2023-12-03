use runner::*;
use std::collections::{HashMap, HashSet};

pub fn start(ctx: &mut Ctx) {
    let input = ctx.input();

    part1(ctx, input);
    ctx.update_timer(Ctx::PART1);

    part2(ctx, input);
    ctx.update_timer(Ctx::PART2);
}

fn parse_line(
    line: &str,
    row: isize,
    tag: &mut u32,
    tag_to_partnum: &mut HashMap<u32, u32>,
    tag_coords: &mut HashMap<u32, Vec<(isize, isize)>>,
    coord_to_tag: &mut HashMap<(isize, isize), u32>,
    symbol_set: &mut HashSet<(isize, isize)>,
    symbols: &mut HashSet<char>,
    gears: &mut Vec<(isize, isize)>,
) {
    let mut num_buf = String::new();
    let mut handle_num = false;

    for (col, ch) in line.chars().enumerate() {
        if ch.is_ascii_digit() {
            handle_num = true;
            num_buf.push(ch);
            let coords = (row, col as isize);
            coord_to_tag.insert(coords, *tag);
            tag_coords.entry(*tag).or_insert(Vec::new()).push(coords);
        }
        else {
            if handle_num {
                let partnum = num_buf.parse::<u32>().unwrap();
                num_buf.clear();
                tag_to_partnum.insert(*tag, partnum);
                *tag += 1;
                handle_num = false;
            }
            if ch != '.' {
                if ch == '*' {
                    gears.push((row, col as isize));
                }
                symbols.insert(ch);
                symbol_set.insert((row, col as isize));
            }
        }
    }
    if handle_num {
        let partnum = num_buf.parse::<u32>().unwrap();
        tag_to_partnum.insert(*tag, partnum);
        *tag += 1;
    }
}

fn partnum_sum(
    tag_coords: &HashMap<u32, Vec<(isize, isize)>>,
    tag_to_partnum: &HashMap<u32, u32>,
    symbol_set: &HashSet<(isize, isize)>,
) -> u32 {
    let dirs: [isize; 3] = [-1, 0, 1];
    let mut tags: HashSet<u32> = HashSet::new();

    for (tag, coords) in tag_coords.iter() {
        for coord in coords.iter() {
            for &c in dirs.iter() {
                for &r in dirs.iter() {
                    if r == 0 && c == 0 {
                        continue;
                    }
                    if symbol_set.contains(&(coord.0 + r, coord.1 + c)) {
                        tags.insert(*tag);
                    }
                }
            }
        }
    }

    tags.iter().flat_map(|t| tag_to_partnum.get(t)).sum()
}

fn gear_ratios(
    tag_to_partnum: &HashMap<u32, u32>,
    coord_to_tag: &mut HashMap<(isize, isize), u32>,
    gears: &mut Vec<(isize, isize)>,
) -> u32 {
    let mut sum = 0;
    let dirs: [isize; 3] = [-1, 0, 1];
    let mut tags: HashSet<u32> = HashSet::new();

    for (row, col) in gears.iter() {
        for &r in dirs.iter() {
            for &c in dirs.iter() {
                if r == 0 && c == 0 {
                    continue;
                }
                if let Some(&tag) = coord_to_tag.get(&(row + r, col + c)) {
                    tags.insert(tag);
                }
            }
        }
        if tags.len() > 1 {
            sum += tags
                .drain()
                .flat_map(|k| tag_to_partnum.get(&k))
                .map(|&p| p)
                .product::<u32>();
        }
        else {
            tags.clear();
        }
    }
    sum
}

fn part1(ctx: &mut Ctx, input: &str) {
    let mut tag = 0;
    let mut tag_to_partnum = HashMap::new();
    let mut tag_coords = HashMap::new();
    let mut coord_to_tag = HashMap::new();
    let mut symbol_set = HashSet::new();
    let mut symbols = HashSet::new();
    let mut gears = Vec::new();

    for (row, line) in input.lines().enumerate() {
        parse_line(
            line,
            row as isize,
            &mut tag,
            &mut tag_to_partnum,
            &mut tag_coords,
            &mut coord_to_tag,
            &mut symbol_set,
            &mut symbols,
            &mut gears,
        );
    }

    let sum = partnum_sum(&tag_coords, &tag_to_partnum, &symbol_set);

    outputln!(ctx, "part 1: {}", sum);
}

fn part2(ctx: &mut Ctx, input: &str) {
    let mut tag = 0;
    let mut tag_to_partnum = HashMap::new();
    let mut tag_coords = HashMap::new();
    let mut coord_to_tag = HashMap::new();
    let mut symbol_set = HashSet::new();
    let mut symbols = HashSet::new();
    let mut gears = Vec::new();

    for (row, line) in input.lines().enumerate() {
        parse_line(
            line,
            row as isize,
            &mut tag,
            &mut tag_to_partnum,
            &mut tag_coords,
            &mut coord_to_tag,
            &mut symbol_set,
            &mut symbols,
            &mut gears,
        );
    }

    let sum = gear_ratios(&tag_to_partnum, &mut coord_to_tag, &mut gears);
    outputln!(ctx, "part 2: {}", sum);
}
