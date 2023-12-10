use std::{collections::{VecDeque, HashMap}, usize};

#[derive(Debug)]
#[derive(PartialEq)]
enum Direction {
    North,
    South,
    West,
    East
}

#[derive(Clone)]
pub struct Tile {
    pos: (usize, usize),
    c: char
}

pub struct PipeSketch {
    pub map: Vec<Vec<char>>,
    pub expanded_map: Vec<Vec<char>>,
    start_pos: (usize, usize)
}

impl Tile {
    fn expand_tile(tile: &mut Tile) {
        tile.pos = (tile.pos.0 * 2 + 1, tile.pos.1 * 2 + 1)
    }

    pub fn expand_tiles(tiles: &VecDeque<Tile>) -> VecDeque<Tile> {
        let mut tiles_copy = tiles.clone();

        tiles_copy.iter_mut().for_each(|tile| Tile::expand_tile(tile));

        let mut interpolated_tiles = VecDeque::new();
        let mut it = tiles_copy.iter().peekable();
        let mut it2 = tiles_copy.iter().peekable();
        
        while let Some(tile) = it.next() {
            let mut next_tile_wrapped = it.peek();

            if next_tile_wrapped.is_none() {
                next_tile_wrapped = it2.peek();
            }

            let next_tile = next_tile_wrapped.unwrap();

            let diff = (next_tile.pos.0 as isize - (tile.pos.0 as isize), next_tile.pos.1 as isize - (tile.pos.1 as isize));
            let new_pos_isize = (tile.pos.0 as isize + diff.0 / 2, tile.pos.1 as isize + diff.1 / 2);
            let new_pos = (new_pos_isize.0 as usize, new_pos_isize.1 as usize);

            interpolated_tiles.push_back(tile.clone());
            interpolated_tiles.push_back(Tile { pos: new_pos, c: tile.c });

        }


        return interpolated_tiles;
    }
}

impl Direction {
    fn determine_direction(curr_coords: &(usize, usize), next_coords: &(usize, usize)) -> Direction {
        if curr_coords.0 > next_coords.0 {
            return Direction::North
        } else if curr_coords.0 < next_coords.0 {
            return Direction::South;
        } else if curr_coords.1 > next_coords.1 {
            return Direction::West;
        } else if curr_coords.1 < next_coords.1 {
            return Direction::East;
        }

        panic!("Unkown direction");
    }
}

struct TestNode {
    visited: bool,
    directions_to_test: Vec<Direction> // TODO: remove this vector
}

impl PipeSketch {
    fn get_next_node_coords(curr_coords: &(usize, usize), next_coords: &(usize, usize), node: char) -> (usize, usize) {
        let mut res = (next_coords.0, next_coords.1);

        // Direction we are moving in from curr_coords -> next_coords
        let direction = Direction::determine_direction(curr_coords, next_coords);
        //println!("Direction: {:?}", direction);
        match node {
            '|' => {
                if direction == Direction::South {
                    res.0 += 1;
                } else if direction == Direction::North {
                    res.0 -= 1;
                } else {
                    panic!();
                }
            }
            '-' => {
                if direction == Direction::East {
                    res.1 += 1;
                } else if direction == Direction::West {
                    res.1 -= 1;
                } else {
                    panic!();
                }
            }
            'L' => {
                if direction == Direction::South {
                    res.1 += 1;
                } else if direction == Direction::West {
                    res.0 -= 1;
                } else {
                    panic!();
                }
            }
            'J' => {
                if direction == Direction::South {
                    res.1 -= 1;
                } else if direction == Direction::East {
                    res.0 -= 1;
                } else {
                    panic!();
                }
            }
            '7' => {
                if direction == Direction::East {
                    res.0 += 1;
                } else if direction == Direction::North {
                    res.1 -= 1;
                } else {
                    panic!();
                }
            }
            'F' => {
                if direction == Direction::North {
                    res.1 += 1;
                } else if direction == Direction::West {
                    res.0 += 1;
                } else {
                    panic!();
                }
            }
            _ => panic!("Unexpected node {}", node)
        }

        return res;
    }

    fn find_next_from_start_nodes(&self) -> Vec<(usize, usize)> {
        let mut next_nodes = Vec::new();
        let sp = self.start_pos;

        if sp.0 > 0 { // North
            let node = self.map[sp.0 - 1][sp.1];
            if node == '|' || node == '7' || node == 'F' {
                next_nodes.push((sp.0 - 1, sp.1))
            }
        }
        
        if sp.0 < self.map.len() - 1 { // South
            let node = self.map[sp.0 + 1][sp.1];
            if node == '|' || node == 'L' || node == 'J' {
                next_nodes.push((sp.0 + 1, sp.1))
            }
        }
        
        if sp.1 > 0 { // West
            let node = self.map[sp.0][sp.1 - 1];
            if node == '-' || node == 'L' || node == 'F' {
                next_nodes.push((sp.0, sp.1 - 1))
            }
        }
        
        if sp.1 < self.map.first().unwrap().len() - 1 { // East
            let node = self.map[sp.0][sp.1 + 1];
            if node == '-' || node == 'J' || node == '7' {
                next_nodes.push((sp.0, sp.1 + 1))
            }
        }

        return next_nodes;
    }

