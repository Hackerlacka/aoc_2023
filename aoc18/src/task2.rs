use crate::utils::DigPlan;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let dig_plan = DigPlan::parse("../input/18_1_example_input.txt", true);
        let area = dig_plan.get_lava_capacity_optimized();

        assert_eq!(area, 952408144115);
    }
}

pub fn run_task() {
    let dig_plan = DigPlan::parse("input/18_1_input.txt", true);
    let area = dig_plan.get_lava_capacity_optimized();

    println!("Area is: {}", area);
}