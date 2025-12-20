const std = @import("std");

pub const real_input: []const u8 = @embedFile("input.txt");

pub const test_input: []const u8 =
    \\987654321111111
    \\811111111111119
    \\234234234234278
    \\818181911112111
    \\
;

pub fn processPart1(input: []const u8) !usize {
    var banks = std.mem.splitScalar(u8, input, '\n');

    var sum: usize = 0;
    while (banks.next()) |bank| {
        if (bank.len == 0) continue;

        var max: u8 = 0;

        for (0..(bank.len - 1)) |a_idx| {
            const a = (bank[a_idx] - '0');

            for ((a_idx + 1)..bank.len) |b_idx| {
                const b = bank[b_idx] - '0';
                const num = a * 10 + b;
                max = @max(max, num);
            }
        }

        sum += max;
    }

    return sum;
}

pub fn processPart2(input: []const u8) !usize {
    var banks = std.mem.splitScalar(u8, input, '\n');

    var sum: usize = 0;
    while (banks.next()) |bank| {
        if (bank.len == 0) continue;

        // Find the maximum digit value that could be in position 1 and only
        // consider those digits
        const max_d1 = std.mem.max(u8, bank[0..(bank.len - 11)]) - '0';

        var max: usize = 0;

        for (0..(bank.len - 11)) |idx1| {
            const d1: usize = bank[idx1] - '0';
            if (d1 != max_d1) continue;

            // Repeat for all digits
            const max_d2 = std.mem.max(u8, bank[(idx1 + 1)..(bank.len - 10)]) - '0';

            for ((idx1 + 1)..(bank.len - 10)) |idx2| {
                const d2: usize = bank[idx2] - '0';
                if (d2 != max_d2) continue;

                const max_d3 = std.mem.max(u8, bank[(idx2 + 1)..(bank.len - 9)]) - '0';

                for ((idx2 + 1)..(bank.len - 9)) |idx3| {
                    const d3: usize = bank[idx3] - '0';
                    if (d3 != max_d3) continue;

                    const max_d4 = std.mem.max(u8, bank[(idx3 + 1)..(bank.len - 8)]) - '0';

                    for ((idx3 + 1)..(bank.len - 8)) |idx4| {
                        const d4: usize = bank[idx4] - '0';
                        if (d4 != max_d4) continue;

                        const max_d5 = std.mem.max(u8, bank[(idx4 + 1)..(bank.len - 7)]) - '0';

                        for ((idx4 + 1)..(bank.len - 7)) |idx5| {
                            const d5: usize = bank[idx5] - '0';
                            if (d5 != max_d5) continue;

                            const max_d6 = std.mem.max(u8, bank[(idx5 + 1)..(bank.len - 6)]) - '0';

                            for ((idx5 + 1)..(bank.len - 6)) |idx6| {
                                const d6: usize = bank[idx6] - '0';
                                if (d6 != max_d6) continue;

                                const max_d7 = std.mem.max(u8, bank[(idx6 + 1)..(bank.len - 5)]) - '0';

                                for ((idx6 + 1)..(bank.len - 5)) |idx7| {
                                    const d7: usize = bank[idx7] - '0';
                                    if (d7 != max_d7) continue;

                                    const max_d8 = std.mem.max(u8, bank[(idx7 + 1)..(bank.len - 4)]) - '0';

                                    for ((idx7 + 1)..(bank.len - 4)) |idx8| {
                                        const d8: usize = bank[idx8] - '0';
                                        if (d8 != max_d8) continue;

                                        const max_d9 = std.mem.max(u8, bank[(idx8 + 1)..(bank.len - 3)]) - '0';

                                        for ((idx8 + 1)..(bank.len - 3)) |idx9| {
                                            const d9: usize = bank[idx9] - '0';
                                            if (d9 != max_d9) continue;

                                            const max_d10 = std.mem.max(u8, bank[(idx9 + 1)..(bank.len - 2)]) - '0';

                                            for ((idx9 + 1)..(bank.len - 2)) |idx10| {
                                                const d10: usize = bank[idx10] - '0';
                                                if (d10 != max_d10) continue;

                                                const max_d11 = std.mem.max(u8, bank[(idx10 + 1)..(bank.len - 1)]) - '0';

                                                for ((idx10 + 1)..(bank.len - 1)) |idx11| {
                                                    const d11: usize = bank[idx11] - '0';
                                                    if (d11 != max_d11) continue;

                                                    const max_d12 = std.mem.max(u8, bank[(idx11 + 1)..bank.len]) - '0';

                                                    for ((idx11 + 1)..bank.len) |idx12| {
                                                        const d12: usize = bank[idx12] - '0';
                                                        if (d12 != max_d12) continue;

                                                        // zig fmt: off
                                                        const num = d1 * 100_000_000_000 + d2 * 10_000_000_000 + d3 * 1_000_000_000
                                                            + d4 * 100_000_000 + d5 * 10_000_000 + d6 * 1_000_000
                                                            + d7 * 100_000 + d8 * 10_000 + d9 * 1000
                                                            + d10 * 100 + d11 * 10 + d12;
                                                        // zig fmt: on

                                                        max = @max(max, num);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        sum += max;
    }

    return sum;
}

test "processPart1 test" {
    try std.testing.expectEqual(357, processPart1(test_input));
    try std.testing.expectEqual(16_973, processPart1(real_input));
}

test "processPart2 test" {
    try std.testing.expectEqual(3_121_910_778_619, processPart2(test_input));
    try std.testing.expectEqual(168_027_167_146_027, processPart2(real_input));
}
