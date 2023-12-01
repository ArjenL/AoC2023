use clap::Parser;
use runner::Runner;
use std::time::Duration;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

const DAYS: &[(u32, &str, fn(&mut runner::Ctx))] = &[
    (1, "day-01-trebuchet", day_01::start)
];

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Options {
    /// Day to run, or "all"  for all days.
    #[arg(long, value_parser = day_parser)]
    day: u32,
    /// Part to run, if not set both parts are run.
    #[arg(long, value_parser = part_parser)]
    part: Option<u32>,
    /// Input file to use
    #[arg(long, default_value_t = String::from("input.txt"))]
    input: String,
    /// Benchmark.
    #[arg(long)]
    bench: bool,
    /// Buffer output instead of printing to stdout right away
    #[arg(long)]
    buffered: bool,
}

fn day_parser(s: &str) -> Result<u32, String> {
    if s == "all" {
        return Ok(0);
    }
    let day = s.parse::<u32>().map_err(|e| format!("{}", e))?;
    if day < 1 {
        return Err(format!("invalid day: {}", s));
    }
    Ok(day)
}

fn part_parser(s: &str) -> Result<u32, String> {
    let part = s.parse::<u32>().map_err(|e| format!("{}", e))?;
    if part < 1 || part > 2 {
        return Err(format!("invalid part: {}", s));
    }
    Ok(part)
}

fn main() {
    let opts = Options::parse();
    let mut tot_elapsed = Duration::from_secs(0);

    for (day, dir, func) in DAYS {
        if (opts.day == 0 && *day < 100) || opts.day == *day {
            let mut runner = Runner::new(dir, &opts.input, opts.bench, opts.buffered, func);
            runner.run();
            tot_elapsed += runner.elapsed();
        }
    }

    if opts.day == 0 {
        eprintln!("\nTotal time: {:?}", tot_elapsed);
    }
}
