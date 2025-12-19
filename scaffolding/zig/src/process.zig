const std = @import("std");
const aoc_lib = @import("aoc_lib");

pub const real_input: []const u8 = @embedFile("input.txt");

pub const test_input: []const u8 =
    \\Blank line is important
    \\
;

pub fn processPart1(allocator: std.mem.Allocator, input: []const u8) !usize {
    _ = allocator;
    _ = input;
    return 0;
}

pub fn processPart2(allocator: std.mem.Allocator, input: []const u8) !usize {
    _ = allocator;
    _ = input;
    return 0;
}

test "processPart1 test" {
    try std.testing.expectEqual(1, processPart1(std.testing.allocator, test_input));
    try std.testing.expectEqual(1, processPart1(std.testing.allocator, real_input));
}

test "processPart2 test" {
    try std.testing.expectEqual(1, processPart2(std.testing.allocator, test_input));
    try std.testing.expectEqual(1, processPart2(std.testing.allocator, real_input));
}
