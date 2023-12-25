use crate::utils::HailstoneMap;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let hailstone_map = HailstoneMap::parse("../input/24_1_example_input.txt");
        let start = (7, 7, 0);
        let end = (27, 27, 0);
        let xy_intersections = hailstone_map.find_xy_intersections_in_area(start, end);

        assert_eq!(xy_intersections, 2);
    }
}

pub fn run_task() {
    let hailstone_map = HailstoneMap::parse("input/24_1_input.txt");
    let start = (200000000000000, 200000000000000, 0);
    let end = (400000000000000, 400000000000000, 0);
    let xy_intersections = hailstone_map.find_xy_intersections_in_area(start, end);

    println!("XY intersections: {}", xy_intersections);
}