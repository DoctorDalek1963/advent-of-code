const std = @import("std");
const aoc_lib = @import("aoc_lib");

pub const real_input: []const u8 = @embedFile("input.txt");

fn get_seat_id(string: []const u8) u10 {
    std.debug.assert(string.len == 10);
    var n: u10 = 0;

    for (string) |c| {
        n *= 2;
        if (c == 'F' or c == 'L') {
            n += 0;
        } else if (c == 'B' or c == 'R') {
            n += 1;
        } else unreachable;
    }

    return n;
}

pub fn process_part1(input: []const u8) u10 {
    var max: u10 = 0;

    var lines = std.mem.splitScalar(u8, input, '\n');
    while (lines.next()) |line| {
        if (line.len == 0) break;
        const n = get_seat_id(line);
        if (n > max) {
            max = n;
        }
    }

    return max;
}

pub fn process_part2(input: []const u8) usize {
    var seat_nums: [1024]bool = [_]bool{false} ** 1024;

    var lines = std.mem.splitScalar(u8, input, '\n');
    while (lines.next()) |line| {
        if (line.len == 0) break;
        const n = get_seat_id(line);
        seat_nums[n] = true;
    }

    for (1..1023) |idx| {
        if (seat_nums[idx - 1] and !seat_nums[idx] and seat_nums[idx + 1]) {
            return idx;
        }
    }

    unreachable;
}

test "get_seat_id test" {
    try std.testing.expectEqual(567, get_seat_id("BFFFBBFRRR"));
    try std.testing.expectEqual(119, get_seat_id("FFFBBBFRRR"));
    try std.testing.expectEqual(820, get_seat_id("BBFFBBFRLL"));
}

test "process_part1 test" {
    try std.testing.expectEqual(991, process_part1(real_input));
}

test "process_part2 test" {
    try std.testing.expectEqual(534, process_part2(real_input));
}
