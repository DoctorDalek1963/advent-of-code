const std = @import("std");

pub const real_input: []const u8 = @embedFile("input.txt");

pub const test_input: []const u8 =
    \\3-5
    \\10-14
    \\16-20
    \\12-18
    \\
    \\1
    \\5
    \\8
    \\11
    \\17
    \\32
    \\
;

const Range = struct {
    start: usize,
    end: usize,
};

fn parseDatabase(
    allocator: std.mem.Allocator,
    input: []const u8,
) !struct { std.ArrayList(Range), std.ArrayList(usize) } {
    var ranges = try std.ArrayList(Range).initCapacity(allocator, 4);
    var ids = try std.ArrayList(usize).initCapacity(allocator, 4);

    var chunks = std.mem.splitSequence(u8, input, "\n\n");

    const ranges_string = chunks.next() orelse return error.BadInput;
    var ranges_lines = std.mem.splitScalar(u8, ranges_string, '\n');

    while (ranges_lines.next()) |line| {
        if (line.len == 0) continue;

        const mid = std.mem.indexOf(u8, line, "-") orelse return error.BadInput;
        const start = try std.fmt.parseInt(usize, line[0..mid], 10);
        const end = try std.fmt.parseInt(usize, line[(mid + 1)..line.len], 10);

        try ranges.append(allocator, Range{ .start = start, .end = end });
    }

    const ids_string = chunks.next() orelse return error.BadInput;
    var ids_lines = std.mem.splitScalar(u8, ids_string, '\n');

    while (ids_lines.next()) |line| {
        if (line.len == 0) continue;

        const id = try std.fmt.parseInt(usize, line, 10);
        try ids.append(allocator, id);
    }

    return .{ ranges, ids };
}

fn isIdFresh(id: usize, ranges: []const Range) bool {
    for (ranges) |range| {
        if (id >= range.start and id <= range.end)
            return true;
    }

    return false;
}

pub fn processPart1(allocator: std.mem.Allocator, input: []const u8) !usize {
    var ranges, var ids = try parseDatabase(allocator, input);
    defer {
        ranges.deinit(allocator);
        ids.deinit(allocator);
    }

    var fresh_ids: usize = 0;

    for (ids.items) |id| {
        if (isIdFresh(id, ranges.items))
            fresh_ids += 1;
    }

    return fresh_ids;
}

pub fn processPart2(allocator: std.mem.Allocator, input: []const u8) !usize {
    var ranges, var ids = try parseDatabase(allocator, input);
    ids.deinit(allocator);
    defer ranges.deinit(allocator);

    const lessThan = struct {
        fn lessThan(ctx: void, lhs: Range, rhs: Range) bool {
            _ = ctx;
            return lhs.start < rhs.start;
        }
    }.lessThan;

    std.sort.block(Range, ranges.items, {}, lessThan);

    var merged_ranges = try std.ArrayList(Range).initCapacity(allocator, @divFloor(ranges.items.len, 2));
    defer merged_ranges.deinit(allocator);

    outer: for (ranges.items) |range| {
        for (0..merged_ranges.items.len) |m_idx| {
            const m_range = &merged_ranges.items[m_idx];

            if (range.start >= m_range.start and range.start <= m_range.end) {
                m_range.*.end = @max(range.end, m_range.end);
                continue :outer;
            }

            if (range.end >= m_range.start and range.start <= m_range.end) {
                m_range.*.start = @min(range.start, m_range.start);
                continue :outer;
            }
        }

        try merged_ranges.append(allocator, range);
    }

    var fresh_ids: usize = 0;
    for (merged_ranges.items) |range|
        fresh_ids += (range.end - range.start) + 1;

    return fresh_ids;
}

test "processPart1 test" {
    try std.testing.expectEqual(3, processPart1(std.testing.allocator, test_input));
    try std.testing.expectEqual(885, processPart1(std.testing.allocator, real_input));
}

test "processPart2 test" {
    try std.testing.expectEqual(14, processPart2(std.testing.allocator, test_input));
    try std.testing.expectEqual(348_115_621_205_535, processPart2(std.testing.allocator, real_input));
}
