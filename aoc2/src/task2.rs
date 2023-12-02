use crate::utils::Game;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_2_example_input() {
        let games = Game::parse_games("../input/2_1_example_input.txt");

        assert_eq!(games.len(), 5);

        let games_power: Vec<u32> = games.iter().map(|g| g.calculate_min_bag().power()).collect();
        
        assert_eq!(games_power, vec![48, 12, 1560, 630, 36]);
        
        let power_sum: u32 = games_power.iter().sum();

        assert_eq!(power_sum, 2286);
    }
}

pub fn run_task() {
    let games = Game::parse_games("input/2_1_input.txt");

    let games_power: Vec<u32> = games.iter().map(|g| g.calculate_min_bag().power()).collect();
    
    let power_sum: u32 = games_power.iter().sum();

    println!("Power sum is {}", power_sum);
}