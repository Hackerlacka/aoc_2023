use crate::utils::RockMap;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let mut rock_map = RockMap::parse("../input/14_1_example_input.txt");
        
        rock_map.spin_n_cycles(1000000000);

        let north_load = rock_map.calculate_north_load();

        assert_eq!(north_load, 64);
    }
}

pub fn run_task() {
    let mut rock_map = RockMap::parse("input/14_1_input.txt");
        
        rock_map.spin_n_cycles(1000000000);

        let north_load = rock_map.calculate_north_load();

        println!("North load: {}", north_load);
}