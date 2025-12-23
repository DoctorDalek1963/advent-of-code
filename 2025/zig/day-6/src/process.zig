const std = @import("std");

pub const real_input: []const u8 = @embedFile("input.txt");

pub const test_input: []const u8 =
    // Trailing whitespace is vitally important
    \\123 328  51 64 
    \\ 45 64  387 23 
    \\  6 98  215 314
    \\*   +   *   +  
    \\
;

const Problem = struct {
    numbers: std.ArrayList(usize),
    operator: enum {
        add,
        multiply,
        unknown,
    },
};

fn parseProblemsForPart1(allocator: std.mem.Allocator, input: []const u8) !std.ArrayList(Problem) {
    var problems = try std.ArrayList(Problem).initCapacity(allocator, 0);
    errdefer problems.deinit(allocator);

    const num_count = std.mem.count(u8, input, "\n") - 1;

    var lines = std.mem.splitScalar(u8, input, '\n');

    // First number
    {
        const line = lines.next() orelse return error.BadInput;
        var nums = std.mem.splitScalar(u8, line, ' ');
        while (nums.next()) |num_str| {
            if (num_str.len == 0) continue;

            const num = try std.fmt.parseInt(usize, num_str, 10);

            var numbers = try std.ArrayList(usize).initCapacity(allocator, num_count);
            errdefer numbers.deinit(allocator);
            try numbers.append(allocator, num);

            try problems.append(allocator, Problem{ .numbers = numbers, .operator = .unknown });
        }
    }

    // Other numbers
    for (1..num_count) |_| {
        const line = lines.next() orelse return error.BadInput;
        var nums = std.mem.splitScalar(u8, line, ' ');
        var idx: usize = 0;

        while (nums.next()) |num_str| {
            if (num_str.len == 0) continue;

            const num = try std.fmt.parseInt(usize, num_str, 10);
            try problems.items[idx].numbers.append(allocator, num);

            idx += 1;
        }
    }

    // Operators
    {
        const line = lines.next() orelse return error.BadInput;
        var ops = std.mem.splitScalar(u8, line, ' ');
        var idx: usize = 0;

        while (ops.next()) |op_str| {
            if (op_str.len != 1) continue;

            problems.items[idx].operator = switch (op_str[0]) {
                '+' => .add,
                '*' => .multiply,
                else => return error.BadInput,
            };

            idx += 1;
        }
    }

    return problems;
}

pub fn processPart1(allocator: std.mem.Allocator, input: []const u8) !usize {
    var alloc = std.heap.ArenaAllocator.init(allocator);
    const arena = alloc.allocator();
    defer alloc.deinit();

    const problems = try parseProblemsForPart1(arena, input);

    var sum: usize = 0;
    for (problems.items) |problem| {
        var tmp: usize = undefined;

        switch (problem.operator) {
            .add => {
                tmp = 0;
                for (problem.numbers.items) |n| tmp += n;
            },
            .multiply => {
                tmp = 1;
                for (problem.numbers.items) |n| tmp *= n;
            },
            else => return error.InvalidOperator,
        }

        sum += tmp;
    }

    return sum;
}

fn parseProblemsForPart2(allocator: std.mem.Allocator, input: []const u8) !std.ArrayList(Problem) {
    var problems = try std.ArrayList(Problem).initCapacity(allocator, 0);
    errdefer problems.deinit(allocator);

    var lines = try std.ArrayList([]const u8).initCapacity(allocator, std.mem.count(u8, input, "\n"));
    defer lines.deinit(allocator);

    var line_iter = std.mem.splitScalar(u8, input, '\n');
    while (line_iter.next()) |line| {
        if (line.len == 0) continue;

        try lines.append(allocator, line);
    }

    var numbers = try std.ArrayList(usize).initCapacity(allocator, 5);
    errdefer numbers.deinit(allocator);
    var problem = Problem{ .numbers = numbers, .operator = .unknown };

    var i = std.mem.indexOf(u8, input, "\n") orelse return error.BadInput;
    i -= 1;

    while (true) : ({
        if (i == 0) break;
        i -= 1;
    }) {
        var is_blank = true;
        for (lines.items) |line| {
            if (line[i] != ' ') {
                is_blank = false;
                break;
            }
        }

        if (is_blank) {
            try problems.append(allocator, problem);
            numbers = try std.ArrayList(usize).initCapacity(allocator, 5);
            problem = Problem{ .numbers = numbers, .operator = .unknown };
            continue;
        }

        var num: usize = 0;
        for (lines.items) |line| {
            switch (line[i]) {
                '0'...'9' => num = num * 10 + (line[i] - '0'),
                ' ' => {},
                '+' => problem.operator = .add,
                '*' => problem.operator = .multiply,
                else => return error.BadInput,
            }
        }
        try problem.numbers.append(allocator, num);
    }

    try problems.append(allocator, problem);

    return problems;
}

pub fn processPart2(allocator: std.mem.Allocator, input: []const u8) !usize {
    var alloc = std.heap.ArenaAllocator.init(allocator);
    const arena = alloc.allocator();
    defer alloc.deinit();

    const problems = try parseProblemsForPart2(arena, input);

    var sum: usize = 0;
    for (problems.items) |problem| {
        var tmp: usize = undefined;

        switch (problem.operator) {
            .add => {
                tmp = 0;
                for (problem.numbers.items) |n| tmp += n;
            },
            .multiply => {
                tmp = 1;
                for (problem.numbers.items) |n| tmp *= n;
            },
            else => return error.InvalidOperator,
        }

        sum += tmp;
    }

    return sum;
}

test "processPart1 test" {
    try std.testing.expectEqual(4_277_556, processPart1(std.testing.allocator, test_input));
    try std.testing.expectEqual(3_525_371_263_915, processPart1(std.testing.allocator, real_input));
}

test "processPart2 test" {
    try std.testing.expectEqual(3_263_827, processPart2(std.testing.allocator, test_input));
    try std.testing.expectEqual(6_846_480_843_636, processPart2(std.testing.allocator, real_input));
}
