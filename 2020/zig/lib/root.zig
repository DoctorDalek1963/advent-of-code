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
    const line_count = std.mem.count(u8, buf, "\n");

    var nums = try std.ArrayList(T).initCapacity(allocator, line_count);

    var lines = std.mem.splitScalar(u8, buf, '\n');
    while (lines.next()) |line| {
        if (line.len == 0) break;
        try nums.append(try std.fmt.parseInt(T, line, 10));
    }

    std.debug.assert(nums.items.len == line_count);

    return nums;
}
