const std = @import("std");
const aoc_lib = @import("aoc_lib");
const lib = @import("lib.zig");

pub const real_input: []const u8 = @embedFile("input.txt");

pub const test_input: []const u8 =
    \\1-3 a: abcde
    \\1-3 b: cdefg
    \\2-9 c: ccccccccc
    \\
;

pub fn process_part1(input: []const u8) !usize {
    var valid_passwords: usize = 0;

    var lines = std.mem.splitScalar(u8, input, '\n');
    while (lines.next()) |line| {
        if (line.len == 0) break;

        const policy_pair = try lib.parse_password_policy_pair(line);
        if (policy_pair.is_valid()) {
            valid_passwords += 1;
        }
    }

    return valid_passwords;
}

pub fn process_part2(input: []const u8) !usize {
    var valid_passwords: usize = 0;

    var lines = std.mem.splitScalar(u8, input, '\n');
    while (lines.next()) |line| {
        if (line.len == 0) break;

        const policy_pair = try lib.parse_password_policy_pair(line);
        if (policy_pair.is_valid_part2()) {
            valid_passwords += 1;
        }
    }

    return valid_passwords;
}

test "process_part1 test" {
    try std.testing.expectEqual(2, process_part1(test_input));
    try std.testing.expectEqual(422, process_part1(real_input));
}

test "process_part2 test" {
    try std.testing.expectEqual(1, process_part2(test_input));
    try std.testing.expectEqual(451, process_part2(real_input));
}
