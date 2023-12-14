use std::collections::HashMap;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum Direction {
    NORTH,
    WEST,
    SOUTH,
    EAST
}

pub struct RockMap {
    map: Vec<Vec<char>>
}

static ROUNDED_ROCK: char = 'O';
//static CUBE_ROCK: char = '#';
static EMPTY: char = '.';

impl RockMap {
    pub fn print(&self) {
        self.map.iter().for_each(|line| println!("{:?}", line));
    }

    pub fn calculate_north_load(&self) -> u64 {
        let mut score = 0;
        for (i, line_vec) in self.map.iter().enumerate() {
            let row_single_rock_load = (self.map.len() - i) as u64;
            score += (line_vec.iter().filter(|c| **c == ROUNDED_ROCK).count() as u64) * row_single_rock_load;
        }

        return score;
    }

    fn transform(&mut self, direction: &Direction) {
        let width = self.map.first().unwrap().len();

        if *direction == Direction::NORTH {
            // Do nothing?
        } else if *direction == Direction::WEST {
            let mut new_map = Vec::new();
            for x in 0..width {
                let new_row: Vec<char> = self.map.iter_mut().map(|row| row[x]).rev().collect();
                new_map.push(new_row);
            }
            self.map = new_map;
        } else if *direction == Direction::SOUTH {
            let height = self.map.len();

            for x in 0..width {
                let tmp_col: Vec<char> = self.map.iter().map(|line| line[x]).rev().collect();
                
                for (y, c) in tmp_col.into_iter().enumerate() {
                    self.map[y][x] = c;
                }
            }
        }  else if *direction == Direction::EAST {
            let mut new_map = Vec::new();
            for x in (0..width).rev() {
                let new_row: Vec<char> = self.map.iter_mut().map(|row| row[x]).collect();
                new_map.push(new_row);
            }
            self.map = new_map;
        } else {
            panic!("Unknown transform direction");
        }
    }

    fn tilt(&mut self, direction: Direction) {
        let mut tmp_direction = direction;
        self.transform(&tmp_direction);
        self.tilt_north();
        if tmp_direction == Direction::WEST {
            tmp_direction = Direction::EAST;
        } else if tmp_direction == Direction::EAST {
            tmp_direction = Direction::WEST;
        }
        self.transform(&tmp_direction);
    }

    pub fn tilt_north(&mut self) {
        let width = self.map.first().unwrap().len();

        for y in 0..self.map.len() {
            for x in 0..width {
                let c = self.map[y][x];

                if c != ROUNDED_ROCK {
                    continue;
                }

                let tmp_part_col: Vec<char> = self.map[0..y].iter().map(|line| line[x]).collect();
                let mut new_row = 0;
                for i in (0..tmp_part_col.len()).rev() {
                    if tmp_part_col[i] != EMPTY {
                        new_row = i + 1;
                        break;
                    }
                }

                self.map[y][x] = EMPTY;
                self.map[new_row][x] = ROUNDED_ROCK;
            }
        }
    }

    pub fn spin_cycle(&mut self) {
        self.tilt(Direction::NORTH);
        self.tilt(Direction::WEST);
        self.tilt(Direction::SOUTH);
        self.tilt(Direction::EAST);
    }

    fn find_cycle(&mut self, n: u32) -> (u32, u32) {
        let mut hm = HashMap::new();

        for i in 0..n {
            self.spin_cycle();

            let hash = aoc_helper::hash_ref(&self.map);
            if hm.contains_key(&hash) {
                return (i, *hm.get(&hash).unwrap());
            } else {
                hm.insert(hash, i);
            }
        }

        panic!("Found no cycle!");
    }

    pub fn spin_n_cycles(&mut self, n: u32) {
        let cycle = self.find_cycle(n);
        println!("Cycle: {:?}", cycle);

        let cycle_len = cycle.0 - cycle.1;

        let n_remaining = n - (cycle.0 + 1);
        let cycles_mod = n_remaining % cycle_len;

        for _ in 0..cycles_mod {
            self.spin_cycle();
        }
    }

    pub fn parse(file: &str) -> RockMap {
        let lines = aoc_helper::read_lines(file);
        let mut map = Vec::new();

        for line in lines {
            let line_vec: Vec<char> = line.chars().collect();

            map.push(line_vec);
        }

        RockMap { map: map }
    }
}