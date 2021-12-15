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
            let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

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

        let (nums, num_bits) = parse_input(crate::data::DAY03);

        let answer_part1 = part1(&nums, num_bits);
        writeln!(&mut result, "Day 03, Problem 1 - [{}]", answer_part1).unwrap();

        let answer_part2 = part2(&nums, num_bits);
        writeln!(&mut result, "Day 03, Problem 2 - [{}]", answer_part2).unwrap();

        result
    }

    fn parse_input(input: &str) -> (Vec<u16>, usize) {
        let nums = input
            .lines()
            .map(|line| u16::from_str_radix(line, 2).unwrap())
            .collect();
        let num_bits = input.lines().next().unwrap().len();

        (nums, num_bits)
    }

    fn part1(nums: &[u16], num_bits: usize) -> usize {
        let half_len = nums.len() / 2;

        let mut gamma = 0;
        for bit in 0..num_bits {
            let mask = 1 << bit;
            let count = nums.iter().filter(|&&num| (num & mask) > 0).count();
            gamma |= mask * (count > half_len) as u16;
        }

        let mask = (0..num_bits - 1).fold(1, |acc, _| (acc << 1) + 1);

        let epsilon = !gamma & mask;
        gamma as usize * epsilon as usize
    }

    fn part2(nums: &[u16], num_bits: usize) -> usize {
        let calc = |invert: bool| -> usize {
            let mut nums = nums.to_vec();
            let mut mask = 1 << (num_bits - 1);
            while nums.len() > 1 {
                let count = nums.iter().filter(|&num| (num & mask) > 0).count();
                let needs_bit = (count >= (nums.len() - count)) ^ invert;
                nums.retain(|num| ((num & mask) > 0) == needs_bit);
                mask >>= 1;
            }
            assert_eq!(nums.len(), 1);
            nums[0] as usize
        };

        let oxygen = calc(false);
        let co2 = calc(true);

        oxygen * co2
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn examples() {
            let input = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

            let (nums, num_bits) = parse_input(input);
            assert_eq!(part1(&nums, num_bits), 198);
            assert_eq!(part2(&nums, num_bits), 230);
        }

        #[test]
        fn verify() {
            let (nums, num_bits) = parse_input(crate::data::DAY03);
            assert_eq!(part1(&nums, num_bits), 3885894);
            assert_eq!(part2(&nums, num_bits), 4375225);
        }
    }
}

pub mod day04 {
    use std::fmt::Write;

    type Moves = Vec<u8>;
    type Tile = (u8, bool);
    type Board = Vec<Tile>;

    pub fn run() -> String {
        let mut result = String::with_capacity(128);

        let (moves, boards) = parse_input(crate::data::DAY04);

        let answer_part1 = part1(&moves, boards.clone());
        writeln!(&mut result, "Day 04, Problem 1 - [{}]", answer_part1).unwrap();

        let answer_part2 = part2(&moves, boards);
        writeln!(&mut result, "Day 04, Problem 2 - [{}]", answer_part2).unwrap();

        result
    }

    fn parse_input(input: &str) -> (Moves, Vec<Board>) {
        // Read moves from first line
        let first_line = input.lines().next().unwrap();
        let moves: Vec<u8> = input
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|num| num.parse::<u8>().unwrap())
            .collect();

        // Rest of input is boards, separated by an empty line
        let board_input = input[first_line.len()..].trim();
        let boards: Vec<Board> = board_input
            .split("\r\n\r\n")
            .map(|chunk| {
                chunk
                    .split_ascii_whitespace()
                    .flat_map(|line| line.split(' '))
                    .map(|num| (num.parse::<u8>().unwrap(), false))
                    .collect()
            })
            .collect();

        (moves, boards)
    }

    fn part1(moves: &[u8], mut boards: Vec<Board>) -> usize {
        for move_value in moves {
            let mut found_tile = false;
            for board in &mut boards {
                // Mark tiles
                for tile in board.iter_mut() {
                    if tile.0 == *move_value {
                        tile.1 = true;
                        found_tile = true;
                    }
                }

                // If any tile marked, check for win
                if found_tile {
                    if let Some(answer) = check_board(*move_value, board) {
                        return answer;
                    }
                }
            }
        }

        unreachable!("Failed to find a solution");
    }

    fn part2(moves: &[u8], mut boards: Vec<Board>) -> usize {
        // Apply all moves
        for move_value in moves {
            // Apply move to each board
            let mut board_idx = 0;
            while board_idx < boards.len() {
                let board = &mut boards[board_idx];
                let mut found_tile = false;

                // Apply move to board
                for tile in board.iter_mut() {
                    if tile.0 == *move_value {
                        tile.1 = true;
                        found_tile = true;
                    }
                }

                // If tile found, check for win
                if found_tile {
                    // If board wins, remove it from list
                    if let Some(_) = check_board(*move_value, board) {
                        // Return answer if it's the last board
                        if boards.len() == 1 {
                            return boards[0]
                                .iter()
                                .filter_map(|(value, marked)| (!marked).then(|| *value as usize))
                                .sum::<usize>()
                                * *move_value as usize;
                        }

                        boards.remove(board_idx);
                        continue;
                    }
                }

                // Apply move to next board
                board_idx += 1;
            }
        }

        unreachable!("Failed to find a solution");
    }

    fn check_board(last_value: u8, board: &Board) -> Option<usize> {
        // Overly clever way to generate row and col indices
        lazy_static::lazy_static! {
            static ref ROWCOLS: Vec<Vec<u8>> =
                (0..5).map(|row| (row * 5..row * 5 + 5).collect())
                .chain(
                    (0..5).map(|col| (col..25).step_by(5).collect())
                ).collect();
        }

        // Test each rowcol
        for rowcol in ROWCOLS.iter() {
            // Get tiles
            let tiles = rowcol.iter().map(|idx| board[*idx as usize]);

            // Check if all tiles in rowcol marked
            if tiles.clone().all(|(_, marked)| marked) {
                // If yes, answer is UNMARKED tiles *
                let result = board
                    .iter()
                    .filter_map(|(value, marked)| (!marked).then(|| *value as usize))
                    .sum::<usize>()
                    * last_value as usize;
                return Some(result);
            }
        }

        None
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn examples() {
            let (moves, boards) = parse_input(crate::data::_DAY04_EXAMPLE1);
            assert_eq!(part1(&moves, boards.clone()), 4512);
            assert_eq!(part2(&moves, boards), 1924);
        }

        #[test]
        fn verify() {
            let (moves, boards) = parse_input(crate::data::DAY04);
            assert_eq!(part1(&moves, boards.clone()), 22680);
            assert_eq!(part2(&moves, boards), 16168);
        }
    }
}

