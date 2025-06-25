const std = @import("std");

pub const Passport = struct {
    byr: ?[]const u8,
    iyr: ?[]const u8,
    eyr: ?[]const u8,
    hgt: ?[]const u8,
    hcl: ?[]const u8,
    ecl: ?[]const u8,
    pid: ?[]const u8,
    cid: ?[]const u8,

    const Self = @This();

    const default: Self = .{
        .byr = null,
        .iyr = null,
        .eyr = null,
        .hgt = null,
        .hcl = null,
        .ecl = null,
        .pid = null,
        .cid = null,
    };

    /// Check that all fields except `cid` are non-null.
    pub fn all_present_except_cid(self: *const Self) bool {
        return self.byr != null and
            self.iyr != null and
            self.eyr != null and
            self.hgt != null and
            self.hcl != null and
            self.ecl != null and
            self.pid != null;
    }

    /// Check that every field (not including `cid`) is valid according to each field's rules.
    pub fn is_valid(self: *const Self) bool {
        // zig fmt: off
        return (self.byr != null and validate.year(self.byr.?, 1920, 2002))
            and (self.iyr != null and validate.year(self.iyr.?, 2010, 2020))
            and (self.eyr != null and validate.year(self.eyr.?, 2020, 2030))
            and (self.hgt != null and validate.height(self.hgt.?))
            and (self.hcl != null and validate.hex_colour(self.hcl.?))
            and (self.ecl != null and validate.eye_colour(self.ecl.?))
            and (self.pid != null and validate.pid(self.pid.?));
        // zig fmt: on
    }
};

const validate = struct {
    /// Check that the string is a number between `min` and `max`.
    fn year(string: []const u8, min: u32, max: u32) bool {
        const num = std.fmt.parseUnsigned(u32, string, 10) catch {
            return false;
        };
        return num >= min and num <= max;
    }

    /// Check that the string is a reasonable height in centimetres or inches.
    fn height(string: []const u8) bool {
        var idx: usize = 0;
        var num: usize = 0;

        while (idx < string.len) {
            const c = string[idx];
            if (c >= '0' and c <= '9') {
                num *= 10;
                num += c - '0';
                idx += 1;
            } else {
                const units = string[idx..];

                if (std.mem.eql(u8, units, "cm")) {
                    return num >= 150 and num <= 193;
                } else if (std.mem.eql(u8, units, "in")) {
                    return num >= 59 and num <= 76;
                } else {
                    return false;
                }
            }
        }

        return false;
    }

    /// Check that the string is a valid lowercase hex colour.
    fn hex_colour(string: []const u8) bool {
        if (string.len != 7 or string[0] != '#') return false;
        for (1..7) |idx| {
            const c = string[idx];
            if (!((c >= '0' and c <= '9') or (c >= 'a' and c <= 'f'))) return false;
        }
        return true;
    }

    /// Check that the string is one of the 7 allowed eye colours.
    fn eye_colour(string: []const u8) bool {
        return std.mem.eql(u8, string, "amb") or
            std.mem.eql(u8, string, "blu") or
            std.mem.eql(u8, string, "brn") or
            std.mem.eql(u8, string, "gry") or
            std.mem.eql(u8, string, "grn") or
            std.mem.eql(u8, string, "hzl") or
            std.mem.eql(u8, string, "oth");
    }

    /// Check that the string is 9 digits.
    fn pid(string: []const u8) bool {
        if (string.len != 9) return false;
        for (string) |c| {
            if (c < '0' or c > '9') return false;
        }
        return true;
    }
};

/// Parse a list of passports, separated by blank lines
pub fn parse_passports(allocator: std.mem.Allocator, input: []const u8) !std.ArrayList(Passport) {
    const password_count = std.mem.count(u8, input, "\n\n") + 1;

    var list = try std.ArrayList(Passport).initCapacity(allocator, password_count);

    var block_iter = std.mem.splitSequence(u8, input, "\n\n");
    while (block_iter.next()) |block| {
        try list.append(parse_one_passport(block));
    }

    return list;
}

/// Parse a single passport.
fn parse_one_passport(block: []const u8) Passport {
    const FieldParseState = enum { name, value };
    var field_parse_state: FieldParseState = .name;

    var idx: usize = 0;
    var passport = Passport.default;

    var field: []const u8 = undefined;
    var value: []const u8 = undefined;

    while (idx < block.len) {
        switch (field_parse_state) {
            .name => {
                field = block[idx .. idx + 3];
                idx += 4; // Consume the 3 chars of field, and the colon
                field_parse_state = .value;
            },
            .value => {
                const start_idx = idx;
                while (idx < block.len and block[idx] != ' ' and block[idx] != '\n') {
                    idx += 1;
                }
                // block[idx] is now out of bounds or pointing at whitespace
                value = block[start_idx..idx];
                idx += 1;
                field_parse_state = .name;

                // Before we continue to the next field, set this one
                if (std.mem.eql(u8, field, "byr")) {
                    passport.byr = value;
                } else if (std.mem.eql(u8, field, "iyr")) {
                    passport.iyr = value;
                } else if (std.mem.eql(u8, field, "eyr")) {
                    passport.eyr = value;
                } else if (std.mem.eql(u8, field, "hgt")) {
                    passport.hgt = value;
                } else if (std.mem.eql(u8, field, "hcl")) {
                    passport.hcl = value;
                } else if (std.mem.eql(u8, field, "ecl")) {
                    passport.ecl = value;
                } else if (std.mem.eql(u8, field, "pid")) {
                    passport.pid = value;
                } else if (std.mem.eql(u8, field, "cid")) {
                    passport.cid = value;
                } else unreachable;
            },
        }
    }

    return passport;
}

test "parse_one_passport test" {
    try std.testing.expectEqualDeep(Passport{
        .ecl = "gry",
        .pid = "860033327",
        .eyr = "2020",
        .hcl = "#fffffd",
        .byr = "1937",
        .iyr = "2017",
        .cid = "147",
        .hgt = "183cm",
    }, parse_one_passport(
        \\ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        \\byr:1937 iyr:2017 cid:147 hgt:183cm
    ));
}
