use std::collections::HashMap;

pub mod bin;

/// An object in the amphipod's filesystem, either a file or a block of free space.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FsObject {
    File { id: usize, size: usize },
    FreeSpace(usize),
}

pub fn parse_disk_map(input: &str) -> Vec<FsObject> {
    let mut filesystem = Vec::new();
    let mut id = 0;

    for (idx, c) in input.chars().enumerate() {
        let n: usize = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            '\n' => break,
            _ => panic!("c should only ever be in '0'..='9', not c={c:?}"),
        };

        if idx % 2 == 0 {
            filesystem.push(FsObject::File { id, size: n });
            id += 1;
        } else {
            filesystem.push(FsObject::FreeSpace(n))
        }
    }

    filesystem
}

pub fn filesystem_to_id_list(filesystem: &[FsObject]) -> Vec<Option<usize>> {
    let mut v = Vec::with_capacity(
        filesystem
            .iter()
            .map(|obj| match obj {
                FsObject::File { size, .. } => size,
                FsObject::FreeSpace(size) => size,
            })
            .sum(),
    );

    for obj in filesystem {
        match obj {
            FsObject::File { id, size } => v.extend(vec![Some(*id); *size]),
            FsObject::FreeSpace(size) => v.extend(vec![None; *size]),
        }
    }

    v
}

/// Compact the filesystem ID list by moving individual blocks into the first free space.
pub fn compact_fs_id_list(mut id_list: Vec<Option<usize>>) -> Vec<Option<usize>> {
    loop {
        let (last_file_idx, _) = id_list
            .iter()
            .enumerate()
            .filter(|&(_idx, obj)| obj.is_some())
            .last()
            .unwrap();

        let (first_free_idx, _) = id_list
            .iter()
            .enumerate()
            .find(|&(_idx, obj)| obj.is_none())
            .unwrap();

        if first_free_idx > last_file_idx {
            break;
        }

        id_list[first_free_idx] = id_list[last_file_idx];
        id_list[last_file_idx] = None;
    }

    id_list
}

/// Defrag the filesystem ID list by trying to move whole files into the first available free space
/// block that will fit them.
pub fn defrag_fs(filesystem: Vec<FsObject>) -> Vec<Option<usize>> {
    let max_id = filesystem
        .iter()
        .filter_map(|obj| match obj {
            FsObject::File { id, .. } => Some(*id),
            FsObject::FreeSpace(_) => None,
        })
        .last()
        .unwrap();

    // `id_start_idx_map[x]` will give the index in `id_list` of the start of the file with ID `x`.
    // We could create a similar map for free spaces but the sizes of free space chunks can change
    // when we move files, so that map would only be useful the first time we move a file.
    let id_start_idx_map = {
        let mut start_idx_map = Vec::with_capacity(max_id + 1);

        // Initialize the vec with zeroes to allow for indexing directly into it
        start_idx_map.extend(vec![0; max_id + 1]);

        let mut idx = 0;

        for obj in &filesystem {
            match &obj {
                FsObject::File { id, size } => {
                    start_idx_map[*id] = idx;
                    idx += size;
                }
                FsObject::FreeSpace(size) => idx += size,
            }
        }

        start_idx_map
    };

    // Map from ID to the size of that file
    let size_map: HashMap<usize, usize> = filesystem
        .iter()
        .filter_map(|obj| match obj {
            FsObject::File { id, size } => Some((*id, *size)),
            FsObject::FreeSpace(_) => None,
        })
        .collect();

    let mut id_list = filesystem_to_id_list(&filesystem);

    let mut current_id = max_id;
    loop {
        let start_idx = id_start_idx_map[current_id];
        let size = *size_map.get(&current_id).unwrap();

        // Find the index of the first free space that will fit this file. If no large enough free
        // space exists, continue to the next file. Also do not consider free spaces after the
        // current file's position.
        let Some(empty_idx) = id_list
            .windows(size)
            .enumerate()
            .take(start_idx)
            .find_map(|(idx, blocks)| blocks.iter().all(Option::is_none).then_some(idx))
        else {
            if let Some(id) = current_id.checked_sub(1) {
                current_id = id;
                continue;
            } else {
                break;
            }
        };

        for offset in 0..size {
            debug_assert!(id_list[empty_idx + offset].is_none());
            id_list[empty_idx + offset] = Some(current_id);
        }

        for offset in 0..size {
            debug_assert!(id_list[start_idx + offset].is_some());
            id_list[start_idx + offset] = None;
        }

        if let Some(id) = current_id.checked_sub(1) {
            current_id = id;
        } else {
            break;
        }
    }

    id_list
}

/// Compute the checksum of the filesystem id_list.
pub fn compute_checksum(id_list: &[Option<usize>]) -> usize {
    id_list
        .iter()
        .enumerate()
        .map(|(idx, id)| idx * id.unwrap_or(0))
        .sum()
}

#[cfg(test)]
fn pretty_print_filesystem(fs: &[FsObject]) -> String {
    use std::fmt::Write;

    let mut s = String::new();

    for &obj in fs {
        match obj {
            FsObject::File { id, size } => write!(
                s,
                "{}",
                (if id < 10 {
                    format!("{id}")
                } else {
                    format!("({id})")
                })
                .repeat(size)
            )
            .unwrap(),
            FsObject::FreeSpace(size) => write!(s, "{:.<1$}", "", size).unwrap(),
        }
    }

    s
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"2333133121414131402"#;

#[cfg(test)]
mod tests {
    use super::*;
    // use pretty_assertions::assert_eq;

    #[test]
    fn test_pretty_print_filesystem() {
        assert_eq!(
            pretty_print_filesystem(&parse_disk_map("12345")),
            String::from("0..111....22222")
        );

        assert_eq!(
            pretty_print_filesystem(&parse_disk_map(TEST_INPUT)),
            String::from("00...111...2...333.44.5555.6666.777.888899")
        );
    }

    #[test]
    fn test_compact_fs_id_list() {
        // 00...111...2...333.44.5555.6666.777.888899
        // 0099811188827773336446555566..............
        assert_eq!(
            compact_fs_id_list(filesystem_to_id_list(&parse_disk_map(TEST_INPUT))),
            vec![
                Some(0),
                Some(0),
                Some(9),
                Some(9),
                Some(8),
                Some(1),
                Some(1),
                Some(1),
                Some(8),
                Some(8),
                Some(8),
                Some(2),
                Some(7),
                Some(7),
                Some(7),
                Some(3),
                Some(3),
                Some(3),
                Some(6),
                Some(4),
                Some(4),
                Some(6),
                Some(5),
                Some(5),
                Some(5),
                Some(5),
                Some(6),
                Some(6),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ]
        );
    }

    #[test]
    fn test_defrag_fs() {
        // 00...111...2...333.44.5555.6666.777.888899
        // 00992111777.44.333....5555.6666.....8888..
        assert_eq!(
            defrag_fs(parse_disk_map(TEST_INPUT)),
            vec![
                Some(0),
                Some(0),
                Some(9),
                Some(9),
                Some(2),
                Some(1),
                Some(1),
                Some(1),
                Some(7),
                Some(7),
                Some(7),
                None,
                Some(4),
                Some(4),
                None,
                Some(3),
                Some(3),
                Some(3),
                None,
                None,
                None,
                None,
                Some(5),
                Some(5),
                Some(5),
                Some(5),
                None,
                Some(6),
                Some(6),
                Some(6),
                Some(6),
                None,
                None,
                None,
                None,
                None,
                Some(8),
                Some(8),
                Some(8),
                Some(8),
                None,
                None,
            ]
        );
    }
}