pub mod day05 {
    use regex::Regex;
    use std::fmt::Write;

    type Point = fts_vecmath::point2::Point2<i32>;
    type Vector = fts_vecmath::vector2::Vector2<i32>;
    type Segment = (Point, Point);

    pub fn run() -> String {
        let mut result = String::with_capacity(128);

        let segments = parse_input(crate::data::DAY05);

        let answer_part1 = part1(&segments);
        writeln!(&mut result, "Day 05, Problem 1 - [{}]", answer_part1).unwrap();

        let answer_part2 = part2(&segments);
        writeln!(&mut result, "Day 05, Problem 2 - [{}]", answer_part2).unwrap();

        result
    }

    fn parse_input(input: &str) -> Vec<Segment> {
        lazy_static::lazy_static! {
            static ref LINE_REGEX: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        }

        input
            .lines()
            .map(|line| {
                let caps = LINE_REGEX.captures(line).unwrap();
                let x1 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let y1 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let x2 = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
                let y2 = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
                (Point::new(x1, y1), Point::new(x2, y2))
            })
            .collect()
    }

    fn solve(segments: &[Segment], include_diagonals: bool) -> usize {
        // Find max
        let mut width = 0;
        let mut height = 0;
        for segment in segments {
            width = width.max(segment.0.x).max(segment.1.x);
            height = height.max(segment.0.y).max(segment.1.y);
        }

        // Create 1D vec of overlap counts
        let mut overlaps: Vec<u16> = Default::default();
        overlaps.resize((width as usize + 1) * (height as usize + 1), 0);

        // Process all segments
        for segment in segments {
            // Compute segment delta
            // Assumed to be horizontal, vertical, 45-degree diagonal
            let dx = (segment.1.x - segment.0.x).clamp(-1, 1);
            let dy = (segment.1.y - segment.0.y).clamp(-1, 1);
            let delta = Vector::new(dx, dy);

            // Diagonals may or may not be ignored
            if !include_diagonals && dx != 0 && dy != 0 {
                continue;
            }

            // Iterate segment
            let mut pt = segment.0;
            let end = segment.1 + delta;
            while pt != end {
                let idx = pt.y * width + pt.x;
                overlaps[idx as usize] += 1;
                pt += delta;
            }
        }

        // Count points with more than 1 overlap
        overlaps.iter().filter(|count| **count >= 2).count()
    }

    fn part1(segments: &[Segment]) -> usize {
        solve(segments, false)
    }

    fn part2(segments: &[Segment]) -> usize {
        solve(segments, true)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn examples() {
            let segments = parse_input(crate::data::_DAY05_EXAMPLE1);
            assert_eq!(part1(&segments), 5);
            assert_eq!(part2(&segments), 12);
        }

        #[test]
        fn verify() {
            let segments = parse_input(crate::data::DAY05);
            assert_eq!(part1(&segments), 5145);
            assert_eq!(part2(&segments), 16518);
        }
    }
}

pub mod day06 {
    use std::fmt::Write;

    pub fn run() -> String {
        let mut result = String::with_capacity(128);

        let fish = parse_input(crate::data::DAY06);
        let answer_part1 = solve(&fish, 80);
        writeln!(&mut result, "Day 06, Problem 1 - [{}]", answer_part1).unwrap();

        let answer_part2 = solve(&fish, 256);
        writeln!(&mut result, "Day 06, Problem 2 - [{}]", answer_part2).unwrap();

        result
    }

    fn parse_input(input: &str) -> Vec<u8> {
        input.split(",").map(|s| s.parse::<u8>().unwrap()).collect()
    }

    fn solve(fishies: &[u8], num_days: usize) -> usize {
        // Create buckets
        let mut buckets: [usize; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];

        // Initialize buckets
        for fish in fishies {
            buckets[*fish as usize] += 1;
        }

        // Simulate
        for _ in 0..num_days {
            buckets.rotate_left(1);
            buckets[6] += buckets[8];
        }

        // Count total number of fishies
        buckets.iter().sum()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn examples() {
            let input = "3,4,3,1,2";
            let fish = parse_input(input);
            assert_eq!(solve(&fish, 80), 5934);
            assert_eq!(solve(&fish, 256), 26_984_457_539);
        }

        #[test]
        fn verify() {
            let fish = parse_input(crate::data::DAY06);
            assert_eq!(solve(&fish, 80), 390923);
            assert_eq!(solve(&fish, 256), 1_749_945_484_935);
        }
    }
}

pub mod day07 {
    use std::fmt::Write;

    pub fn run() -> String {
        let mut result = String::with_capacity(128);

        let data = parse_input(crate::data::DAY07);

        let answer_part1 = part1(data.clone());
        writeln!(&mut result, "Day 07, Problem 1 - [{}]", answer_part1).unwrap();

        let answer_part2 = part2(data);
        writeln!(&mut result, "Day 07, Problem 2 - [{}]", answer_part2).unwrap();

        result
    }

    fn parse_input(input: &str) -> Vec<i32> {
        input
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    }

    fn part1(crabs: Vec<i32>) -> usize {
        let mut crabs: Vec<i32> = crabs.iter().cloned().collect();

        let idx = crabs.len() / 2;
        crabs.select_nth_unstable(idx);
        let target = crabs[idx];

        crabs.iter().map(|crab| (*crab - target).abs()).sum::<i32>() as usize
    }

