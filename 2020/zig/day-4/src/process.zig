const std = @import("std");
const aoc_lib = @import("aoc_lib");
const lib = @import("lib.zig");

pub const real_input: []const u8 = @embedFile("input.txt");

pub const test_input: []const u8 =
    \\ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    \\byr:1937 iyr:2017 cid:147 hgt:183cm
    \\
    \\iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
    \\hcl:#cfa07d byr:1929
    \\
    \\hcl:#ae17e1 iyr:2013
    \\eyr:2024
    \\ecl:brn pid:760753108 byr:1931
    \\hgt:179cm
    \\
    \\hcl:#cfa07d eyr:2025 pid:166559648
    \\iyr:2011 ecl:brn hgt:59in
    \\
;

pub fn process_part1(allocator: std.mem.Allocator, input: []const u8) !usize {
    const passports = try lib.parse_passports(allocator, input);
    defer passports.deinit();

    var count: usize = 0;
    for (passports.items) |passport| {
        if (passport.all_present_except_cid()) {
            count += 1;
        }
    }

    return count;
}

pub fn process_part2(allocator: std.mem.Allocator, input: []const u8) !usize {
    const passports = try lib.parse_passports(allocator, input);
    defer passports.deinit();

    var count: usize = 0;
    for (passports.items) |passport| {
        if (passport.is_valid()) {
            count += 1;
        }
    }

    return count;
}

test "process_part1 test" {
    try std.testing.expectEqual(2, process_part1(std.testing.allocator, test_input));
    try std.testing.expectEqual(204, process_part1(std.testing.allocator, real_input));
}

test "process_part2 test" {
    try std.testing.expectEqual(0, process_part2(std.testing.allocator,
        \\eyr:1972 cid:100
        \\hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
        \\
        \\iyr:2019
        \\hcl:#602927 eyr:1967 hgt:170cm
        \\ecl:grn pid:012533040 byr:1946
        \\
        \\hcl:dab227 iyr:2012
        \\ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
        \\
        \\hgt:59cm ecl:zzz
        \\eyr:2038 hcl:74454a iyr:2023
        \\pid:3556412378 byr:2007
        \\
    ));

    try std.testing.expectEqual(4, process_part2(std.testing.allocator,
        \\pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        \\hcl:#623a2f
        \\
        \\eyr:2029 ecl:blu cid:129 byr:1989
        \\iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
        \\
        \\hcl:#888785
        \\hgt:164cm byr:2001 iyr:2015 cid:88
        \\pid:545766238 ecl:hzl
        \\eyr:2022
        \\
        \\iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
        \\
    ));

    try std.testing.expectEqual(179, process_part2(std.testing.allocator, real_input));
}
