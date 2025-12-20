const std = @import("std");

pub const real_input: []const u8 = @embedFile("input.txt");

pub const test_input: []const u8 =
    \\11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
    \\
;

const IdRange = struct {
    first: usize,
    last: usize,
};

fn parseRanges(allocator: std.mem.Allocator, input: []const u8) !std.ArrayList(IdRange) {
    var tuples = try std.ArrayList(IdRange).initCapacity(allocator, std.mem.count(u8, input, ",") + 1);
    var ranges = std.mem.splitScalar(
        u8,
        input[0..(input.len - 1)], // Strip the trailing newline
        ',',
    );

    while (ranges.next()) |range| {
        const idx = std.mem.indexOf(u8, range, "-") orelse return error.BadInput;

        const first = try std.fmt.parseInt(usize, range[0..idx], 10);
        const last = try std.fmt.parseInt(usize, range[(idx + 1)..], 10);

        try tuples.append(allocator, IdRange{ .first = first, .last = last });
    }

    return tuples;
}

fn isInvalidPart1(id: usize) !bool {
    var buf: [16]u8 = undefined;
    const string = try std.fmt.bufPrint(&buf, "{}", .{id});

    if (string.len % 2 == 1) return false;

    const mid = string.len / 2;
    return std.mem.eql(u8, string[0..mid], string[mid..]);
}

pub fn processPart1(allocator: std.mem.Allocator, input: []const u8) !usize {
    var tuples = try parseRanges(allocator, input);
    defer tuples.deinit(allocator);

    var sum: usize = 0;
    for (tuples.items) |tup| {
        for (tup.first..(tup.last + 1)) |id| {
            if (try isInvalidPart1(id)) sum += id;
        }
    }

    return sum;
}

fn isInvalidPart2(id: usize) !bool {
    var buf: [16]u8 = undefined;
    const string = try std.fmt.bufPrint(&buf, "{}", .{id});

    outer: for (1..(@divFloor(string.len, 2) + 1)) |chunk_len| {
        if (@rem(string.len, chunk_len) != 0) continue;

        for (0..(@divFloor(string.len, chunk_len) - 1)) |count| {
            const first_chunk = string[(count * chunk_len)..((count + 1) * chunk_len)];
            const second_chunk = string[((count + 1) * chunk_len)..((count + 2) * chunk_len)];
            if (!std.mem.eql(u8, first_chunk, second_chunk)) continue :outer;
        }

        return true;
    }

    return false;
}

pub fn processPart2(allocator: std.mem.Allocator, input: []const u8) !usize {
    var tuples = try parseRanges(allocator, input);
    defer tuples.deinit(allocator);

    var sum: usize = 0;
    for (tuples.items) |tup| {
        for (tup.first..(tup.last + 1)) |id| {
            if (try isInvalidPart2(id)) sum += id;
        }
    }

    return sum;
}

test "processPart1 test" {
    try std.testing.expectEqual(1_227_775_554, processPart1(std.testing.allocator, test_input));
    try std.testing.expectEqual(29_818_212_493, processPart1(std.testing.allocator, real_input));
}

test "processPart2 test" {
    try std.testing.expectEqual(4_174_379_265, processPart2(std.testing.allocator, test_input));
    try std.testing.expectEqual(37_432_260_594, processPart2(std.testing.allocator, real_input));
}