    fn part2(crabs: Vec<i32>) -> usize {
        let calc = |target: i32| -> usize {
            crabs
                .iter()
                .map(|crab| {
                    let diff = (*crab - target).abs();
                    ((diff * (diff + 1)) / 2) as usize
                })
                .sum()
        };

        let avg: f64 = crabs.iter().map(|crab| *crab as f64).sum::<f64>() / crabs.len() as f64;

        let lo = calc(avg.floor() as i32);
        let hi = calc(avg.ceil() as i32);
        lo.min(hi) as usize
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn examples() {
            let input = "16,1,2,0,4,2,7,1,2,14";
            let data = parse_input(input);
            assert_eq!(part1(data.clone()), 37);
            assert_eq!(part2(data), 168);
        }

        #[test]
        fn verify() {
            let data = parse_input(crate::data::DAY07);
            assert_eq!(part1(data.clone()), 329389);
            assert_eq!(part2(data), 86397080);
        }
    }
}

pub mod day08 {
    use std::fmt::Write;

    type Digit<'a> = &'a str;
    type Signal<'a> = Vec<Digit<'a>>;
    type Output<'a> = Vec<Digit<'a>>;
    type Entry<'a> = (Signal<'a>, Output<'a>);

    pub fn run() -> String {
        let mut result = String::with_capacity(128);

        let entries = parse_input(crate::data::DAY08);

        let answer_part1 = part1(&entries);
        writeln!(&mut result, "Day 08, Problem 1 - [{}]", answer_part1).unwrap();

        let answer_part2 = part2(&entries);
        writeln!(&mut result, "Day 08, Problem 2 - [{}]", answer_part2).unwrap();

        result
    }

    fn parse_input(input: &str) -> Vec<Entry> {
        input
            .lines()
            .map(|line| {
                let halves: Vec<_> = line.split(" | ").collect();
                (
                    halves[0].split(" ").collect(),
                    halves[1].split(" ").collect(),
                )
            })
            .collect()
    }

    fn part1(entries: &[Entry]) -> usize {
        entries
            .iter()
            .flat_map(|(_, output)| output.iter())
            .filter(|digit| {
                let len = digit.len();
                len == 2 || len == 3 || len == 4 || len == 7
            })
            .count()
    }

    //   0:      1:      2:      3:      4:
    //  (6)     (2)     (5)     (5)     (4)
    //  aaaa    ....    aaaa    aaaa    ....
    // b    c  .    c  .    c  .    c  b    c
    // b    c  .    c  .    c  .    c  b    c
    //  ....    ....    dddd    dddd    dddd
    // e    f  .    f  e    .  .    f  .    f
    // e    f  .    f  e    .  .    f  .    f
    //  gggg    ....    gggg    gggg    ....

    //   5:      6:      7:      8:      9:
    //  (5)     (6)     (3)     (7)     (6)
    //  aaaa    aaaa    aaaa    aaaa    aaaa
    // b    .  b    .  .    c  b    c  b    c
    // b    .  b    .  .    c  b    c  b    c
    //  dddd    dddd    ....    dddd    dddd
    // .    f  e    f  .    f  e    f  .    f
    // .    f  e    f  .    f  e    f  .    f
    //  gggg    gggg    ....    gggg    gggg

    // 0 = A       0000
    // 1 = B      1    2
    // 2 = C      1    2
    // 3 = D       3333
    // 4 = E      4    5
    // 5 = F      4    5
    // 6 = G       6666

    fn decode_entry(entry: &Entry) -> usize {
        // Helpers
        let char_to_mask: [u8; 7] = [
            0b_000_0001,
            0b_000_0010,
            0b_000_0100,
            0b_000_1000,
            0b_001_0000,
            0b_010_0000,
            0b_100_0000,
        ];
        let signal_to_mask = |signal: Digit| -> u8 {
            signal
                .chars()
                .map(|c| c as usize - 'a' as usize)
                .fold(0, |mask, i| mask | char_to_mask[i])
        };

        // Compute signal masks
        let signals = &entry.0;
        let signal_masks: Vec<u8> = signals
            .iter()
            .map(|signal| signal_to_mask(signal))
            .collect();

        // Calculate how many times each bit occurs
        let bit_counts: Vec<usize> = (0..7)
            .map(|i| 1 << i)
            .map(|bit| signal_masks.iter().filter(|mask| (*mask & bit) > 0).count())
            .collect();

        // Helper functions
        let exclude_bits = |masks: &mut [u8], exclude_mask: u8, ignores: &[usize]| {
            masks
                .iter_mut()
                .enumerate()
                .filter(|(idx, _)| !ignores.contains(idx))
                .for_each(|(_, mask)| *mask &= !exclude_mask);
        };

        let require_bits = |masks: &mut [u8], mask: u8, indices: &[usize]| {
            indices.iter().for_each(|idx| masks[*idx] &= mask);
            exclude_bits(masks, mask, indices);
        };

        // Initialize segment bits
        let all_bits = 0b_111_1111;
        let mut segment_bits: [u8; 7] = [
            all_bits, all_bits, all_bits, all_bits, all_bits, all_bits, all_bits,
        ];

        // find "clock one"
        let signal = signals.iter().find(|signals| signals.len() == 2).unwrap();
        require_bits(&mut segment_bits, signal_to_mask(signal), &[2, 5]);

        // find "clock seven"
        // this solves segment 0
        let signal = *signals.iter().find(|signal| signal.len() == 3).unwrap();
        require_bits(&mut segment_bits, signal_to_mask(signal), &[0, 2, 5]);
        assert_eq!(segment_bits[0].count_ones(), 1);

        // find bit that occurs 6 times.
        // this solves segment 1
        let (bit, _) = bit_counts
            .iter()
            .enumerate()
            .find(|(_, count)| **count == 6)
            .unwrap();
        require_bits(&mut segment_bits, 1 << bit, &[1]);
        assert_eq!(segment_bits[1].count_ones(), 1);

        // find bit that occurs 9 times. this is segment 5
        // this solves segment 5 directly
        // this solves segment 2 indirectly
        let (bit, _) = bit_counts
            .iter()
            .enumerate()
            .find(|(_, count)| **count == 9)
            .unwrap();
        require_bits(&mut segment_bits, 1 << bit, &[5]);
        assert_eq!(segment_bits[5].count_ones(), 1);
        assert_eq!(segment_bits[2].count_ones(), 1);

        // find bit that occurs 4 times. this is segment 4
        let (bit, _) = bit_counts
            .iter()
            .enumerate()
            .find(|(_, count)| **count == 4)
            .unwrap();
        require_bits(&mut segment_bits, 1 << bit, &[4]);
        assert_eq!(segment_bits[4].count_ones(), 1);

        // find signal with len 6 AND contains bit in segment 4
        // this is "clock 0". it's missing bit is segment 3
        // this solves segment 3 directly
        // this solves segment 6 indirectly
        let mut iter = signals
            .iter()
            .enumerate()
            .filter(|(_, signal)| signal.len() == 6)
            .filter(|(idx, _)| (signal_masks[*idx] & segment_bits[4]) != 0)
            .filter(|(idx, _)| (signal_masks[*idx] & segment_bits[2]) != 0);
        assert_eq!(iter.clone().count(), 1);
        let (idx, _) = iter.next().unwrap();
        require_bits(&mut segment_bits, signal_masks[idx] ^ all_bits, &[3]);
        assert_eq!(segment_bits[3].count_ones(), 1);
        assert_eq!(segment_bits[6].count_ones(), 1);

        // All bits solved!
        assert_eq!(segment_bits.iter().all(|bits| bits.count_ones() == 1), true);

        // Helper to compute result
        let mask_to_digit: std::collections::HashMap<u8, usize> = [
            (0b1110111, 0),
            (0b0100100, 1),
            (0b1011101, 2),
            (0b1101101, 3),
            (0b0101110, 4),
            (0b1101011, 5),
            (0b1111011, 6),
            (0b0100101, 7),
            (0b1111111, 8),
            (0b1101111, 9),
        ]
        .iter()
        .cloned()
        .collect();

        // Compute result
        let outputs = &entry.1;

        let result = outputs
            .iter()
            .map(|output| signal_to_mask(output))
            .map(|output_mask| {
                // transcribe bits
                let segment_mask = (0..7)
                    .filter(|idx| (output_mask & segment_bits[*idx]) > 0)
                    .map(|idx| 1 << idx)
                    .fold(0, |acc, next| acc | next);
                mask_to_digit.get(&segment_mask).unwrap()
            })
            .fold(0, |acc, digit| acc * 10 + digit);
        result
    }

