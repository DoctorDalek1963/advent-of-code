pub mod bin;

use nom::{
    bytes::complete::tag, character::complete, multi::separated_list1, sequence::tuple, IResult,
    Parser,
};
use std::collections::HashSet;

pub type Point = (i32, i32, i32);

pub fn parse_points(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(
        tag("\n"),
        tuple((
            complete::i32,
            tag(","),
            complete::i32,
            tag(","),
            complete::i32,
        ))
        .map(|(a, _, b, _, c)| (a, b, c)),
    )(input)
}

#[inline]
fn adjacent_points(p: &Point) -> [Point; 6] {
    let (x, y, z) = *p;
    [
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ]
}

pub fn find_total_surface_area(cubes: Vec<Point>) -> usize {
    let occupied: HashSet<Point> = cubes.iter().copied().collect();
    let outsides: Vec<Point> = cubes.iter().flat_map(adjacent_points).collect();

    outsides.iter().filter(|&p| !occupied.contains(p)).count()
}

pub fn find_exterior_surface_area(cubes: Vec<Point>) -> usize {
    let original_shape: HashSet<Point> = cubes.iter().copied().collect();
    let mx = 1 + cubes.iter().map(|&(x, _, _)| x).max().unwrap();
    let my = 1 + cubes.iter().map(|&(_, y, _)| y).max().unwrap();
    let mz = 1 + cubes.iter().map(|&(_, _, z)| z).max().unwrap();

    // - Start at origin with set of "flooded" points
    // - Expand all flooded points by adding all floodable points to set
    // - A point is floodable iff it is not original or flooded, and it's accessible orthogonally
    // from a flooded point
    // - Continue expanding flooded points set until a loop where no expansions can be made
    // - Create set of points within bounding box which are not original or flooded and find it's
    // total surface area
    // - Subtract this surface area from the total surface area of the original shape
    let mut flooded_points = HashSet::from([(0, 0, 0)]);

    loop {
        let points: Vec<Point> = flooded_points.iter().copied().collect();
        let mut extended = false;

        for p in points {
            let floodable: Vec<Point> = adjacent_points(&p)
                .iter()
                .filter(|&p| {
                    !original_shape.contains(p)
                        && !flooded_points.contains(p)
                        && p.0 >= 0
                        && p.0 <= mx
                        && p.1 >= 0
                        && p.1 <= my
                        && p.2 >= 0
                        && p.2 <= mz
                })
                .copied()
                .collect();

            if !floodable.is_empty() {
                extended = true;
                flooded_points.extend(floodable);
            }
        }

        if !extended {
            break;
        }
    }

    let total_surface_area = find_total_surface_area(cubes);
    let internal: Vec<Point> = (0..=mx)
        .into_iter()
        .flat_map(|x| {
            (0..=my)
                .into_iter()
                .flat_map(move |y| (0..=mz).into_iter().map(move |z| (x, y, z)))
        })
        .filter(|p| !original_shape.contains(p) && !flooded_points.contains(p))
        .collect();

    total_surface_area - find_total_surface_area(internal)
}

#[cfg(test)]
pub const TEST_INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_points_test() {
        assert_eq!(
            parse_points(TEST_INPUT),
            Ok((
                "\n",
                vec![
                    (2, 2, 2),
                    (1, 2, 2),
                    (3, 2, 2),
                    (2, 1, 2),
                    (2, 3, 2),
                    (2, 2, 1),
                    (2, 2, 3),
                    (2, 2, 4),
                    (2, 2, 6),
                    (1, 2, 5),
                    (3, 2, 5),
                    (2, 1, 5),
                    (2, 3, 5),
                ]
            ))
        );
    }

    #[test]
    fn simple_find_total_surface_area_test() {
        assert_eq!(find_total_surface_area(vec![(1, 1, 1), (2, 1, 1)]), 10);
    }

    #[test]
    fn simple_find_exterior_surface_area_test() {
        assert_eq!(find_exterior_surface_area(vec![(1, 1, 1), (2, 1, 1)]), 10);
    }
}
