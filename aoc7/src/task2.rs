use crate::utils::Hand;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let mut hands = Hand::parse_all("../input/7_1_example_input.txt", true);
        hands.sort();

        let mut total_winnings = 0;
        for (i, hand) in hands.iter().enumerate() {
            let rank = (i + 1) as u64;
            total_winnings += rank * hand.bid;
        }

        assert_eq!(total_winnings, 5905);
    }
}

pub fn run_task() {
    let mut hands = Hand::parse_all("input/7_1_input.txt", true);
    hands.sort();

    let mut total_winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        let rank = (i + 1) as u64;
        total_winnings += rank * hand.bid;
    }

    println!("Total winnings (w. joker): {}", total_winnings);
}