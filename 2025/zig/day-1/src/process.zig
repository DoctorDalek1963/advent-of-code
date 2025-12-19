const std = @import("std");

pub const real_input: []const u8 = @embedFile("input.txt");

pub const test_input: []const u8 =
    \\L68
    \\L30
    \\R48
    \\L5
    \\R60
    \\L55
    \\L1
    \\L99
    \\R14
    \\L82
    \\
;

const Direction = enum { left, right };

const Move = struct {
    direction: Direction,
    distance: u16,
};

pub fn parseMoves(allocator: std.mem.Allocator, input: []const u8) !std.ArrayList(Move) {
    var lines = std.mem.splitScalar(u8, input, '\n');
    var moves = try std.ArrayList(Move).initCapacity(allocator, std.mem.count(u8, input, "\n"));

    while (lines.next()) |line| {
        if (line.len == 0) break;

        const direction: Direction = switch (line[0]) {
            'L' => .left,
            'R' => .right,
            else => return error.BadInput,
        };
        const distance = try std.fmt.parseUnsigned(u16, line[1..], 10);

        try moves.append(allocator, Move{ .direction = direction, .distance = distance });
    }

    return moves;
}

pub fn processPart1(allocator: std.mem.Allocator, input: []const u8) !usize {
    var moves = try parseMoves(allocator, input);
    defer moves.deinit(allocator);

    var num: i32 = 50;
    var password: usize = 0;
    for (moves.items) |move| {
        switch (move.direction) {
            .left => {
                num = @mod(num - @as(i32, move.distance), 100);
            },
            .right => {
                num = @mod(num + @as(i32, move.distance), 100);
            },
        }

        if (num == 0) password += 1;
    }

    return password;
}

pub fn processPart2(allocator: std.mem.Allocator, input: []const u8) !usize {
    var moves = try parseMoves(allocator, input);
    defer moves.deinit(allocator);

    var num: i32 = 50;
    var password: usize = 0;
    for (moves.items) |move| {
        password += @divFloor(move.distance, 100);
        const distance: i32 = @rem(move.distance, 100);

        switch (move.direction) {
            .left => {
                const new_num: i32 = num - distance;
                if (new_num < 0 and num != 0) password += 1;
                num = @mod(new_num, 100);
            },
            .right => {
                const new_num: i32 = num + distance;
                if (new_num > 100) password += 1;
                num = @mod(new_num, 100);
            },
        }

        if (num == 0) password += 1;
    }

    return password;
}

test "processPart1 test" {
    try std.testing.expectEqual(3, processPart1(std.testing.allocator, test_input));
    try std.testing.expectEqual(982, processPart1(std.testing.allocator, real_input));
}

test "processPart2 test" {
    try std.testing.expectEqual(6, processPart2(std.testing.allocator, test_input));
    try std.testing.expectEqual(6106, processPart2(std.testing.allocator, real_input));
}
