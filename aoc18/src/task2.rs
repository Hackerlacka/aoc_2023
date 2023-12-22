use crate::utils::DigPlan;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let dig_plan = DigPlan::parse("../input/18_1_example_input.txt");
        dig_plan.test_part_2();
    }
}

pub fn run_task() {

}