    pub fn get_main_loop(&self) -> VecDeque<Tile> {
        let mut main_loop = VecDeque::new();

        let mut next_from_start_nodes = self.find_next_from_start_nodes();
        if next_from_start_nodes.len() != 2 {
            panic!("Unexpected number of 'next from start nodes': {}", next_from_start_nodes.len());
        }

        main_loop.push_back(Tile { pos: self.start_pos, c: 'S' }); // Push start node
        
        let mut curr_node_coords = self.start_pos;
        let mut next_node_coords = next_from_start_nodes.pop().unwrap();
        while next_node_coords != self.start_pos {
            let next_node = self.map[next_node_coords.0][next_node_coords.1];
            //println!("Curr_node_coords: {:?}, Next_node_coords: {:?}, next_node {}", curr_node_coords, next_node_coords, next_node);

            main_loop.push_back(Tile { pos: next_node_coords, c: next_node }); // TODO: Or push front?

            let tmp_next_node_coords = Self::get_next_node_coords(&curr_node_coords, &next_node_coords, next_node);
            curr_node_coords = next_node_coords;
            next_node_coords = tmp_next_node_coords;
        }

        return main_loop;
    }

    fn find_start_pos(map: &Vec<Vec<char>>) -> (usize, usize) {
        for y in 0..map.len() {
            for x in 0..map.first().unwrap().len() {
                if map[y][x] == 'S' {
                    return (y, x);
                }
            }
        }

        return (0, 0);
    }

    fn test_node(map: &mut Vec<Vec<char>>, test_nodes: &mut Vec<Vec<TestNode>>, pos: (usize, usize)) {
        let height = test_nodes.len();
        let width = test_nodes.first().unwrap().len();

        test_nodes[pos.0][pos.1].visited = true;
        map[pos.0][pos.1] = 'O';

        let directions = vec![Direction::North, Direction::South, Direction::West, Direction::East];

        //println!("{:?}", pos);

        for direction in directions.iter() {
            let mut next_pos = (pos.0, pos.1);

            // Check map bounds
            if *direction == Direction::North {
                if pos.0 == 0 {
                    continue;
                }
                next_pos.0 -= 1;
            } else if *direction == Direction::South {
                if pos.0 == height - 1 {
                    continue;
                }
                next_pos.0 += 1;
            } else if *direction == Direction::West {
                if pos.1 == 0 {
                    continue;
                }
                next_pos.1 -= 1;
            } else if *direction == Direction::East {
                if pos.1 == width - 1 {
                    continue;
                }
                next_pos.1 += 1;
            }

            let next_test_node = &test_nodes[next_pos.0][next_pos.1];
            if next_test_node.visited || map[next_pos.0][next_pos.1] == 'X' { // Skip if visited or is part of main loop
                continue;
            }

            Self::test_node(map, test_nodes, next_pos);
        }

        //println!("return {:?}", pos);

    }

    pub fn fill_reachable_tiles_ext_map(&mut self) {
        // Keep track of visited nodes
        let mut test_nodes = Vec::new();
        for y in 0..self.expanded_map.len() {
            let mut line = Vec::new();
            for x in 0..self.expanded_map.first().unwrap().len() {
                let directions_to_test = vec![Direction::North, Direction::South, Direction::West, Direction::East];
                line.push(TestNode {visited: false, directions_to_test: directions_to_test});
            }
            test_nodes.push(line);
        }

        Self::test_node(&mut self.expanded_map, &mut test_nodes, (0, 0));
    }

    pub fn print_map(map: &Vec<Vec<char>>) {
        for line in map.iter() {
            for c in line.iter() {
                print!("{}", c);
            }
            println!()
        }
    }

    pub fn add_overlay_to_exp_map(&mut self, overlay: &VecDeque<Tile>) {
        for tile in overlay.iter() {
            self.expanded_map[tile.pos.0][tile.pos.1] = 'X';
        }
    }

    pub fn print_overlay(map: &Vec<Vec<char>>, overlay: &VecDeque<Tile>) {
        let mut map = map.clone();

        for tile in overlay.iter() {
            map[tile.pos.0][tile.pos.1] = 'X';
        }

        Self::print_map(&map);
    }

    pub fn count_empty_spaces(&self) -> usize {
        let mut count = 0;
        for y in 0..self.map.len() {
            for x in 0..self.map.first().unwrap().len() {
                let c = self.map[y][x];

                if c != 'X' && c != 'O' {
                    count += 1;
                }
            }
        }

        return count;
    }

    pub fn shrink_expanded_map(&mut self) {
        for y in 0..self.expanded_map.len() {
            for x in 0..self.expanded_map.first().unwrap().len() {
                if y % 2 == 1 && x % 2 == 1 {
                    self.map[y / 2][x / 2] = self.expanded_map[y][x];
                }
            }
        }
    }

    pub fn create_expanded_map(&mut self) {
        let mut map = &mut self.expanded_map;

        let filler_char = '=';

        // Push first line
        let map_width = self.map.first().unwrap().len() * 2 + 1;
        let extra_line: Vec<char> = filler_char.to_string().repeat(map_width).chars().collect();
        map.push(extra_line.clone());
        
        // Push the actual content + sides
        for line in self.map.iter() {
            let mut line_vec = Vec::new();
            for c in line.iter() {
                line_vec.push(filler_char);
                line_vec.push(*c);
            }
            line_vec.push(filler_char);

            map.push(line_vec);
            map.push(extra_line.clone())
        }
    }

    pub fn parse(file: &str) -> PipeSketch {
        let lines = aoc_helper::read_lines(file);
        let mut map = Vec::new();

        for line in lines {
            let line_vec: Vec<char> = line.chars().collect();
            map.push(line_vec);
        }

        let start_pos = Self::find_start_pos(&map);
        
        PipeSketch { map: map, start_pos: start_pos, expanded_map: Vec::new() }
    }
}