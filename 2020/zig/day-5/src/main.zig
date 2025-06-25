const std = @import("std");
const process = @import("process.zig");

pub fn main() !void {
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    try stdout.print("Part 1: {}\n", .{process.process_part1(process.real_input)});
    try stdout.print("Part 2: {}\n", .{process.process_part2(process.real_input)});

    try bw.flush();
}

test {
    std.testing.refAllDecls(@This());
}
