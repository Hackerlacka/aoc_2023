use crate::utils::BrickSnapshot;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let mut snapshot = BrickSnapshot::parse("../input/22_1_example_input.txt");
        let fall_count = snapshot.determine_bricks_that_would_fall();

        assert_eq!(fall_count, 7);
    }
}

pub fn run_task() {
    let mut snapshot = BrickSnapshot::parse("input/22_1_input.txt");
    let fall_count = snapshot.determine_bricks_that_would_fall();

    println!("Fall count: {}", fall_count);
}