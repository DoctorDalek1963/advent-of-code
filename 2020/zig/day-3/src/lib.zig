const std = @import("std");

pub fn count_tree_collisions(map_lines: std.ArrayList([]const u8), right_move: usize, down_move: usize) usize {
    var collisions: usize = 0;
    var x: usize = 0;
    var y: usize = 0;

    const row_count = map_lines.items.len;
    const row_width = map_lines.items[0].len;

    while (y < row_count) {
        if (map_lines.items[y][x] == '#') {
            collisions += 1;
        }

        x = (x + right_move) % row_width;
        y += down_move;
    }

    return collisions;
}
