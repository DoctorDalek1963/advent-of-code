const std = @import("std");

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
