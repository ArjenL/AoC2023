pub fn solve(input: &str) {
    part1(input);
    part2(input);
}

fn word_to_digit(s: &str) -> u32 {
    let num = match s {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!("Invalid digit"),
    };
    num
}

fn part1(input: &str) {
    let mut res: u32 = 0;
    let now = std::time::Instant::now();
    for line in input.lines() {
        let digits: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        let num = *digits.first().unwrap() as u32 * 10 + *digits.last().unwrap() as u32;
        res += num;
    }
    let elapsed = now.elapsed();
    println!("{res} ({elapsed:?})");
}

fn part2(input: &str) {
    let mut res: u32 = 0;

    let digits = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];

    let now = std::time::Instant::now();
    let mut found: Vec<(usize, &str)> = vec![];
    for line in input.lines() {
        for d in digits.iter() {
            found.extend(line.match_indices(d));
        }
        let first = found.iter().min_by_key(|e| e.0).unwrap().1;
        let last = found.iter().max_by_key(|e| e.0).unwrap().1;
        let num = word_to_digit(first) * 10 + word_to_digit(last);
        res += num;
        found.clear();
    }
    let elapsed = now.elapsed();
    println!("{res} ({elapsed:?})");
}
