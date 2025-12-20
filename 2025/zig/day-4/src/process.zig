const std = @import("std");

pub const real_input: []const u8 = @embedFile("input.txt");

pub const test_input: []const u8 =
    \\..@@.@@@@.
    \\@@@.@.@.@@
    \\@@@@@.@.@@
    \\@.@@@@..@.
    \\@@.@@@@.@@
    \\.@@@@@@@.@
    \\.@.@.@.@@@
    \\@.@@@.@@@@
    \\.@@@@@@@@.
    \\@.@.@@@.@.
    \\
;

fn parseMap(allocator: std.mem.Allocator, input: []const u8) !std.ArrayList(std.ArrayList(bool)) {
    var rows = try std.ArrayList(std.ArrayList(bool)).initCapacity(allocator, std.mem.count(u8, input, "\n"));
    var lines = std.mem.splitScalar(u8, input, '\n');

    while (lines.next()) |line| {
        if (line.len == 0) continue;
        var row = try std.ArrayList(bool).initCapacity(allocator, line.len);

        for (line) |c| {
            try row.append(allocator, c == '@');
        }

        try rows.append(allocator, row);
    }

    return rows;
}

const Coord = struct {
    x: usize,
    y: usize,
};

fn getAccessibleCoords(
    allocator: std.mem.Allocator,
    rows: *const std.ArrayList(std.ArrayList(bool)),
) !std.ArrayList(Coord) {
    var coords = try std.ArrayList(Coord).initCapacity(allocator, 16);

    for (rows.items, 0..) |row, y| {
        for (0..row.items.len) |x| {
            if (!rows.items[y].items[x]) continue;

            var adjacent_count: u8 = 0;

            // Start at top centre and go clockwise
            if (y > 0 and rows.items[y - 1].items[x])
                adjacent_count += 1;

            if (y > 0 and x + 1 < row.items.len and rows.items[y - 1].items[x + 1])
                adjacent_count += 1;

            if (x + 1 < row.items.len and rows.items[y].items[x + 1])
                adjacent_count += 1;

            if (y + 1 < rows.items.len and x + 1 < row.items.len and rows.items[y + 1].items[x + 1])
                adjacent_count += 1;

            if (y + 1 < rows.items.len and rows.items[y + 1].items[x])
                adjacent_count += 1;

            if (y + 1 < rows.items.len and x > 0 and rows.items[y + 1].items[x - 1])
                adjacent_count += 1;

            if (x > 0 and rows.items[y].items[x - 1])
                adjacent_count += 1;

            if (y > 0 and x > 0 and rows.items[y - 1].items[x - 1])
                adjacent_count += 1;

            if (adjacent_count < 4) try coords.append(allocator, Coord{ .x = x, .y = y });
        }
    }

    return coords;
}

pub fn processPart1(allocator: std.mem.Allocator, input: []const u8) !usize {
    var alloc = std.heap.ArenaAllocator.init(allocator);
    const arena = alloc.allocator();
    defer alloc.deinit();

    const rows = try parseMap(arena, input);
    const coords = try getAccessibleCoords(arena, &rows);

    return coords.items.len;
}

pub fn processPart2(allocator: std.mem.Allocator, input: []const u8) !usize {
    var alloc = std.heap.ArenaAllocator.init(allocator);
    const arena = alloc.allocator();
    defer alloc.deinit();

    var rows = try parseMap(arena, input);

    var removed: usize = 0;
    while (true) {
        // for (rows.items) |row| {
        //     for (row.items) |c| {
        //         if (c) {
        //             std.debug.print("@", .{});
        //         } else {
        //             std.debug.print(".", .{});
        //         }
        //     }
        //     std.debug.print("\n", .{});
        // }
        // std.debug.print("\n\n\n", .{});

        const coords = try getAccessibleCoords(arena, &rows);
        if (coords.items.len == 0) break;

        removed += coords.items.len;

        for (coords.items) |coord|
            rows.items[coord.y].items[coord.x] = false;
    }

    return removed;
}

test "processPart1 test" {
    try std.testing.expectEqual(13, processPart1(std.testing.allocator, test_input));
    try std.testing.expectEqual(1445, processPart1(std.testing.allocator, real_input));
}

test "processPart2 test" {
    try std.testing.expectEqual(43, processPart2(std.testing.allocator, test_input));
    try std.testing.expectEqual(8317, processPart2(std.testing.allocator, real_input));
}
