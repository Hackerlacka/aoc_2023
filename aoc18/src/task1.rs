use crate::utils::DigPlan;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let dig_plan = DigPlan::parse("../input/18_1_example_input.txt", false);
        let lava_capacity = dig_plan.get_lava_capacity();

        assert_eq!(lava_capacity, 62);
    }
}

pub fn run_task() {
    let dig_plan = DigPlan::parse("input/18_1_input.txt", false);
    let lava_capacity = dig_plan.get_lava_capacity();

    println!("Lava capacity is: {}", lava_capacity);
}