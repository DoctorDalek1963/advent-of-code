const std = @import("std");
const process = @import("process.zig");

pub fn main() !void {
    var stdout_buffer: [1024]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    try stdout.print("Part 1: {}\n", .{try process.processPart1(process.real_input)});
    try stdout.flush();

    try stdout.print("Part 2: {}\n", .{try process.processPart2(process.real_input)});
    try stdout.flush();
}

test {
    std.testing.refAllDecls(@This());
}
