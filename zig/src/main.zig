const expect = @import("std").testing.expect;
const std = @import("std");


pub fn main() anyerror!void {
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};
    defer std.debug.assert(!alloc.deinit());

    //try day01(&alloc.allocator);
    //try day02(&alloc.allocator);
    try day03(&alloc.allocator);
}

pub fn day01(alloc: *std.mem.Allocator) anyerror!void
{   
    // Read file
    var cwd = std.fs.cwd();
    const file_string : []u8 = try cwd.readFileAlloc(alloc, "../data/day01.txt", std.math.maxInt(usize) );
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
        var part1 : u32 = 0;
        var i: u32 = 0;

        while (i < len-1) : (i += 1) {
            const j = i+1;
            if (nums.items[j] > nums.items[i]) {
                part1 += 1;
            }        
        }
        std.log.info("Day 1, Problem 1 - [{}]", .{part1});
        try expect(part1 == 1696);
    }

    // Part 2
    {
        var part2 : u32 = 0;
        var i : u32 = 0;
        while (i < len - 3) : (i +=1) {
            const j = i + 3;
            if (nums.items[j] > nums.items[i]) {
                part2 += 1;
            }
        }

        std.log.info("Day 1, Problem 2 - [{}]", .{part2});
        try expect(part2 == 1737);
    }
}

pub fn day02(alloc: *std.mem.Allocator) anyerror!void
{   
    // Data types
    const Dir = enum {
        Forward,
        Up,
        Down
    };

    const Entry = struct {
        dir: Dir,
        amount: i32
    };

    // Read file
    var cwd = std.fs.cwd();
    const file_string : []u8 = try cwd.readFileAlloc(alloc, "../data/day02.txt", std.math.maxInt(usize) );
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
            try commands.append( Entry{ .dir = Dir.Forward, .amount = amount});
        } else if (std.mem.eql(u8, dir_str, "up")) {
            try commands.append( Entry{ .dir = Dir.Up, .amount = amount});
        } else if (std.mem.eql(u8, dir_str, "down")) {
            try commands.append( Entry{ .dir = Dir.Down, .amount = amount});
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

        const result = x*y;
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

        const result = x*y;
        std.log.info("Day 2, Problem 2 - [{}]", .{result});
    }
}

pub fn day03(alloc: *std.mem.Allocator) anyerror!void {

    // Read file
    var cwd = std.fs.cwd();
    const file_string : []u8 = try cwd.readFileAlloc(alloc, "../data/day03.txt", std.math.maxInt(usize) );
    defer alloc.free(file_string);

    // Parse file
    var nums = std.ArrayList(u16).init(alloc);
    defer nums.deinit();

    var lines = std.mem.tokenize(file_string, "\r\n");
    var num_bits : usize = 0;
    while (lines.next()) |line| {
        const num = try std.fmt.parseInt(u16, line, 2);
        num_bits = line.len;
        try nums.append(num);
    }


    // Part 1
    {
        var gamma : usize = 0;
        var epsilon : usize = 0;
        var i : usize = 0;
        while (i<num_bits) : (i+=1) {
            const mask : u16 = @truncate(u16, @as(usize, 1) << @truncate(u6, i));
            
            // Count bits
            var count : usize = 0;
            for (nums.items) |num| {
                //std.log.info("num {}  mask {}", .{num, mask});
                if ((num & mask) > 0) {
                    count += 1;
                }
            }

            // Accumulate gamma
            if (count > nums.items.len/2) {
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
        const filter = struct {
            fn count_bits(invert: bool) usize {
                return 0;
            }
        }.count_bits;
    }
}

// pub fn main() void {
//     const j = 1;
//     var b = struct{
//         fn function(x: i32) i32 {
//             return x+j;
//         }
//     }.function;

//     var c = b(1);
// }