    fn part2(entries: &[Entry]) -> usize {
        entries.iter().map(|entry| decode_entry(entry)).sum()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn examples() {
            let entries = parse_input(crate::data::_DAY08_EXAMPLE1);
            assert_eq!(part1(&entries), 26);

            let sample = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
            let sample_entries = parse_input(sample);
            assert_eq!(sample_entries.len(), 1);
            assert_eq!(decode_entry(&sample_entries[0]), 5353);
            assert_eq!(part2(&entries), 61229);
        }

        #[test]
        fn verify() {
            let entries = parse_input(crate::data::DAY08);
            assert_eq!(part1(&entries), 342);
            assert_eq!(part2(&entries), 1068933);
        }
    }
}

pub mod day09 {
    use std::fmt::Write;

    pub fn run() -> String {
        let mut result = String::with_capacity(128);

        let (tiles, width) = parse_input(&crate::data::DAY09);

        let answer_part1 = part1(&tiles, width);
        writeln!(&mut result, "Day 09, Problem 1 - [{}]", answer_part1).unwrap();

        let answer_part2 = part2(&tiles, width);
        writeln!(&mut result, "Day 09, Problem 2 - [{}]", answer_part2).unwrap();

        result
    }

    fn parse_input(input: &str) -> (Vec<u8>, usize) {
        let lines = input.lines();
        let width = lines.clone().next().unwrap().len() + 2;

        let mut pad_row: Vec<u8> = Default::default();
        let pad_value = 10;
        pad_row.resize(width, pad_value);

        let mut tiles: Vec<u8> = Default::default();
        tiles.extend(&pad_row);

        for line in lines {
            tiles.push(pad_value);
            for c in line.chars() {
                tiles.push(c as u8 - '0' as u8);
            }
            tiles.push(pad_value);
        }
        tiles.extend(&pad_row);

        (tiles, width)
    }

    fn find_low_points(tiles: &[u8], width: usize) -> Vec<usize> {
        let mut result: Vec<usize> = Default::default();

        let height = tiles.len() / width;

        let to_idx = |row: usize, col: usize| -> usize { row * width + col };

        for row in 1..height - 1 {
            for col in 1..width - 1 {
                let l = to_idx(row, col - 1);
                let r = to_idx(row, col + 1);
                let u = to_idx(row - 1, col);
                let d = to_idx(row + 1, col);

                let vi = to_idx(row, col);
                let v = tiles[vi];
                if v < tiles[l] && v < tiles[r] && v < tiles[u] && v < tiles[d] {
                    result.push(vi);
                }
            }
        }

        result
    }

    fn part1(tiles: &[u8], width: usize) -> usize {
        find_low_points(&tiles, width)
            .iter()
            .map(|idx| tiles[*idx] as usize + 1)
            .sum()
    }

