use crate::utils::InitSequence;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let init_seq = InitSequence::parse("../input/15_1_example_input.txt");
        let hashes = init_seq.hash_steps();

        assert_eq!(hashes, vec![30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231]);

        let sum = hashes.iter().sum::<u64>();
        assert_eq!(sum, 1320)
    }
}

pub fn run_task() {
    let init_seq = InitSequence::parse("input/15_1_input.txt");
    let hashes = init_seq.hash_steps();

    let sum = hashes.iter().sum::<u64>();
    println!("Sum is: {}", sum);
}