use crate::utils::ScratchCard;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let cards = ScratchCard::parseAll("../input/4_1_example_input.txt");
        
        let score = cards.iter().map(|sc| sc.calculate_score()).sum::<u32>();

        assert_eq!(score, 13);
    }
}

pub fn run_task() {
    let cards = ScratchCard::parseAll("input/4_1_input.txt");
        
    let score = cards.iter().map(|sc| sc.calculate_score()).sum::<u32>();

    println!("Score is {}", score);
}