    fn part2(tiles: &[u8], width: usize) -> usize {
        let mut next_basin_id: u16 = 0;
        let mut basin_sizes: Vec<usize> = Default::default();

        // Precompute offsets
        let offsets: [isize; 4] = [-1, 1, -(width as isize), width as isize];

        // Find all low points
        let low_points = find_low_points(tiles, width);

        // Initialize basin map
        let mut basin_map: Vec<u16> = Default::default();
        let unvisited = u16::MAX;
        basin_map.resize(tiles.len(), unvisited);

        // Process each low point
        for low_point in low_points {
            // Counter for this basin
            let mut basin_size: usize = 0;

            // Initialize basin data
            let mut open_list: Vec<usize> = vec![low_point];
            basin_map[low_point] = next_basin_id;
            basin_size += 1;

            // Flood fill
            while !open_list.is_empty() {
                // Process next tile
                let tile_idx = open_list.pop().unwrap();

                // Check neighbors
                for offset in offsets {
                    // Get neighbor index
                    let neighbor_idx = (tile_idx as isize + offset) as usize;

                    // Skip visited tiles
                    if basin_map[neighbor_idx] != unvisited {
                        continue;
                    }

                    // Skip tall tiles
                    let neighbor_value = tiles[neighbor_idx];
                    if neighbor_value >= 9 {
                        continue;
                    }

                    open_list.push(neighbor_idx);
                    basin_map[neighbor_idx] = next_basin_id;
                    basin_size += 1;
                }
            }

            // Store this basin count
            basin_sizes.push(basin_size);

            // Next basin
            next_basin_id += 1;
        }

        // Get three largest
        basin_sizes.sort();
        let len = basin_sizes.len();
        let a = basin_sizes[len - 3];
        let b = basin_sizes[len - 2];
        let c = basin_sizes[len - 1];
        a * b * c
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn examples() {
            let (tiles, width) = parse_input(&crate::data::_DAY09_EXAMPLE1);
            assert_eq!(part1(&tiles, width), 15);
            assert_eq!(part2(&tiles, width), 1134);
        }

        #[test]
        fn verify() {
            let (tiles, width) = parse_input(&crate::data::DAY09);
            assert_eq!(part1(&tiles, width), 452);
            assert_eq!(part2(&tiles, width), 1263735);
        }
    }
}
pub mod day10 {
    use std::fmt::Write;

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum ParseResult {
        Ok,
        IllegalChar(char),
        IncompleteLine(Vec<char>),
    }

    pub fn run() -> String {
        let mut result = String::with_capacity(128);

        let answer_part1 = part1(crate::data::DAY10);
        writeln!(&mut result, "Day 10, Problem 1 - [{}]", answer_part1).unwrap();

        let answer_part2 = part2(crate::data::DAY10);
        writeln!(&mut result, "Day 10, Problem 2 - [{}]", answer_part2).unwrap();

        result
    }

    fn parse_line(input: &str) -> ParseResult {
        let mut open_list: Vec<char> = Default::default();

        for c in input.trim().chars() {
            match c {
                '(' | '[' | '{' | '<' => {
                    open_list.push(c);
                }
                ')' | ']' | '}' | '>' => {
                    if open_list.is_empty() {
                        unreachable!("Problem does not define an unexpected close char");
                    }

                    let open = open_list.pop().unwrap();
                    match (open, c) {
                        ('(', ')') => (),
                        ('[', ']') => (),
                        ('{', '}') => (),
                        ('<', '>') => (),
                        _ => return ParseResult::IllegalChar(c),
                    };
                }
                _ => unreachable!(&format!("Unexpected char: [{}]", c)),
            }
        }

        if !open_list.is_empty() {
            ParseResult::IncompleteLine(open_list)
        } else {
            ParseResult::Ok
        }
    }

    fn part1(input: &str) -> usize {
        let illegal_chars: Vec<char> = input
            .lines()
            .filter_map(|line| {
                if let ParseResult::IllegalChar(c) = parse_line(line) {
                    Some(c)
                } else {
                    None
                }
            })
            .collect();

        let mut counts: [usize; 4] = [0, 0, 0, 0];
        for illegal_char in illegal_chars {
            let idx = match illegal_char {
                ')' => 0,
                ']' => 1,
                '}' => 2,
                '>' => 3,
                _ => unreachable!(&format!("Unexpected illegal char: [{}]", illegal_char)),
            };
            counts[idx] += 1;
        }

        counts[0] * 3 + counts[1] * 57 + counts[2] * 1197 + counts[3] * 25137
    }

    fn part2(input: &str) -> usize {
        let mut scores: Vec<usize> = input
            .lines()
            .filter_map(|line| {
                if let ParseResult::IncompleteLine(incomplete) = parse_line(line) {
                    Some(incomplete)
                } else {
                    None
                }
            })
            .map(|incomplete| {
                incomplete.iter().rev().fold(0, |acc, c| {
                    acc * 5
                        + match c {
                            '(' => 1,
                            '[' => 2,
                            '{' => 3,
                            '<' => 4,
                            _ => unreachable!(&format!("Unexpected open list char: [{}]", c)),
                        }
                })
            })
            .collect();

        let mid = scores.len() / 2;
        scores.select_nth_unstable(mid);
        scores[mid]
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn examples() {
            assert_eq!(
                parse_line("{([(<{}[<>[]}>{[]{[(<()>"),
                ParseResult::IllegalChar('}')
            );
            assert_eq!(
                parse_line("[[<[([]))<([[{}[[()]]] "),
                ParseResult::IllegalChar(')')
            );
            assert_eq!(
                parse_line("[{[{({}]{}}([{[{{{}}([]"),
                ParseResult::IllegalChar(']')
            );
            assert_eq!(
                parse_line("[<(<(<(<{}))><([]([]() "),
                ParseResult::IllegalChar(')')
            );
            assert_eq!(
                parse_line("<{([([[(<>()){}]>(<<{{ "),
                ParseResult::IllegalChar('>')
            );
            assert_eq!(part1(crate::data::_DAY10_EXAMPLE1), 26397);
            assert_eq!(part2(crate::data::_DAY10_EXAMPLE1), 288957);
        }

        #[test]
        fn verify() {
            assert_eq!(part1(crate::data::DAY10), 318081);
            assert_eq!(part2(crate::data::DAY10), 4361305341);
        }
    }
}

pub mod day11 {
    use std::fmt::Write;

    type Point = fts_vecmath::point2::Point2<isize>;
    type Vector = fts_vecmath::vector2::Vector2<isize>;

    pub fn run() -> String {
        let mut result = String::with_capacity(128);

        let (tiles, width) = parse_input(crate::data::DAY11);
        let (answer_part1, answer_part2) = solve(&tiles, width, 100);

        writeln!(&mut result, "Day 11, Problem 1 - [{}]", answer_part1).unwrap();
        writeln!(&mut result, "Day 11, Problem 2 - [{}]", answer_part2).unwrap();

        result
    }

