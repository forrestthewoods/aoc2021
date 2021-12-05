const expect = @import("std").testing.expect;
const Regex = @import("regex").Regex;
const std = @import("std");

pub fn main() anyerror!void {
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};
    defer std.debug.assert(!alloc.deinit());

    //try day01(&alloc.allocator);
    //try day02(&alloc.allocator);
    //try day03(&alloc.allocator);
    try day04(&alloc.allocator);
    //try day05(&alloc.allocator);
}

pub fn day01(alloc: *std.mem.Allocator) anyerror!void {
    // Read file
    var cwd = std.fs.cwd();
    const file_string: []u8 = try cwd.readFileAlloc(alloc, "../data/day01.txt", std.math.maxInt(usize));
    defer alloc.free(file_string);

    // Parse file
    var nums = std.ArrayList(u32).init(alloc);
    defer nums.deinit();

    var lines = std.mem.tokenize(file_string, "\r\n");
    while (lines.next()) |line| {
        const num = try std.fmt.parseInt(u32, line, 10);
        try nums.append(num);
    }
    const len = nums.items.len;

    // Part 1
    {
        var part1: u32 = 0;
        var i: u32 = 0;

        while (i < len - 1) : (i += 1) {
            const j = i + 1;
            if (nums.items[j] > nums.items[i]) {
                part1 += 1;
            }
        }
        std.log.info("Day 1, Problem 1 - [{}]", .{part1});
        try expect(part1 == 1696);
    }

    // Part 2
    {
        var part2: u32 = 0;
        var i: u32 = 0;
        while (i < len - 3) : (i += 1) {
            const j = i + 3;
            if (nums.items[j] > nums.items[i]) {
                part2 += 1;
            }
        }

        std.log.info("Day 1, Problem 2 - [{}]", .{part2});
        try expect(part2 == 1737);
    }
}

pub fn day02(alloc: *std.mem.Allocator) anyerror!void {
    // Data types
    const Dir = enum { Forward, Up, Down };

    const Entry = struct { dir: Dir, amount: i32 };

    // Read file
    var cwd = std.fs.cwd();
    const file_string: []u8 = try cwd.readFileAlloc(alloc, "../data/day02.txt", std.math.maxInt(usize));
    defer alloc.free(file_string);

    // Parse file
    var commands = std.ArrayList(Entry).init(alloc);
    defer commands.deinit();

    var lines = std.mem.tokenize(file_string, "\r\n");
    while (lines.next()) |line| {
        var parts = std.mem.tokenize(line, " ");
        const dir_str = parts.next().?;
        const amount_str = parts.next().?;

        const amount = try std.fmt.parseInt(i32, amount_str, 10);
        if (std.mem.eql(u8, dir_str, "forward")) {
            try commands.append(Entry{ .dir = Dir.Forward, .amount = amount });
        } else if (std.mem.eql(u8, dir_str, "up")) {
            try commands.append(Entry{ .dir = Dir.Up, .amount = amount });
        } else if (std.mem.eql(u8, dir_str, "down")) {
            try commands.append(Entry{ .dir = Dir.Down, .amount = amount });
        } else {
            unreachable;
        }
    }

    // Part 1
    {
        var x: i32 = 0;
        var y: i32 = 0;

        for (commands.items) |command| {
            if (command.dir == Dir.Forward) {
                x += command.amount;
            } else if (command.dir == Dir.Up) {
                y -= command.amount;
            } else if (command.dir == Dir.Down) {
                y += command.amount;
            }
        }

        const result = x * y;
        std.log.info("Day 2, Problem 1 - [{}]", .{result});
    }

    // Part 2
    {
        var x: i32 = 0;
        var y: i32 = 0;
        var aim: i32 = 0;

        for (commands.items) |command| {
            if (command.dir == Dir.Forward) {
                x += command.amount;
                y += command.amount * aim;
            } else if (command.dir == Dir.Up) {
                aim -= command.amount;
            } else if (command.dir == Dir.Down) {
                aim += command.amount;
            }
        }

        const result = x * y;
        std.log.info("Day 2, Problem 2 - [{}]", .{result});
    }
}

