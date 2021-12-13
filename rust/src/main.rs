use structopt::StructOpt;

type DayFn = fn() -> String;

#[derive(StructOpt, Debug)]
#[structopt(name = "aoc2021")]
struct Options {
    #[structopt(short, long)]
    benchmark: bool,

    #[structopt(short, long)]
    all_days: bool,

    #[structopt(short, long)]
    single_day: Option<usize>,
}

fn main() {
    let opt = Options::from_args();

    let days = [
        // aoc2021::day01::run as DayFn,
        // aoc2021::day02::run as DayFn,
        // aoc2021::day03::run as DayFn,
        // aoc2021::day04::run as DayFn,
        // aoc2021::day05::run as DayFn,
        // aoc2021::day06::run as DayFn,
        // aoc2021::day07::run as DayFn,
        // aoc2021::day08::run as DayFn,
        // aoc2021::day09::run as DayFn,
        // aoc2021::day10::run as DayFn,
        // aoc2021::day11::run as DayFn,
        aoc2021::day12::run as DayFn,
        // aoc2021::day13::run as DayFn,
        // aoc2021::day14::run as DayFn,
        // aoc2021::day15::run as DayFn,
        // aoc2021::day16::run as DayFn,
        // aoc2021::day17::run as DayFn,
        // aoc2021::day18::run as DayFn,
        // aoc2021::day19::run as DayFn,
        // aoc2021::day20::run as DayFn,
        // aoc2021::day21::run as DayFn,
        // aoc2021::day22::run as DayFn,
        // aoc2021::day23::run as DayFn,
        // aoc2021::day24::run as DayFn,
        // aoc2021::day25::run as DayFn,
    ];

    let start = std::time::Instant::now();

    days.iter()
        .enumerate()
        .filter(|(idx, _)| opt.all_days || *idx == opt.single_day.map_or(days.len() - 1, |v| v - 1))
        .for_each(|(_, day_fn)| {
            let result = day_fn();
            if !opt.benchmark {
                println!("{}", result);
            }
        });

    println!("Completed in: {:?}", start.elapsed());
}