    fn parse_input(input: &str) -> (Vec<u8>, usize) {
        let width = input.lines().next().unwrap().len();
        let tiles = input
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8))
            .collect();
        (tiles, width)
    }

    fn solve(tiles: &[u8], width: usize, num_steps: usize) -> (usize, usize) {
        let height = tiles.len() / width;

        let mut board: Vec<u8> = tiles.iter().cloned().collect();
        let mut flashed: Vec<bool> = Default::default();
        flashed.resize(tiles.len(), false);
        let mut to_flash: Vec<usize> = Default::default();

        let offsets: Vec<Vector> = vec![
            Vector::new(-1, -1),
            Vector::new(0, -1),
            Vector::new(1, -1),
            Vector::new(-1, 0),
            Vector::new(1, 0),
            Vector::new(-1, 1),
            Vector::new(0, 1),
            Vector::new(1, 1),
        ];

        let mut num_flashes: usize = 0;
        /*
        let print_board = |board: &[u8], step: usize| {
            println!("Board after step {}", step);
            for row in 0..height {
                let mut row_str = String::with_capacity(width);
                for col in 0..width {
                    let idx = row*width + col;
                    row_str.push(char::from_digit(board[idx] as u32, 10).unwrap());
                }
                println!("    {}", row_str);
            }

            println!("");
        };
        */

        let mut answer_part1 = 0;
        let mut step = 0;

        loop {
            step += 1;

            to_flash.clear();

            // Clear flashed
            flashed.fill(false);

            // Increment everything by one
            for (idx, tile) in board.iter_mut().enumerate() {
                *tile += 1;

                // Mark tiles to flash
                if *tile == 10 {
                    to_flash.push(idx);
                    flashed[idx] = true;
                    num_flashes += 1;
                }
            }

            while !to_flash.is_empty() {
                let idx = to_flash.pop().unwrap();

                let row = idx / width;
                let col = idx % width;
                let center = Point::new(col as isize, row as isize);
                let w = width as isize;
                let h = height as isize;

                let neighbors = offsets
                    .iter()
                    .map(|offset| center + *offset)
                    .filter(|pt| pt.x >= 0 && pt.x < w && pt.y >= 0 && pt.y < h)
                    .map(|pt| (pt.y * w + pt.x) as usize);

                for neighbor_idx in neighbors {
                    if flashed[neighbor_idx] {
                        continue;
                    }

                    let neighbor_tile = &mut board[neighbor_idx];
                    *neighbor_tile += 1;

                    if *neighbor_tile == 10 {
                        to_flash.push(neighbor_idx);
                        flashed[neighbor_idx] = true;
                        num_flashes += 1;
                    }
                }
            }

            // Reset 10s
            for tile in &mut board {
                assert_eq!(*tile <= 10, true);
                if *tile == 10 {
                    *tile = 0;
                }
            }

            if step == num_steps {
                answer_part1 = num_flashes;
            }

            if flashed.iter().all(|flash| *flash) {
                let answer_part2 = step;
                return (answer_part1, answer_part2);
            }
        }
        // unreachable due to infinite-loop
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn examples() {
            let (tiles, width) = parse_input(crate::data::_DAY11_EXAMPLE1);

            let (answer_part1, answer_part2) = solve(&tiles, width, 100);
            assert_eq!(answer_part1, 1656);
            assert_eq!(answer_part2, 195);
        }

        #[test]
        fn verify() {
            let (tiles, width) = parse_input(crate::data::DAY11);
            let (answer_part1, answer_part2) = solve(&tiles, width, 100);
            assert_eq!(answer_part1, 1603);
            assert_eq!(answer_part2, 222);
        }
    }
}

pub mod day12 {
    use itertools::Itertools;
    use std::fmt::Write;

    type Edge = (usize, usize);

    pub fn run() -> String {
        let mut result = String::with_capacity(128);

        // parse input
        let (verts, edges) = parse_input(crate::data::DAY12);

        let answer_part1 = solve(&verts, &edges, true);
        writeln!(&mut result, "Day 12, Problem 1 - [{}]", answer_part1).unwrap();

        let answer_part2 = solve(&verts, &edges, false);
        writeln!(&mut result, "Day 12, Problem 2 - [{}]", answer_part2).unwrap();

        result
    }

    fn parse_input(input: &str) -> (Vec<&str>, Vec<Edge>) {
        let mut verts: Vec<&str> = Default::default();
        let mut edges: Vec<(usize, usize)> = Default::default();

        let mut vert_map: std::collections::HashMap<&str, usize> = Default::default();

        for line in input.lines() {
            let (a, b) = line.split("-").collect_tuple().unwrap();

            let idx_a = *vert_map.entry(a).or_insert_with(|| {
                verts.push(a);
                verts.len() - 1
            });
            let idx_b = *vert_map.entry(b).or_insert_with(|| {
                verts.push(b);
                verts.len() - 1
            });
            edges.push((idx_a, idx_b));
        }

        (verts, edges)
    }