pub fn day03(alloc: *std.mem.Allocator) anyerror!void {

    // Read file
    var cwd = std.fs.cwd();
    const file_string: []u8 = try cwd.readFileAlloc(alloc, "../data/day03.txt", std.math.maxInt(usize));
    defer alloc.free(file_string);

    // Parse file
    var nums = std.ArrayList(u16).init(alloc);
    defer nums.deinit();

    var lines = std.mem.tokenize(file_string, "\r\n");
    var num_bits: usize = 0;
    while (lines.next()) |line| {
        const num = try std.fmt.parseInt(u16, line, 2);
        num_bits = line.len;
        try nums.append(num);
    }

    // Part 1
    {
        var gamma: usize = 0;
        var epsilon: usize = 0;
        var i: usize = 0;
        while (i < num_bits) : (i += 1) {
            const mask: u16 = @truncate(u16, @as(usize, 1) << @truncate(u6, i));

            // Count bits
            var count: usize = 0;
            for (nums.items) |num| {
                //std.log.info("num {}  mask {}", .{num, mask});
                if ((num & mask) > 0) {
                    count += 1;
                }
            }

            // Accumulate gamma
            if (count > nums.items.len / 2) {
                gamma |= mask;
            } else {
                epsilon |= mask;
            }
        }

        const solution1 = gamma * epsilon;
        std.log.info("Day 3, Problem 1 - [{}]", .{solution1});
    }

    // Part 2
    {
        const oxygen = try d3p2_calc(alloc, nums.items, num_bits, false);
        const co2 = try d3p2_calc(alloc, nums.items, num_bits, true);

        // Oxygen = [3775]
        // co2: [1159]

        const solution2 = oxygen * co2;
        std.log.info("Day 3, Problem 2 - [{}]", .{solution2});
    }
}

pub fn d3p2_calc(alloc: *std.mem.Allocator, numsInput: []u16, num_bits: usize, invert: bool) anyerror!usize {
    // Make a copy of nums
    var nums = std.ArrayList(u16).init(alloc);
    defer nums.deinit();

    for (numsInput) |num| {
        try nums.append(num);
    }

    var mask: u16 = @truncate(u16, @as(usize, 1) << @truncate(u6, num_bits - 1));
    while (nums.items.len > 1) {
        const count = d3p2_count_bits(nums.items, mask);
        var needs_bit = (count >= (nums.items.len - count));
        if (invert) {
            needs_bit = !needs_bit;
        }

        var idx: usize = 0;
        while (idx < nums.items.len) {
            const num = nums.items[idx];
            const has_bit = (num & mask) > 0;
            if (has_bit != needs_bit) {
                _ = nums.swapRemove(idx);
            } else {
                idx += 1;
            }
        }

        mask >>= 1;
    }
    std.debug.assert(nums.items.len == 1);
    return nums.items[0];
}

pub fn d3p2_count_bits(nums: []u16, mask: u16) usize {
    var count: usize = 0;
    for (nums) |num| {
        if ((num & mask) > 0) {
            count += 1;
        }
    }
    return count;
}

