use std::collections::VecDeque;


#[derive(Debug, Copy, Clone)]
enum Direction {
    North = 0,
    West = 1,
    South = 2,
    East = 3
}

struct Tile {
    c: char,
    visited: Vec<bool>
}

pub struct Contraption {
    layout: Vec<Vec<Tile>>,
    height: usize,
    width: usize
}

const EMPTY: char = '.';
const MIRROR_1: char = '/';
const MIRROR_2: char = '\\';
const SPLITTER_VERTICAL: char = '|';
const SPLITTER_HORIZONTAL: char = '-';

impl Tile {
    fn register_precense(&mut self, heading_dir: &Direction) {
        self.visited[*heading_dir as usize] = true;
    }

    fn prevent_loop(&self, heading_dir: &Direction) -> bool {
        self.visited[*heading_dir as usize]
    }

    fn new(c: char) -> Tile {
        let visited = vec![false, false, false, false];
        Tile { c: c, visited: visited }
    } 
}

impl Contraption {
    fn reset_tiles(&mut self){
        self.layout.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|tile| {
                tile.visited.iter_mut().for_each(|b| *b = false);
            });
        });
    }

    fn calculate_energized_tiles(&self) -> u64 {
        let mut sum = 0;

        self.layout.iter().for_each(|row| {
            row.iter().for_each(|tile| {
                if tile.visited.iter().any(|b| *b == true) {
                    sum += 1;
                }
            });
        });

        return sum;
    }

    fn calculate_next_position(pos: &(usize, usize), next_direction: Direction, height: usize, width: usize) -> Option<(usize, usize)> {
        let mut next_pos = (pos.0, pos.1);
        
        match next_direction {
            Direction::North => {
                if next_pos.0 > 0 {
                    next_pos.0 -= 1;
                }
            },
            Direction::South => {
                if next_pos.0 + 1 < height {
                    next_pos.0 += 1;
                }
            },
            Direction::West => {
                if next_pos.1 > 0 {
                    next_pos.1 -= 1;
                }
            },
            Direction::East => {
                if next_pos.1 + 1 < width {
                    next_pos.1 += 1;
                }
            },
        }

        if next_pos == *pos {
            return None;
        }

        return Some(next_pos);
    }

    fn next_directions(c: char, heading_dir: Direction) -> Vec<Direction> {
        match c {
            EMPTY => return vec![heading_dir],
            MIRROR_1 => {
                return match heading_dir {
                    Direction::North => vec![Direction::East],
                    Direction::West => vec![Direction::South],
                    Direction::South => vec![Direction::West],
                    Direction::East => vec![Direction::North]
                };
            },
            MIRROR_2 => {
                return match heading_dir {
                    Direction::North => vec![Direction::West],
                    Direction::West => vec![Direction::North],
                    Direction::South => vec![Direction::East],
                    Direction::East => vec![Direction::South]
                };
            },
            SPLITTER_VERTICAL => {
                return match heading_dir {
                    Direction::East | Direction::West => vec![Direction::North, Direction::South],
                    _ => vec![heading_dir] // North and south
                };
            },
            SPLITTER_HORIZONTAL => {
                return match heading_dir {
                    Direction::North | Direction::South => vec![Direction::West, Direction::East],
                    _ => vec![heading_dir] // West and east
                };
            },
            _ => panic!("Unexpected char: {}", c)
        }
    }

    fn energize_tile(&mut self, pos: (usize, usize), heading_dir: Direction, queue: &mut VecDeque<((usize, usize), Direction)>) {
        let tile = &mut self.layout[pos.0][pos.1];

        // Prevent loops + register precense
        if tile.prevent_loop(&heading_dir) {
            return;
        }

        tile.register_precense(&heading_dir);

        let next_directions = Self::next_directions(tile.c, heading_dir);

        for next_direction in next_directions.into_iter() {
            let next_pos = Self::calculate_next_position(&pos, next_direction, self.height, self.width);

            if next_pos.is_none() {
                continue;
            }

            queue.push_back((next_pos.unwrap(), next_direction));
        }
    }

    pub fn find_max_tiles_energized(&mut self) -> u64 {
        let mut vec_energized_tiles = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if y == 0 {
                    vec_energized_tiles.push(self.test_energize_tiles((y, x), Direction::South));
                } else if y == self.height - 1 {
                    vec_energized_tiles.push(self.test_energize_tiles((y, x), Direction::North));
                }
    
                if x == 0 {
                    vec_energized_tiles.push(self.test_energize_tiles((y, x), Direction::East));
                } else if x == self.width - 1 {
                    vec_energized_tiles.push(self.test_energize_tiles((y, x), Direction::West));
                }
            }
        }

        return *vec_energized_tiles.iter().max().unwrap();
    }

    fn test_energize_tiles(&mut self, start_pos: (usize, usize), start_direction: Direction) -> u64 {
        // Use queue approach instead of recursion (otherwise stack overflow)
        let mut queue = VecDeque::new();
        queue.push_back((start_pos, start_direction));

        while let Some((pos, direction)) = queue.pop_front() {
            self.energize_tile(pos, direction, &mut queue);
        }

        let energized_tiles = self.calculate_energized_tiles();

        self.reset_tiles();

        return energized_tiles;
    }

    pub fn energize_tiles(&mut self) -> u64 {
        self.test_energize_tiles((0, 0), Direction::East)
    }

    pub fn parse(file: &str) -> Contraption {
        let lines = aoc_helper::read_lines(file);
        let layout: Vec<Vec<Tile>> = lines.into_iter().map(|line| {
            line.chars().map(|c| Tile::new(c)).collect::<Vec<Tile>>() // Line vec
        }).collect();

        let height = layout.len();
        let width = layout.first().unwrap().len();

        Contraption { layout: layout, height: height, width: width }
    }
}