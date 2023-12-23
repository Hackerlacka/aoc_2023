use crate::utils::TrailMap;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let mut trail_map = TrailMap::parse("../input/23_1_example_input.txt");
        let longest_hike = trail_map.find_longest_hike();

        assert_eq!(longest_hike, 94);
    }
}

pub fn run_task() {

}