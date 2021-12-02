mod data;

/*
pub mod day00 {
    use std::fmt::Write;

    pub fn run() -> String {
        let mut result = String::with_capacity(128);
        /*
        let answer_part1 = part1(crate::data::DAY00);
        writeln!(&mut result, "Day 00, Problem 1 - [{}]", answer_part1).unwrap();

        let answer_part2 = part2(crate::data::DAY00);
        writeln!(&mut result, "Day 00, Problem 2 - [{}]", answer_part2).unwrap();
        */
        result
    }

    fn part1(_input: &str) -> usize {
        0
    }

    fn part2(_input: &str) -> usize {
        0
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn examples() {
        }

        #[test]
        fn verify() {
        }
    }
}
*/

mod util {
    pub trait StripCarriageReturn {
        fn strip_carriage_return(&self) -> String;
    }

    impl StripCarriageReturn for &str {
        fn strip_carriage_return(&self) -> String {
            self.replace("\r", "")
        }
    }
}

pub mod day01 {
    use std::fmt::Write;

    use itertools::Itertools;

    pub fn run() -> String {
        let mut result = String::with_capacity(128);

        let nums = parse_data(crate::data::DAY01);

        let answer_part1 = part1(&nums);
        writeln!(&mut result, "Day 01, Problem 1 - [{}]", answer_part1).unwrap();

        let answer_part2 = part2(&nums);
        writeln!(&mut result, "Day 01, Problem 2 - [{}]", answer_part2).unwrap();

        result
    }

    fn parse_data(data: &str) -> Vec<i32> {
        data.lines()
            .map(|line| line.parse::<i32>().unwrap())
            .collect()
    }

    fn part1(input: &[i32]) -> usize {
        input.iter().tuple_windows().filter(|(a, b)| b > a).count()
    }

    fn part2(input: &[i32]) -> usize {
        input
            .iter()
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .tuple_windows()
            .filter(|(a, b)| b > a)
            .count()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn examples() {
            let test_data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
            assert_eq!(part1(&test_data), 7);
            assert_eq!(part2(&test_data), 5);
        }

        #[test]
        fn verify() {
            let data = crate::data::DAY01;
            let nums = parse_data(data);
            assert_eq!(part1(&nums), 1696);
            assert_eq!(part2(&nums), 1737);
        }
    }
}

pub mod day02 {
    use std::fmt::Write;
    use fts_vecmath::vector2::*;

    type Vector = fts_vecmath::vector2::Vector2<i32>;

    pub fn run() -> String {
        let mut result = String::with_capacity(128);

        let data = parse_input(crate::data::DAY02);

        let answer_part1 = part1(&data);
        writeln!(&mut result, "Day 02, Problem 1 - [{}]", answer_part1).unwrap();

        let answer_part2 = part2(&data);
        writeln!(&mut result, "Day 02, Problem 2 - [{}]", answer_part2).unwrap();
        result
    }

    fn parse_input(input: &str) -> Vec<Vector> {
        input
            .lines()
            .map(|line| {
                let parts = line.split(" ").collect::<Vec<_>>();
                let dir = parts[0];
                let amount = parts[1].parse::<i32>().unwrap();

                match dir {
                    "forward" => Vector::new(amount, 0),
                    "up" => Vector::new(0, -amount),
                    "down" => Vector::new(0, amount),
                    _ => panic!("Failed to parse {}", line),
                }
            })
            .collect()
    }

    fn part1(data: &[Vector]) -> i32 {
        let sum : Vector = data.iter().fold(Vector::zero(), |acc, next| acc + *next);
        sum.x * sum.y
    }

    fn part2(data: &[Vector]) -> i32 {
        let mut pos = Vector::zero();
        let mut aim : i32 = 0;
        for point in data {
            pos.x += point.x;
            aim += point.y;
            pos.y += aim * point.x;
        }
        pos.x * pos.y
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn examples() {
            let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

            let data = super::parse_input(&input);
            assert_eq!(part1(&data), 150);
            assert_eq!(part2(&data), 900);
        }

        #[test]
        fn verify() {
            let data = parse_input(crate::data::DAY02);
            assert_eq!(part1(&data), 1840243);
            assert_eq!(part2(&data), 1727785422);
        }
    }
}
