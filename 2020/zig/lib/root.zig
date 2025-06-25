const std = @import("std");

/// Split the `buf` into an array of lines.
pub fn split_lines(allocator: std.mem.Allocator, buf: []const u8) !std.ArrayList([]const u8) {
    const line_count = std.mem.count(u8, buf, "\n");

    var line_array = try std.ArrayList([]const u8).initCapacity(allocator, line_count);

    var line_iter = std.mem.splitScalar(u8, buf, '\n');
    while (line_iter.next()) |line| {
        if (line.len == 0) break;
        try line_array.append(line);
    }

    std.debug.assert(line_array.items.len == line_count);

    return line_array;
}

/// Read the lines in `buf` as the integer type `T`.
pub fn read_lines_as_ints(comptime T: type, allocator: std.mem.Allocator, buf: []const u8) !std.ArrayList(T) {
    const parse = struct {
        fn f(line: []const u8) std.fmt.ParseIntError!T {
            return std.fmt.parseInt(T, line, 10);
        }
    }.f;

    return try read_lines_as_something_with_errors(T, parse, allocator, buf);
}

/// Read the lines if `buf` and apply `func` to each, then return an ArrayList of the return type of `func`.
// zig fmt: off
pub fn read_lines_as_something(
    comptime T: type,
    comptime func: fn (line: []const u8) T,
    allocator: std.mem.Allocator,
    buf: []const u8
) !std.ArrayList(T) {
    // zig fmt: on
    const line_count = std.mem.count(u8, buf, "\n");

    var things = try std.ArrayList(T).initCapacity(allocator, line_count);

    var lines = std.mem.splitScalar(u8, buf, '\n');
    while (lines.next()) |line| {
        if (line.len == 0) break;
        try things.append(func(line));
    }

    std.debug.assert(things.items.len == line_count or things.items.len == line_count + 1);

    return things;
}

/// Same as `read_lines_as_something`, but `func` can error, which we propagate.
// zig fmt: off
pub fn read_lines_as_something_with_errors(
    comptime T: type,
    comptime func: fn (line: []const u8) anyerror!T,
    allocator: std.mem.Allocator,
    buf: []const u8
) !std.ArrayList(T) {
// zig fmt: on
    const line_count = std.mem.count(u8, buf, "\n");

    var things = try std.ArrayList(T).initCapacity(allocator, line_count);

    var lines = std.mem.splitScalar(u8, buf, '\n');
    while (lines.next()) |line| {
        if (line.len == 0) break;
        try things.append(try func(line));
    }

    std.debug.assert(things.items.len == line_count or things.items.len == line_count + 1);

    return things;
}

/// Same as `read_lines_as_something`, but splits on double newlines.
// zig fmt: off
pub fn read_blocks_as_something(
    comptime T: type,
    comptime func: fn (line: []const u8) T,
    allocator: std.mem.Allocator,
    buf: []const u8
) !std.ArrayList(T) {
    // zig fmt: on
    const block_count = std.mem.count(u8, buf, "\n\n") + 1;

    var things = try std.ArrayList(T).initCapacity(allocator, block_count);

    var blocks = std.mem.splitSequence(u8, buf, "\n\n");
    while (blocks.next()) |block| {
        if (block.len == 0) break;
        try things.append(func(block));
    }

    std.debug.assert(things.items.len == block_count);

    return things;
}

/// Same as `read_blocks_as_something`, but `func` can error, which we propagate.
// zig fmt: off
pub fn read_blocks_as_something_with_errors(
    comptime T: type,
    comptime func: fn (line: []const u8) anyerror!T,
    allocator: std.mem.Allocator,
    buf: []const u8
) !std.ArrayList(T) {
    // zig fmt: on
    const block_count = std.mem.count(u8, buf, "\n\n") + 1;

    var things = try std.ArrayList(T).initCapacity(allocator, block_count);

    var blocks = std.mem.splitSequence(u8, buf, "\n\n");
    while (blocks.next()) |block| {
        if (block.len == 0) break;
        try things.append(try func(block));
    }

    std.debug.assert(things.items.len == block_count);

    return things;
}
