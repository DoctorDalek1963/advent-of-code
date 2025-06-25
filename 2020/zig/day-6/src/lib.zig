const std = @import("std");
const aoc_lib = @import("aoc_lib");

/// Parse a single person's response into a u26 bitarray of flags.
fn parse_one_person(line: []const u8) u26 {
    var flags: u26 = 0;
    for (line) |c| {
        std.debug.assert(c >= 'a' and c <= 'z');
        flags |= @as(u26, 1) << @intCast(c - 'a');
    }
    return flags;
}

/// Parse a group into an array of u26's.
pub fn parse_group(allocator: std.mem.Allocator, group: []const u8) !std.ArrayList(u26) {
    return try aoc_lib.read_lines_as_something(u26, parse_one_person, allocator, group);
}

/// Parse the whole input into many groups, separated by blank lines.
pub fn parse_many_groups(allocator: std.mem.Allocator, input: []const u8) !std.ArrayList(std.ArrayList(u26)) {
    const block_count = std.mem.count(u8, input, "\n\n") + 1;

    var groups = try std.ArrayList(std.ArrayList(u26)).initCapacity(allocator, block_count);

    var blocks = std.mem.splitSequence(u8, input, "\n\n");
    while (blocks.next()) |block| {
        if (block.len == 0) break;
        const group = try parse_group(allocator, block);
        try groups.append(group);
    }

    std.debug.assert(groups.items.len == block_count);

    return groups;
}

/// Return the bitflags of questions that were answered "yes" by any member of this group.
pub fn questions_any_yes(group: *const []u26) u26 {
    var answered: u26 = 0;

    for (group.*) |response| {
        answered |= response;
    }

    return answered;
}

/// Return the bitflags of questions that were answered "yes" by every member of this group.
pub fn questions_every_yes(group: *const []u26) u26 {
    var answered: u26 = std.math.maxInt(u26);

    for (group.*) |response| {
        answered &= response;
    }

    return answered;
}

/// Count the number of yesses in this response.
pub fn count_yesses(response: u26) u32 {
    var yesses: u32 = 0;

    inline for (0..26) |i| {
        yesses += @as(u32, @as(u26, 1) & (response >> i));
    }

    return yesses;
}

test "parse_one_person test" {
    try std.testing.expectEqual(0b0111, parse_one_person("abc"));
    try std.testing.expectEqual(0b1001, parse_one_person("ad"));
    try std.testing.expectEqual(0b0101, parse_one_person("ac"));
    try std.testing.expectEqual(0, parse_one_person(""));
}
