use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard
}

impl HandType {
    // TODO: Can do enum as u64 instead, and also assign values to enums like in C
    pub fn score(&self) -> u64 {
        match self {
            Self::FiveOfKind => 7,
            Self::FourOfKind => 6,
            Self::FullHouse => 5,
            Self::ThreeOfKind => 4,
            Self::TwoPair => 3,
            Self::OnePair => 2,
            Self::HighCard => 1,
        }
    }

    pub fn cmp_hand_type(h1: &HandType, h2: &HandType) -> Ordering {
        h1.score().cmp(&h2.score())
    }
}

#[derive(Debug)]
pub struct Hand {
    hand: String,
    pub bid: u64,
    hand_type: HandType,
    joker: bool,
    //sort_score: u64, // hand_type + value of all other cards i.e. Y00000 + ABCDE
}

impl PartialOrd for Hand
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {        
        // Compare hand types
        let res = HandType::cmp_hand_type(&self.hand_type, &other.hand_type);
        if res != Ordering::Equal {
            return res;
        }

        // Then compare hand strs if equal
        Hand::cmp_hand_strs(&self.hand, &other.hand, self.joker)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.hand == other.hand
    }
}

impl Eq for Hand {
    
}

impl Hand {
    fn get_card_score(c: char, joker: bool) -> u32 {
        match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => {
                if joker {
                    1
                } else {
                    11
                }
            },
            'T' => 10,
            other => other.to_digit(10).unwrap()
        }
    }

    fn cmp_cards(c1: char, c2: char, joker: bool) -> Ordering {
        Hand::get_card_score(c1, joker).cmp(&Hand::get_card_score(c2, joker))
    }

    fn cmp_hand_strs(hand_1: &str, hand_2: &str, joker: bool) -> Ordering {
        for (c1, c2) in hand_1.chars().zip(hand_2.chars()) {
            let cmp_res = Hand::cmp_cards(c1, c2, joker);

            if cmp_res != Ordering::Equal {
                return cmp_res;
            }
        }

        return Ordering::Equal;
    }

    fn determine_hand_type_joker(hand_type: &HandType, joker_count: u32) -> HandType {
        if joker_count == 0 {
            return hand_type.clone();
        }

        let new_hand_type = match hand_type {
            HandType::HighCard => HandType::OnePair,
            HandType::OnePair => HandType::ThreeOfKind,
            HandType::TwoPair => HandType::FullHouse,
            HandType::ThreeOfKind => HandType::FourOfKind,
            HandType::FourOfKind => HandType::FiveOfKind,
            _ => panic!("Unexpected hand type: {:?}, joker count: {}", hand_type, joker_count)
        };

        return Hand::determine_hand_type_joker(&new_hand_type, joker_count - 1);
    } 

    fn determine_hand_type(hand: &str, joker: bool) -> HandType {
        // Count card occurences
        let mut card_hmap: HashMap<char, u32> = HashMap::new();
        for c in hand.chars() {
           let card_cnt = *card_hmap.get(&c).unwrap_or(&0);

           card_hmap.insert(c, card_cnt + 1);
        }

        // Remove jokers if we are using them
        let mut joker_count = 0;
        if joker {
            if let Some(cnt) = card_hmap.remove(&'J') {
                joker_count = cnt;
            }
        }

        let card_cnts: Vec<u32> = card_hmap.into_iter().map(|(_, value)| value).collect();

        // Put the card counts in another hashmap to count them :D
        let mut card_cnt_hmap: HashMap<u32, u32> = HashMap::new();
        for cnt in card_cnts {
            let card_cnt_cnt = *card_cnt_hmap.get(&cnt).unwrap_or(&0);

            card_cnt_hmap.insert(cnt, card_cnt_cnt + 1);
        }

        let unique_ones = *card_cnt_hmap.get(&1).unwrap_or(&0);
        let unique_pairs: u32 = *card_cnt_hmap.get(&2).unwrap_or(&0);
        let unique_threes: u32 = *card_cnt_hmap.get(&3).unwrap_or(&0);
        let unique_fours: u32 = *card_cnt_hmap.get(&4).unwrap_or(&0);
        let unique_fives: u32 = *card_cnt_hmap.get(&5).unwrap_or(&0);

        let mut hand_type: HandType = 
            if unique_fives == 1 {
                HandType::FiveOfKind
            } else if unique_fours == 1 {
                HandType::FourOfKind
            } else if unique_threes == 1 {
                if unique_pairs == 1 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfKind
                }
            } else if unique_pairs == 2 {
                HandType::TwoPair
            } else if unique_pairs == 1 {
                HandType::OnePair
            } else if unique_ones >= 1 && unique_ones <= 5 { // Because of jokers this can now be 1-5
                HandType::HighCard
            } else if joker && joker_count == 5 {
                return HandType::FiveOfKind
            } else {
                println!("{:?}", card_cnt_hmap);
                panic!("Unknown hand type!");
            };

        if joker && joker_count > 0 {
            hand_type = Hand::determine_hand_type_joker(&hand_type, joker_count);
        }

        return hand_type;
    }

    fn new(hand: String, bid: u64, joker: bool) -> Hand {
        let hand_type = Hand::determine_hand_type(&hand, joker);

        Hand { hand: hand, bid: bid, hand_type: hand_type, joker: joker }
    }

    fn parse(line: &str, joker: bool) -> Hand {
        let mut split_line = line.split(" ");
        let hand_str = split_line.next().unwrap().to_owned();
        let bid = split_line.next().unwrap().parse::<u64>().unwrap();

        Hand::new(hand_str, bid, joker)
    }

    pub fn parse_all(file: &str, joker: bool) -> Vec<Hand> {
        let lines = aoc_helper::read_lines(file);

        lines.iter().map(|line| Hand::parse(line, joker)).collect()
    }
}