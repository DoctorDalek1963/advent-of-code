const std = @import("std");
const lib = @import("aoc_lib");

pub const real_input: []const u8 = @embedFile("input.txt");

pub const test_input: []const u8 =
    \\1721
    \\979
    \\366
    \\299
    \\675
    \\
;

/// Find the two numbers in the list that sum to 2020 and multiply them.
pub fn process_part1(allocator: std.mem.Allocator, input: []const u8) !usize {
    const nums = try lib.read_lines_as_ints(u32, allocator, input);
    defer nums.deinit();

    const num_count = nums.items.len;

    for (0..num_count) |i| {
        for (i..num_count) |j| {
            const n_i = nums.items[i];
            const n_j = nums.items[j];

            if (n_i + n_j == 2020) {
                return n_i * n_j;
            }
        }
    }

    return error.NumbersNotFound;
}

/// Find the three numbers in the list that sum to 2020 and multiply them.
pub fn process_part2(allocator: std.mem.Allocator, input: []const u8) !usize {
    const nums = try lib.read_lines_as_ints(u32, allocator, input);
    defer nums.deinit();

    const num_count = nums.items.len;

    for (0..num_count) |i| {
        for (i..num_count) |j| {
            for (j..num_count) |k| {
                const n_i = nums.items[i];
                const n_j = nums.items[j];
                const n_k = nums.items[k];

                if (n_i + n_j + n_k == 2020) {
                    return n_i * n_j * n_k;
                }
            }
        }
    }

    return error.NumbersNotFound;
}

test "process part1 test" {
    try std.testing.expectEqual(process_part1(std.testing.allocator, test_input), 514_579);
    try std.testing.expectEqual(process_part1(std.testing.allocator, real_input), 928_896);
}

test "process part2 test" {
    try std.testing.expectEqual(process_part2(std.testing.allocator, test_input), 241_861_950);
    try std.testing.expectEqual(process_part2(std.testing.allocator, real_input), 295_668_576);
}
