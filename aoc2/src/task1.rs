use crate::utils::{Game, GameSet};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_2_example_input() {
        let games = Game::parse_games("../input/2_1_example_input.txt");

        assert_eq!(games.len(), 5);

        let possible_games = Game::find_possible_games(GameSet{ red: 12, green: 13, blue: 14 }, &games);
        
        assert_eq!(possible_games.len(), 3);

        let possible_game_ids: Vec<u32> = possible_games.iter().map(|g| g.id).collect();

        assert_eq!(possible_game_ids, vec![1, 2, 5]);

        let sum: u32 = possible_game_ids.iter().sum();

        println!("Sum is {}", sum);
    }
}

pub fn run_task() {
    let games = Game::parse_games("input/2_1_input.txt");

    let possible_games = Game::find_possible_games(GameSet{ red: 12, green: 13, blue: 14 }, &games);

    let possible_game_ids: Vec<u32> = possible_games.iter().map(|g| g.id).collect();

    let sum: u32 = possible_game_ids.iter().sum();
    println!("Sum is {}", sum);
}