    fn solve(verts: &[&str], edges: &[Edge], part_one: bool) -> usize {
        // Precompute small cave flag
        let is_small_cave: Vec<bool> = verts
            .iter()
            .map(|cave| cave.chars().all(|c| c.is_ascii_lowercase()))
            .collect();

        // Precompute edges per vert
        let mut vert_to_edge: Vec<Vec<&Edge>> = Default::default();
        vert_to_edge.resize(verts.len(), Default::default());
        for edge in edges {
            vert_to_edge[edge.0].push(edge);
            vert_to_edge[edge.1].push(edge);
        }

        let get_neighbor = |edge: &Edge, this: usize| -> usize {
            if edge.0 != this {
                edge.0
            } else {
                edge.1
            }
        };

        // Create open list
        type Path = Vec<usize>;
        type Entry = (Path, bool);

        let mut open_list: Vec<Entry> = Default::default();
        let mut unique_paths = 0;

        // Initialize Open list
        let start_idx = verts
            .iter()
            .enumerate()
            .find(|(_, name)| **name == "start")
            .unwrap()
            .0;
        let end_idx = verts
            .iter()
            .enumerate()
            .find(|(_, name)| **name == "end")
            .unwrap()
            .0;
        open_list.push((vec![start_idx], false));

        while !open_list.is_empty() {
            // Get an open path
            let entry: Entry = open_list.pop().unwrap();
            let path: &Path = &entry.0;
            let has_visited_small_cave_twice = entry.1;
            let idx = *path.last().unwrap();

            // get edges
            let edges = &vert_to_edge[idx];
            for edge in edges {
                // get neighbor
                let neighbor = get_neighbor(edge, idx);

                // never go back to start
                if neighbor == start_idx {
                    continue;
                }

                // check for end
                if neighbor == end_idx {
                    unique_paths += 1;
                    continue;
                }

                let mut is_second_visit = false;
                if part_one {
                    // don't visit small caves twice
                    if is_small_cave[neighbor] && path.contains(&neighbor) {
                        continue;
                    }
                } else {
                    // visit at most one small cave twice
                    if is_small_cave[neighbor] && path.contains(&neighbor) {
                        if has_visited_small_cave_twice {
                            continue;
                        } else {
                            is_second_visit = true;
                        }
                    }
                }

                // Visit neighbor
                let mut new_path = path.clone();
                new_path.push(neighbor);
                open_list.push((new_path, has_visited_small_cave_twice || is_second_visit));
            }
        }

        unique_paths
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn examples() {
            let (verts, edges) = parse_input(crate::data::_DAY12_EXAMPLE0);
            assert_eq!(solve(&verts, &edges, true), 10);
            assert_eq!(solve(&verts, &edges, false), 36);

            let (verts, edges) = parse_input(crate::data::_DAY12_EXAMPLE1);
            assert_eq!(solve(&verts, &edges, true), 19);
            assert_eq!(solve(&verts, &edges, false), 103);

            let (verts, edges) = parse_input(crate::data::_DAY12_EXAMPLE2);
            assert_eq!(solve(&verts, &edges, true), 226);
            assert_eq!(solve(&verts, &edges, false), 3509);
        }

        #[test]
        fn verify() {
            let (verts, edges) = parse_input(crate::data::DAY12);
            assert_eq!(solve(&verts, &edges, true), 3738);
            assert_eq!(solve(&verts, &edges, false), 120506);
        }
    }
}

pub mod day13 {
    use itertools::Itertools;
    use std::fmt::Write;

    type Point = fts_vecmath::point2::Point2<i16>;

    enum Fold {
        X(i16),
        Y(i16),
    }

    pub fn run() -> String {
        let mut result = String::with_capacity(128);

        let (points, folds) = parse_input(crate::data::DAY13);

        let answer_part1 = solve(points.clone(), &folds[..1], false);
        writeln!(&mut result, "Day 13, Problem 1 - [{}]", answer_part1).unwrap();

        let answer_part2 = solve(points.clone(), &folds, false);
        writeln!(&mut result, "Day 13, Problem 2 - [{}]", answer_part2).unwrap();

        result
    }

    fn parse_input(input: &str) -> (Vec<Point>, Vec<Fold>) {
        let (points_chunk, folds_chunk) = input.split("\r\n\r\n").collect_tuple().unwrap();

        let points: Vec<Point> = points_chunk
            .lines()
            .map(|line| {
                let (x, y) = line.split(",").collect_tuple().unwrap();
                Point::new(x.parse::<i16>().unwrap(), y.parse::<i16>().unwrap())
            })
            .collect();

        let folds = folds_chunk
            .lines()
            .map(|line| {
                let prefix = "fold along ";
                let (axis, value) = line[prefix.len()..].split("=").collect_tuple().unwrap();
                let value = value.parse::<i16>().unwrap();

                match axis {
                    "x" => Fold::X(value),
                    "y" => Fold::Y(value),
                    _ => unreachable!(&format!("Unexpected axis: [{}]", axis)),
                }
            })
            .collect();

        (points, folds)
    }

