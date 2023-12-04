use std::collections::HashMap;

use crate::utils::ScratchCard;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let cards = ScratchCard::parseAll("../input/4_2_example_input.txt");
        
        let wins: Vec<u32> = cards.iter().map(|sc| sc.calculate_wins()).collect();


        let card_copies_hm = calculate_card_copies_hm(&wins);


        let copies = card_copies_hm.values().sum::<u32>();
        
        assert_eq!(copies, 30);
    }
}

fn calculate_card_copies_hm(wins: &Vec<u32>) -> HashMap<usize, u32>{
    let mut res = HashMap::new();

    // There is one of each scratch card
    for i in 0..wins.len() {
        res.insert(i, 1);
    }

    // Loop through each card and win new ones
    for (i, score) in wins.iter().enumerate() {
        let card_cnt = *res.get(&i).unwrap();

        // Add won cards
        for n in 0..(*score as usize) {
            let k = i + n + 1;
            if k >= wins.len() { // At the end, no more cards to win
                break;
            }
            let old_cnt = res.get(&(k)).unwrap();
            let new_cnt = old_cnt + 1 * card_cnt;
            res.insert(k, new_cnt as u32);
        }
    }

    return res;
}

pub fn run_task() {
    let cards = ScratchCard::parseAll("input/4_1_input.txt");
        
    let wins: Vec<u32> = cards.iter().map(|sc| sc.calculate_wins()).collect();


    let card_copies_hm = calculate_card_copies_hm(&wins);


    let copies = card_copies_hm.values().sum::<u32>();

    println!("Card copies: {}", copies);
}