pub fn day04(alloc: *std.mem.Allocator) anyerror!void {
    const Tile = struct { number: u8, marked: bool };
    const Tiles = std.ArrayList(Tile);
    const Board = struct { tiles: Tiles, solved: bool };

    // Read file
    var cwd = std.fs.cwd();
    const file_string: []u8 = try cwd.readFileAlloc(alloc, "../data/day04_example1.txt", std.math.maxInt(usize));
    defer alloc.free(file_string);

    // Parse file
    var numbers = std.ArrayList(u8).init(alloc);
    defer numbers.deinit();

    // Split into chunks
    var chunks = std.mem.split(file_string, "\r\n\r\n");

    // First chunk is numbers
    const numbers_chunk = chunks.next().?;
    var numbers_iter = std.mem.tokenize(numbers_chunk, ",");
    while (numbers_iter.next()) |number_str| {
        const num: u8 = try std.fmt.parseInt(u8, number_str, 10);
        try numbers.append(num);
    }

    // Create List of Boards
    var boards = std.ArrayList(Board).init(alloc);
    defer {
        for (boards.items) |board|
            board.tiles.deinit();
        boards.deinit();
    }

    // All remainder chunks are boards
    while (chunks.next()) |board_chunk| {
        var lines = std.mem.split(board_chunk, "\r\n");
        var new_board = Board{ .tiles = Tiles.init(alloc), .solved = false };
        while (lines.next()) |line| {
            var board_numbers = std.mem.tokenize(line, " ");
            while (board_numbers.next()) |board_number_str| {
                const num = try std.fmt.parseInt(u8, board_number_str, 10);
                try new_board.tiles.append(Tile{ .number = num, .marked = false });
            }
        }
        try boards.append(new_board);
    }

    // Helper function to check if a board has a solution or not
    const check_board = (struct {
        fn call(self: @This(), last_number: u8, tiles: []Tile) ?usize {
            const rowcols = [_][5]u16{
                // rows
                [_]u16{ 0, 1, 2, 3, 4 },
                [_]u16{ 5, 6, 7, 8, 9 },
                [_]u16{ 10, 11, 12, 13, 14 },
                [_]u16{ 15, 16, 17, 18, 19 },
                [_]u16{ 20, 21, 22, 23, 24 },

                // cols
                [_]u16{ 0, 5, 10, 15, 20 },
                [_]u16{ 1, 6, 11, 16, 21 },
                [_]u16{ 2, 7, 12, 17, 22 },
                [_]u16{ 3, 8, 13, 18, 23 },
                [_]u16{ 4, 9, 14, 19, 24 },
            };

            // Check all rows and all columns
            for (rowcols) |rowcol| {
                // Check if all tiles in this rowcol are marked
                var all_marked = true;
                for (rowcol) |idx| {
                    if (tiles[idx].marked == false) {
                        all_marked = false;
                        break;
                    }
                }

                // If rowcol is all marked, compute result
                if (all_marked) {
                    var sum: usize = 0;

                    // Sum unmarked tiles
                    for (tiles) |tile| {
                        if (!tile.marked) {
                            sum += tile.number;
                        }
                    }

                    // Answer is sum_of_unmarked * last_number
                    return sum * @as(usize, last_number);
                }
            }

            return null;
        }
    }{}).call;

    var solution1: usize = 0;
    var solution2: usize = 0;
    const num_boards = boards.items.len;
    var num_boards_solved: usize = 0;

    // Apply all moves
    for (numbers.items) |number| numbers_loop: {
        // Mark number on all boards that have not won
        for (boards.items) |board| {
            if (!board.solved) {
                for (board.tiles.items) |*tile| {
                    if (tile.number == number) {
                        tile.marked = true;
                    }
                }
            }
        }

        // Check all unsolved boards for solutions
        for (boards.items) |*board| {
            // Skip solved boards
            if (board.solved) {
                continue;
            }

            // Check if board is solved
            const maybe_answer = check_board(number, board.tiles.items);
            //const maybe_answer: ?usize = null;
            if (maybe_answer) |answer| {
                board.solved = true;
                num_boards_solved += 1;

                if (num_boards_solved == 1) {
                    // First board is answer1
                    solution1 = answer;
                } else if (num_boards_solved == num_boards) {
                    // Last board is answer2
                    solution2 = answer;
                    break :numbers_loop;
                }
            }
        }
    }

    std.log.info("Day 4, Problem 1 - [{}]", .{solution1});
    std.log.info("Day 4, Problem 2 - [{}]", .{solution2});
}

pub fn day05(alloc: *std.mem.Allocator) anyerror!void {
    var re = try Regex.compile(alloc, "\\w+");
    defer re.deinit();

    const matched = try re.match("hej");
    std.debug.assert(matched == true);
}

// Useful stdlib functions
const tokenize = std.mem.tokenize;
const split = std.mem.split;
const indexOf = std.mem.indexOfScalar;
const indexOfAny = std.mem.indexOfAny;
const indexOfStr = std.mem.indexOfPosLinear;
const lastIndexOf = std.mem.lastIndexOfScalar;
const lastIndexOfAny = std.mem.lastIndexOfAny;
const lastIndexOfStr = std.mem.lastIndexOfLinear;
const trim = std.mem.trim;
const sliceMin = std.mem.min;
const sliceMax = std.mem.max;
const strEql = std.mem.eql;

const strToEnum = std.meta.stringToEnum;

const parseInt = std.fmt.parseInt;
const parseFloat = std.fmt.parseFloat;

const min = std.math.min;
const min3 = std.math.min3;
const max = std.math.max;
const max3 = std.math.max3;

const print = std.debug.print;
const assert = std.debug.assert;

const sort = std.sort.sort;
const asc = std.sort.asc;
const desc = std.sort.desc;
