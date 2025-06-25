const std = @import("std");
const process = @import("process.zig");

pub fn main() !void {
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    var alloc = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    const allocator = alloc.allocator();
    defer alloc.deinit();

    try stdout.print("Part 1: {}\n", .{try process.process_part1(allocator, process.real_input)});
    _ = alloc.reset(.retain_capacity);
    try stdout.print("Part 2: {}\n", .{try process.process_part2(allocator, process.real_input)});

    try bw.flush();
}

test {
    std.testing.refAllDecls(@This());
}