    fn solve(mut points: Vec<Point>, folds: &[Fold], print_grid: bool) -> usize {
        for fold in folds {
            points = points
                .iter()
                .map(|point| match fold {
                    Fold::X(fold_x) => {
                        if point.x < *fold_x {
                            *point
                        } else {
                            let new_x = fold_x - (point.x - fold_x);
                            Point::new(new_x, point.y)
                        }
                    }
                    Fold::Y(fold_y) => {
                        if point.y < *fold_y {
                            *point
                        } else {
                            let new_y = fold_y - (point.y - fold_y);
                            Point::new(point.x, new_y)
                        }
                    }
                })
                .unique()
                .collect();
        }

        if print_grid {
            let width = points.iter().map(|pt| pt.x).max().unwrap() + 1;
            let height = points.iter().map(|pt| pt.y).max().unwrap() + 1;

            let points_set: std::collections::HashSet<Point> = points.iter().cloned().collect();

            for row in 0..height {
                let row_str: String = (0..width)
                    .map(|col| {
                        if points_set.contains(&Point::new(col, row)) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect();
                println!("{}", row_str);
            }
        }

        points.len()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn examples() {
            let (points, folds) = parse_input(crate::data::_DAY13_EXAMPLE1);
            assert_eq!(solve(points.clone(), &folds[0..1], false), 17);
            assert_eq!(solve(points.clone(), &folds, false), 16);
        }

        #[test]
        fn verify() {
            let (points, folds) = parse_input(crate::data::DAY13);
            assert_eq!(solve(points.clone(), &folds[0..1], false), 745);
            assert_eq!(solve(points.clone(), &folds, false), 99);
        }
    }
}

pub mod day14 {
    use std::collections::HashMap;
    use std::fmt::Write;

    use itertools::Itertools;

    type Element = u8;
    type Template<'a> = &'a str;
    type Insertion = ((Element, Element), Element);

    pub fn run() -> String {
        let mut result = String::with_capacity(128);

        let (template, insertions) = parse_input(crate::data::DAY14);

        let answer_part1 = solve(template, &insertions, 10);
        writeln!(&mut result, "Day 14, Problem 1 - [{}]", answer_part1).unwrap();

        let answer_part2 = solve(template, &insertions, 40);
        writeln!(&mut result, "Day 14, Problem 2 - [{}]", answer_part2).unwrap();

        result
    }

    fn parse_input(input: &str) -> (Template, Vec<Insertion>) {
        let (template_chunk, insertions_chunk) = input.split("\r\n\r\n").collect_tuple().unwrap();

        let insertions: Vec<Insertion> = insertions_chunk
            .lines()
            .map(|line| {
                let (left, right) = line.split(" -> ").collect_tuple().unwrap();
                (
                    left.chars().map(|c| c as Element).collect_tuple().unwrap(),
                    right.chars().map(|c| c as Element).next().unwrap(),
                )
            })
            .collect();

        (template_chunk, insertions)
    }

    fn solve(template: Template, insertions: &[Insertion], num_steps: usize) -> usize {
        // Convert insertions slice to map
        let rules: HashMap<(Element, Element), Element> = insertions.iter().cloned().collect();

        // Initialize buckets
        type Buckets = HashMap<(Element, Element), usize>;
        let mut buckets: Buckets = template.chars().map(|c| c as Element).tuple_windows().fold(
            Default::default(),
            |mut pairs, pair| {
                *pairs.entry(pair).or_default() += 1;
                pairs
            },
        );

        // Run each step across buckets
        for _ in 0..num_steps {
            buckets = buckets.iter().fold(
                Buckets::with_capacity(buckets.len()),
                |mut buckets: Buckets, (pair, count)| {
                    if let Some(inject) = rules.get(pair) {
                        *buckets.entry((pair.0, *inject)).or_default() += count;
                        *buckets.entry((*inject, pair.1)).or_default() += count;
                    }
                    buckets
                },
            );
        }

        // Count the first element of each pair
        let mut counts: [usize; 26] = Default::default();
        for ((a, _), count) in buckets {
            counts[(a - 'A' as u8) as usize] += count;
        }

        // Increment last element
        let last_idx = template.chars().last().unwrap() as usize - 'A' as usize;
        counts[last_idx] += 1;

        // Compute min-max
        let (min, max) = counts
            .iter()
            .filter(|count| **count > 0)
            .minmax()
            .into_option()
            .unwrap();

        max - min
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn examples() {
            let (template, insertions) = parse_input(crate::data::_DAY14_EXAMPLE1);
            assert_eq!(solve(template, &insertions, 10), 1588);
            assert_eq!(solve(template, &insertions, 10), 1588);
            assert_eq!(solve(template, &insertions, 40), 2188189693529);
        }

        #[test]
        fn verify() {
            let (template, insertions) = parse_input(crate::data::DAY14);
            assert_eq!(solve(template, &insertions, 10), 4517);
            assert_eq!(solve(template, &insertions, 40), 4704817645083);
        }
    }
}

pub mod day15 {
    use priority_queue::PriorityQueue;
    use std::collections::HashSet;
    use std::cmp::Reverse;
    use std::fmt::Write;

    type Point = fts_vecmath::point2::Point2<i16>;
    type Offset = fts_vecmath::vector2::Vector2<i16>;
    type Entry = (Point, Vec<Point>);

    pub fn run() -> String {
        let mut result = String::with_capacity(128);

        let (tiles, width) = parse_input(crate::data::DAY15);

        let answer_part1 = part1(&tiles, width);
        writeln!(&mut result, "Day 15, Problem 1 - [{}]", answer_part1).unwrap();

        let answer_part2 = part2(&tiles, width);
        writeln!(&mut result, "Day 15, Problem 2 - [{}]", answer_part2).unwrap();

        result
    }

    fn parse_input(input: &str) -> (Vec<u8>, usize) {
        let width = input.lines().next().unwrap().len();

        let tiles = input
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8))
            .collect();

        (tiles, width)
    }

    fn part1(tiles: &[u8], width: usize) -> usize {
        // Constants
        let offsets = [
            Offset::new(-1, 0),
            Offset::new(1, 0),
            Offset::new(0, -1),
            Offset::new(0, 1),
        ];

        // Utilities
        let point_to_idx = |pt: Point| pt.y as usize * width + pt.x as usize;

        // Initialize data
        let height = tiles.len() / width;
        let w = width as i16;
        let h = height as i16;
        let goal = Point::new(width as i16 - 1, height as i16 - 1);
        let mut open_list = PriorityQueue::<Entry, Reverse<usize>>::new();
        let mut visited: HashSet<Point> = Default::default();

        let start = Point::zero();
        open_list.push((start, vec![start]), Reverse(0));

        loop {
            let ((pt, path), cost) = open_list.pop().unwrap();
            let cost = cost.0;

            // Check for goal
            if pt == goal {
                return cost;
            }
            
            // Skip if we've already been here
            if !visited.insert(pt) {
                continue;
            }

            // Add neighbors to open list
            let neighbors = offsets.iter().map(|offset| pt + *offset).filter(|p| {
                p.x >= 0
                    && p.x < w
                    && p.y >= 0
                    && p.y < h
                    && !visited.contains(p)
            });

            for neighbor in neighbors {
                let neighbor_idx = point_to_idx(neighbor);
                let neighbor_cost = tiles[neighbor_idx];

                let mut new_path = path.clone();
                new_path.push(neighbor);
                open_list.push((neighbor, new_path), Reverse(cost + neighbor_cost as usize));
            }
        }
    }

    fn part2(tiles: &[u8], width: usize) -> usize {
        let mut big_tiles : Vec<u8> = Default::default();
        big_tiles.resize(tiles.len() * 25, 0);

        let height = tiles.len() / width;
        let big_width = width * 5;
        let big_height = height * 5;

        for row in 0..big_height {
            for col in 0..big_width {
                let r = row % height;
                let c = col % width;

                let inc = (row / height) + (col / width);
                let mut v = tiles[r*width + c] + inc as u8;
                if v >= 10 {
                    v -= 9;
                }

                big_tiles[row*big_width + col] = v;
            }
        }

        part1(&big_tiles, big_width)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn examples() {
            let (tiles, width) = parse_input(crate::data::_DAY15_EXAMPLE1);
            assert_eq!(part1(&tiles, width), 40);
            assert_eq!(part2(&tiles, width), 315);
        }

        #[test]
        fn verify() {
            let (tiles, width) = parse_input(crate::data::DAY15);
            assert_eq!(part1(&tiles, width), 696);
            assert_eq!(part2(&tiles, width), 2952);
        }
    }
}
