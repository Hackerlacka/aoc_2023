use crate::utils::PatternNotes;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let mut pattern_notes = PatternNotes::parse("../input/13_1_example_input.txt");
        let sum = pattern_notes.fix_smudges_and_summarize();

        assert_eq!(sum, 400);
    }
}

pub fn run_task() {
    let mut pattern_notes = PatternNotes::parse("input/13_1_input.txt");
    let sum = pattern_notes.fix_smudges_and_summarize();

    println!("Sum is {}", sum);
}