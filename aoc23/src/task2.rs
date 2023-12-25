use crate::utils::TrailMap;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let mut trail_map = TrailMap::parse("../input/23_1_example_input.txt");
        let longest_hike = trail_map.find_longest_hike(true);

        assert_eq!(longest_hike, 154);
    }
}

pub fn run_task() {
    let mut trail_map = TrailMap::parse("input/23_1_input.txt");
    let longest_hike = trail_map.find_longest_hike(true);

    println!("Longest hike (ignoring slopes): {}", longest_hike);
}