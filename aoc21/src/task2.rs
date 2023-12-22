use crate::utils::FarmMap;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input_1() {
        let farm_map = FarmMap::parse("../input/21_1_example_input.txt");

        let steps = vec![6, 10, 50, 100, 500, 1000]; // , 5000
        let expected_results = vec![16, 50, 1594, 6536, 167004, 668697]; // , 16733044

        aoc_helper::print_time_now();
        for (step, expected_result) in steps.iter().zip(expected_results.iter()) {
            let garden_plots_reached = farm_map.calculate_garden_plots_reached(*step, true);

            assert_eq!(garden_plots_reached, *expected_result);

            println!("Steps: {} OK", step);
            aoc_helper::print_time_now();
        }
    }

    #[test]
    fn test_example_input_2() {
        let farm_map = FarmMap::parse("../input/21_1_example_input.txt");
        let steps: u64 = 1000;
        
        farm_map.output_garden_plots_reached_difference(steps);
    }

    // cargo test -p aoc21 --release test_example_input_3 -- --nocapture
    #[test]
    fn test_example_input_3() {
        let farm_map = FarmMap::parse("../input/21_1_example_input.txt");
        let steps: u64 = 5000;
        
        farm_map.calculate_garden_plots_reached_opt(steps);
    }

    #[test]
    fn test_input() {
        let farm_map = FarmMap::parse("../input/21_1_input.txt");
        let steps: u64 = 1000;
        
        farm_map.output_garden_plots_reached_difference(steps);
    }
}

pub fn run_task() {
    let farm_map = FarmMap::parse("input/21_1_input.txt");
    let steps = 26501365;
    let garden_plots_reached = farm_map.calculate_garden_plots_reached_opt(steps);

    println!("Garden plots reached after {} steps: {}", steps, garden_plots_reached);
}