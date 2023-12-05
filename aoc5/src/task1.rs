use crate::utils::Almanac;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let almanac = Almanac::parse("../input/5_1_example_input.txt");

        let seed_to_locations = almanac.convert_seeds();

        let min_seed_loc = *seed_to_locations.iter().map(|stl| stl.last().unwrap()).min().unwrap();

        assert_eq!(min_seed_loc, 35);
    }
}

pub fn run_task() {
    let almanac = Almanac::parse("input/5_1_input.txt");
    let seed_to_locations = almanac.convert_seeds();
    let min_seed_loc = *seed_to_locations.iter().map(|stl| stl.last().unwrap()).min().unwrap();

    println!("Min seed location: {}", min_seed_loc);
}