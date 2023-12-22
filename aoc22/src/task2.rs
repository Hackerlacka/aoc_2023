use crate::utils::BrickSnapshot;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let mut snapshot = BrickSnapshot::parse("../input/22_1_example_input.txt");
        let disintegration_count = snapshot.determine_disintegration_count();

        assert_eq!(disintegration_count, 5);
    }
}

pub fn run_task() {
    let mut snapshot = BrickSnapshot::parse("input/22_1_input.txt");
    let disintegration_count = snapshot.determine_disintegration_count();

    println!("Disintegration count: {}", disintegration_count);
}