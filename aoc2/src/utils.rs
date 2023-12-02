use std::fs::read_to_string;
use regex::Regex;

pub struct GameSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32
}

pub struct Game {
    pub id: u32,
    sets: Vec<GameSet>
}

impl GameSet {
    pub fn empty_set() -> GameSet {
        GameSet { red: 0, green: 0, blue: 0 }
    }

    pub fn parse_game_set(line: &str) -> GameSet {
        let mut game_set = GameSet::empty_set();
    
        // Separate colors 
        let colors: Vec<&str> = line.split(",").collect();
        for color in colors.iter() {
            // Separate count and color
            let color_split: Vec<&str> = color.trim().split(" ").collect();
    
            if color_split.len() != 2 {
                panic!();
            }
    
            let count: u32 = color_split[0].parse().unwrap();
    
            match color_split[1] {
                "red" => game_set.red = count,
                "green" => game_set.green = count,
                "blue" => game_set.blue = count,
                _ => panic!(),
            }
        }
    
        return game_set;
    }


    pub fn power(&self) -> u32 {
        return self.red * self.green * self.blue;
    }
}

impl Game {
    pub fn calculate_min_bag(&self) -> GameSet {
        let mut bag = GameSet::empty_set();

        for game_set in self.sets.iter() {
            if game_set.red > bag.red {
                bag.red = game_set.red;
            }
            if game_set.green > bag.green {
                bag.green = game_set.green;
            }
            if game_set.blue > bag.blue {
                bag.blue = game_set.blue;
            }
        }

        return bag
    }

    fn parse_game_sets(line: &str) -> Vec<GameSet> {
        let re = Regex::new(r"Game [0-9]+: (.*)$").unwrap();
    
        for (_, [game_sets_line]) in re.captures_iter(line).map(|c| c.extract()) {
           let game_sets_split: Vec<&str> = game_sets_line.split(";").collect();
    
           return game_sets_split.iter().map(|line| GameSet::parse_game_set(line)).collect();
        }
    
        panic!();
    }
    
    fn parse_game_id(line: &str) -> u32 {
        let re = Regex::new(r"Game ([0-9]+): .*$").unwrap();
    
        for (_, [game_id]) in re.captures_iter(line).map(|c| c.extract()) {
            return game_id.parse::<u32>().unwrap();
        }
    
        panic!();
    }
    
    fn parse_game(line: &str) -> Game {
        let game_id = Self::parse_game_id(line);
        let game_sets = Self::parse_game_sets(line);
    
        return Game { id: game_id, sets: game_sets };
    }
    
    pub fn parse_games(file: &str) -> Vec<Game> {
        let mut games: Vec<Game> = Vec::new();
    
        for line in read_to_string(file).unwrap().lines() {
            games.push(Self::parse_game(line));
        }
    
        return games;
    }

    pub fn is_possible(&self, bag: &GameSet) -> bool {
        let max_bag = self.calculate_min_bag();

        return bag.red >= max_bag.red && bag.green >= max_bag.green && bag.blue >= max_bag.blue; 
    }
    
    pub fn find_possible_games(bag: GameSet, games: &Vec<Game>) -> Vec<&Game> {
       return games.iter().filter(|g| g.is_possible(&bag)).collect();
    }
}
