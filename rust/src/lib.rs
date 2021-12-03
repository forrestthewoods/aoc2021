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
    use fts_vecmath::vector2::*;
    use std::fmt::Write;

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
        let sum: Vector = data.iter().fold(Vector::zero(), |acc, next| acc + *next);
        sum.x * sum.y
    }

    fn part2(data: &[Vector]) -> i32 {
        let mut pos = Vector::zero();
        let mut aim: i32 = 0;
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

pub mod day03 {
    use std::fmt::Write;

    pub fn run() -> String {
        let mut result = String::with_capacity(128);

        let (answer_part1, answer_part2) = solve(crate::data::DAY03);
        writeln!(&mut result, "Day 03, Problem 1 - [{}]", answer_part1).unwrap();
        writeln!(&mut result, "Day 03, Problem 2 - [{}]", answer_part2).unwrap();

        result
    }

    fn solve(input: &str) -> (u32, u32) {
        let len = input.lines().next().unwrap().len();
        let num_bits = len;

        // Parse
        let nums: Vec<u32> = input
            .lines()
            .map(|line| u32::from_str_radix(line, 2).unwrap())
            .collect();


        // Part 1
        let mut bit_counts: Vec<u32> = Default::default();
        bit_counts.resize(len, 0);

        let mut num_rows = 0;
        for line in input.lines() {
            num_rows += 1;
            for (idx, char) in line.chars().enumerate() {
                bit_counts[idx] += (char as u8 - '0' as u8) as u32;
            }
        }

        let mut gamma: u32 = 0;
        let mut epsilon: u32 = 0;
        let half_rows = (num_rows / 2) as u32;
        for bit_count in bit_counts {
            gamma <<= 1;
            epsilon <<= 1;

            if bit_count > half_rows {
                gamma += 1;
            } else {
                epsilon += 1;
            }
        }

        // Part 2
        let count_bits = |n: &[u32], mask: u32| -> usize {
            n.iter().filter(|&num| (num & mask) > 0).count()
        };

        let mut mask = 1 << (num_bits - 1);

        let mut oxygen_nums = nums.clone();
        while oxygen_nums.len() > 1 {
            let num_bits_set = count_bits(&oxygen_nums, mask);
            let len = oxygen_nums.len();
            let need_bit = num_bits_set >= (len - num_bits_set);
            println!("Len: [{}], NumSet: [{}]  Need bit: {}", oxygen_nums.len(), num_bits_set, need_bit);
            oxygen_nums = oxygen_nums.iter().filter(|&&num| {
                if need_bit {
                    (num & mask) > 0
                } else {
                    (num & mask) == 0
                }
            }).cloned().collect();
            mask >>= 1;
        }
        assert_eq!(oxygen_nums.len(), 1);
        let oxygen_num = oxygen_nums[0];
        println!("Oxygen num: {}", oxygen_num);



        mask = 1 << (num_bits - 1);

        let mut co2_nums = nums.clone();
        while co2_nums.len() > 1 {
            let num_bits_set = count_bits(&co2_nums, mask);
            let len = co2_nums.len();
            let need_bit = num_bits_set < (len - num_bits_set);
            println!("Len: [{}], NumSet: [{}]  Need bit: {}", co2_nums.len(), num_bits_set, need_bit);
            co2_nums = co2_nums.iter().filter(|&&num| {
                if need_bit {
                    (num & mask) > 0
                } else {
                    (num & mask) == 0
                }
            }).cloned().collect();
            mask >>= 1;
        }
        assert_eq!(co2_nums.len(), 1);
        let co2_num = co2_nums[0];
        println!("co2 num: {}", co2_num);

        /*
00100 = 4
11110 = 30
10110 = 22
10111 = 23
10101 = 21
01111 = 15
00111 = 7
11100 = 28
10000 = 16
11001 = 25
00010 = 2
01010 = 10
        */


        (gamma * epsilon, oxygen_num * co2_num)
    }

    fn part2(_input: &str) -> usize {
        0
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn examples() {
            let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

            let (answer_part1, answer_part2) = solve(input);

            assert_eq!(answer_part1, 198);
            assert_eq!(answer_part2, 230);
        }

        #[test]
        fn verify() {
            let (answer_part1, answer_part2) = solve(crate::data::DAY03);
            assert_eq!(answer_part1, 3885894);
            assert_eq!(answer_part2, 4375225);
        }
    }
}
