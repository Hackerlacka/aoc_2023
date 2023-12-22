use crate::utils::FarmMap;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let farm_map = FarmMap::parse("../input/21_1_example_input.txt");
        let garden_plots_reached = farm_map.calculate_garden_plots_reached(6, false);

        assert_eq!(garden_plots_reached, 16);
    }
}

pub fn run_task() {
    let farm_map = FarmMap::parse("input/21_1_input.txt");
    let steps = 64;
    let garden_plots_reached = farm_map.calculate_garden_plots_reached(steps, false);

    println!("Garden plots reached after {} steps: {}", steps, garden_plots_reached);
}