const std = @import("std");

/// A password policy and associated password.
pub const PasswordPolicyPair = struct {
    const Self = @This();

    min: u8,
    max: u8,
    char: u8,
    password: []const u8,

    /// Check if the password is valid for part 1.
    pub fn is_valid(self: *const Self) bool {
        const count = std.mem.count(u8, self.password, &[_]u8{self.char});
        return count >= self.min and count <= self.max;
    }

    /// Check if the password is valid for part 2.
    pub fn is_valid_part2(self: *const Self) bool {
        if (self.min > self.password.len or self.max > self.password.len) {
            @panic("Both positions should be in bounds");
        }

        const pos1 = self.password[self.min - 1] == self.char;
        const pos2 = self.password[self.max - 1] == self.char;
        return pos1 != pos2;
    }
};

/// Parse a `PasswordPolicyPair` in the form `min-max char: password`.
pub fn parse_password_policy_pair(line: []const u8) !PasswordPolicyPair {
    const ParseState = enum {
        min,
        max,
        password,
    };

    var parse_state = ParseState.min;
    var num: u8 = 0;
    var idx: usize = 0;

    var password_policy_pair: PasswordPolicyPair = undefined;

    while (idx < line.len) {
        const c = line[idx];

        switch (parse_state) {
            .min => {
                // While we have digits for min, parse them
                if (c >= '0' and c <= '9') {
                    num *= 10;
                    num += c - '0';

                    idx += 1;
                } else {
                    // If we've parsed all the digits of min, set the number and consume the dash
                    password_policy_pair.min = num;
                    num = 0;

                    idx += 1;
                    parse_state = .max;
                }
            },
            // Almost the same as min, just for a different field, and we parse the char at the end
            .max => {
                // While we have digits for min, parse them
                if (c >= '0' and c <= '9') {
                    num *= 10;
                    num += c - '0';

                    idx += 1;
                } else {
                    // If we've parsed all the digits of min, parse the number and consume the space
                    password_policy_pair.max = num;
                    num = 0;

                    idx += 1; // Consume space
                    password_policy_pair.char = line[idx];
                    idx += 3; // Consume char, colon, and space
                    parse_state = .password;
                }
            },
            .password => {
                password_policy_pair.password = line[idx..];
                break;
            },
        }
    }

    return password_policy_pair;
}

test "parse_line test" {
    try std.testing.expectEqualDeep(PasswordPolicyPair{
        .min = 1,
        .max = 3,
        .char = 'a',
        .password = "abcdef",
    }, try parse_password_policy_pair("1-3 a: abcdef"));

    try std.testing.expectEqualDeep(PasswordPolicyPair{
        .min = 123,
        .max = 0,
        .char = 'f',
        .password = "shjkdkghjks",
    }, try parse_password_policy_pair("123-0 f: shjkdkghjks"));
}
