use crate::utils::InitSequence;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let init_seq = InitSequence::parse("../input/15_1_example_input.txt");
        let focusing_power = init_seq.calculate_focusing_power();

        assert_eq!(focusing_power, 145)
    }
}

pub fn run_task() {
    let init_seq = InitSequence::parse("input/15_1_input.txt");
    let focusing_power = init_seq.calculate_focusing_power();

    println!("Focusing power: {}", focusing_power);
}