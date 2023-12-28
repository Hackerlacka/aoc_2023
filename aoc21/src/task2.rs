use crate::utils::FarmMap;

// Test input and real input does not share the same properties
pub fn run_task() {
    let farm_map = FarmMap::parse("input/21_1_input.txt");
    let steps = 26501365;
    let garden_plots_reached = farm_map.calculate_garden_plots_reached_opt(steps);

    println!("Garden plots reached in {} steps: {}", steps, garden_plots_reached);
}