const expect = @import("std").testing.expect;
const std = @import("std");


pub fn main() anyerror!void {
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};
    defer std.debug.assert(!alloc.deinit());

    try day01(&alloc.allocator);
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