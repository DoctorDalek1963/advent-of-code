const std = @import("std");
const aoc_lib = @import("aoc_lib");
const lib = @import("lib.zig");

pub const real_input: []const u8 = @embedFile("input.txt");

pub const test_input: []const u8 =
    \\abc
    \\
    \\a
    \\b
    \\c
    \\
    \\ab
    \\ac
    \\
    \\a
    \\a
    \\a
    \\a
    \\
    \\b
    \\
;

pub fn process_part1(allocator: std.mem.Allocator, input: []const u8) !usize {
    const groups = try lib.parse_many_groups(allocator, input);
    defer {
        for (groups.items) |group| {
            group.deinit();
        }
        groups.deinit();
    }

    var total: usize = 0;

    for (groups.items) |group| {
        total += lib.count_yesses(lib.questions_any_yes(&group.items));
    }

    return total;
}

pub fn process_part2(allocator: std.mem.Allocator, input: []const u8) !usize {
    const groups = try lib.parse_many_groups(allocator, input);
    defer {
        for (groups.items) |group| {
            group.deinit();
        }
        groups.deinit();
    }

    var total: usize = 0;

    for (groups.items) |group| {
        total += lib.count_yesses(lib.questions_every_yes(&group.items));
    }

    return total;
}

test "process_part1 test" {
    try std.testing.expectEqual(11, process_part1(std.testing.allocator, test_input));
    try std.testing.expectEqual(6335, process_part1(std.testing.allocator, real_input));
}

test "process_part2 test" {
    try std.testing.expectEqual(6, process_part2(std.testing.allocator, test_input));
    try std.testing.expectEqual(3392, process_part2(std.testing.allocator, real_input));
}
