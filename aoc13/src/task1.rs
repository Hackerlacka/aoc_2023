use crate::utils::PatternNotes;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let pattern_notes = PatternNotes::parse("../input/13_1_example_input.txt");
        let sum = pattern_notes.summarize_notes();

        assert_eq!(sum, 405);
    }
}

pub fn run_task() {
    let pattern_notes = PatternNotes::parse("input/13_1_input.txt");
    let sum = pattern_notes.summarize_notes();

    println!("Sum is {}", sum);
}