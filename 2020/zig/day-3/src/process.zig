const std = @import("std");
const aoc_lib = @import("aoc_lib");
const lib = @import("lib.zig");

pub const real_input: []const u8 = @embedFile("input.txt");

pub const test_input: []const u8 =
    \\..##.......
    \\#...#...#..
    \\.#....#..#.
    \\..#.#...#.#
    \\.#...##..#.
    \\..#.##.....
    \\.#.#.#....#
    \\.#........#
    \\#.##...#...
    \\#...##....#
    \\.#..#...#.#
    \\
;

pub fn process_part1(allocator: std.mem.Allocator, input: []const u8) !usize {
    const lines = try aoc_lib.split_lines(allocator, input);
    defer lines.deinit();

    return lib.count_tree_collisions(lines, 3, 1);
}

pub fn process_part2(allocator: std.mem.Allocator, input: []const u8) !usize {
    const lines = try aoc_lib.split_lines(allocator, input);
    defer lines.deinit();

    var product: usize = 1;
    product *= lib.count_tree_collisions(lines, 1, 1);
    product *= lib.count_tree_collisions(lines, 3, 1);
    product *= lib.count_tree_collisions(lines, 5, 1);
    product *= lib.count_tree_collisions(lines, 7, 1);
    product *= lib.count_tree_collisions(lines, 1, 2);

    return product;
}

test "process_part1 test" {
    try std.testing.expectEqual(7, process_part1(std.testing.allocator, test_input));
    try std.testing.expectEqual(274, process_part1(std.testing.allocator, real_input));
}

test "process_part2 test" {
    try std.testing.expectEqual(336, process_part2(std.testing.allocator, test_input));
    try std.testing.expectEqual(6_050_183_040, process_part2(std.testing.allocator, real_input));
}
