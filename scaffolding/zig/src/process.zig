const std = @import("std");
const aoc_lib = @import("aoc_lib");

pub const real_input: []const u8 = @embedFile("input.txt");

pub const test_input: []const u8 =
    \\Blank line is important
    \\
;

pub fn process_part1(allocator: std.mem.Allocator, input: []const u8) !usize {
    _ = allocator;
    _ = input;
    return 0;
}

pub fn process_part2(allocator: std.mem.Allocator, input: []const u8) !usize {
    _ = allocator;
    _ = input;
    return 0;
}

test "process part1 test" {
    try std.testing.expectEqual(process_part1(std.testing.allocator, test_input), 1);
    try std.testing.expectEqual(process_part1(std.testing.allocator, real_input), 1);
}

test "process part2 test" {
    try std.testing.expectEqual(process_part2(std.testing.allocator, test_input), 1);
    try std.testing.expectEqual(process_part2(std.testing.allocator, real_input), 1);
}
