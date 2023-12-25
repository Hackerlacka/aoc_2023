use std::{rc::Rc, collections::VecDeque};

enum SlopeType {
    North,
    West,
    South,
    East
}

enum TileType {
    Path,
    Forest,
    Slope(SlopeType)
}

type Pos = (usize, usize);

struct TestTile {
    pos: Pos,
    cost: u64,
    visited_pos: Vec<Pos>,
    previously_visited_pos: Vec<Rc<Vec<Pos>>>
}

struct Tile {
    tile_type: TileType,
    //highest_cost: u64
}

pub struct TrailMap {
    map: Vec<Vec<Tile>>
}

impl SlopeType {
    fn parse(c: char) -> Option<SlopeType> {
        match c {
            '^' => Some(SlopeType::North),
            '<' => Some(SlopeType::West),
            'v' => Some(SlopeType::South),
            '>' => Some(SlopeType::East),
            _ => None
        }
    }
}

impl TestTile {
    fn from_other_single(other: TestTile, pos: Pos) -> TestTile {
        TestTile { pos: pos, cost: other.cost, visited_pos: other.visited_pos, previously_visited_pos: other.previously_visited_pos }
    }

    fn from_other_multiple(other: TestTile, positions: Vec<Pos>) -> VecDeque<TestTile> {
        let visited_pos_rc = Rc::new(other.visited_pos);

        return positions.into_iter().map(|pos| {
            let mut previously_visited_pos = other.previously_visited_pos.clone(); // TODO: Does this clone RCs?
            previously_visited_pos.push(Rc::clone(&visited_pos_rc));
            TestTile { pos: pos, cost: other.cost, visited_pos: Vec::new(), previously_visited_pos: previously_visited_pos }
        }).collect();
    }

    fn new(pos: Pos) -> TestTile {
        TestTile { pos: pos, cost: 0, visited_pos: Vec::new(), previously_visited_pos: Vec::new() }
    }
}

impl Tile {
    fn parse(c: char) -> Tile {
        let tile_type = match c {
            '.' => TileType::Path,
            '#' => TileType::Forest,
            _ => TileType::Slope(SlopeType::parse(c).unwrap())
        };

        Tile { tile_type: tile_type }
    }
}

impl TrailMap {
    fn has_been_on_tile(pos: &Pos, test_tile: &TestTile) -> bool {
        if test_tile.visited_pos.contains(pos) 
            || test_tile.previously_visited_pos.iter().any(|visited| visited.contains(pos)) {
            return true;
        }

        // let len: usize = test_tile.visited_pos.len() + test_tile.previously_visited_pos.iter().map(|visited| visited.len()).sum::<usize>();
        // println!("Has been on tile with len: {}", len);

        return false;
    }

    fn add_next_steps(test_tile: TestTile, map: &Vec<Vec<Tile>>, test_tile_queue: &mut VecDeque<TestTile>, ignore_slopes: bool) {
        let height = map.len();
        let width = map.first().unwrap().len();
        let mut possible_next_pos = Vec::new();

        let pos = &test_tile.pos;
        if pos.0 > 0 {
            possible_next_pos.push((pos.0 - 1, pos.1))
        }
        if pos.0 + 1 < height {
            possible_next_pos.push((pos.0 + 1, pos.1))
        }
        if pos.1 > 0 {
            possible_next_pos.push((pos.0, pos.1 - 1))
        }
        if pos.1 + 1 < width {
            possible_next_pos.push((pos.0, pos.1 + 1))
        }

        for i in (0..possible_next_pos.len()).rev() {
            let next_pos = &possible_next_pos[i];

            // Remove visited tiles visited
            if Self::has_been_on_tile(next_pos, &test_tile) {
                possible_next_pos.remove(i);
                continue;
            }
            
            // Check tile type
            let tile_type = &map[next_pos.0][next_pos.1].tile_type;
            match tile_type {
                TileType::Forest => {
                    possible_next_pos.remove(i);
                    continue;
                },
                TileType::Slope(slope_type) => {
                    if ignore_slopes {
                        continue;
                    }

                    // Check if it is possible to walk (depending on direction)
                    let y_diff = next_pos.0 as isize - pos.0 as isize;
                    let x_diff = next_pos.1 as isize - pos.1 as isize;
                    match slope_type {
                        SlopeType::North => {
                            if y_diff > 0 {
                                possible_next_pos.remove(i);
                                continue;
                            }
                        },
                        SlopeType::West => {
                            if x_diff > 0 {
                                possible_next_pos.remove(i);
                                continue;
                            }
                        },
                        SlopeType::South => {
                            if y_diff < 0 {
                                possible_next_pos.remove(i);
                                continue;
                            }
                        },
                        SlopeType::East => {
                            if x_diff < 0 {
                                possible_next_pos.remove(i);
                                continue;
                            }
                        }
                    }
                },
                _ => ()
            }
        }

        // Add new test tiles to queue
        if possible_next_pos.len() == 1 {
            test_tile_queue.push_back(TestTile::from_other_single(test_tile, possible_next_pos.remove(0)));
        } else if possible_next_pos.len() > 1 {
            // TODO: I guess this appends to the back of the queue?
            test_tile_queue.append(&mut TestTile::from_other_multiple(test_tile, possible_next_pos));
        }
    }

    fn move_to_tile(mut test_tile: TestTile, map: &mut Vec<Vec<Tile>>, test_tile_queue: &mut VecDeque<TestTile>, test_tile_finished: &mut Vec<TestTile>, end_pos: &Pos, ignore_slopes: bool) {
        // Add self to visited tiles
        test_tile.visited_pos.push(test_tile.pos);

        // Increase cost
        test_tile.cost += 1;
        
        // Check if at finish tile, if so add to test_tile_finished and return
        if test_tile.pos == *end_pos {
            test_tile_finished.push(test_tile);
            return;
        }

        // TODO: Check if tile has been visited before with more expensive cost (not possible?)
        // let pos = &test_tile.pos;
        // let tile = &mut map[pos.0][pos.1];
        // if tile.highest_cost > test_tile.cost {
        //     return;
        // }
        // tile.highest_cost = test_tile.cost;


        // Determine and add next steps
        Self::add_next_steps(test_tile, map, test_tile_queue, ignore_slopes);
    }

    fn determine_start_and_end_tile_pos(map: &Vec<Vec<Tile>>) -> (Pos, Pos) {
        let mut start_pos = None;
        for (x, tile) in map.first().unwrap().iter().enumerate() {
            match tile.tile_type {
                TileType::Path => start_pos = Some((0, x)),
                _ => continue
            }
        }

        let mut end_pos = None;
        for (x, tile) in map.last().unwrap().iter().enumerate() {
            match tile.tile_type {
                TileType::Path => end_pos = Some((map.len() - 1, x)),
                _ => continue
            }
        }

        return (start_pos.unwrap(), end_pos.unwrap());
    }

    pub fn find_longest_hike(&mut self, ignore_slopes: bool) -> u64 {
        // "if you step onto a slope tile, your next step must be downhill (in the direction the arrow is pointing)"
        // "never step onto the same tile twice"

        // By the looks of it, at every crossroad there are slopes that will prevent us from going back
        // but not mentioned in the text that it is guaranteed, so best not to assume it

        // Keep track of visited nodes, when entering a cross road, give each next node a reference to previous paths visited
        // i.e. use Rc (they can have vectors of multiple Rcs)
        // When testing new paths, always check back if the path has been visited before by this route
        // In crossroad nodes, also store the current maximum cost to there so that "cheaper" branches can stop testing

        let mut test_tile_queue = VecDeque::new();
        let mut test_tile_finished = Vec::new();

        // Determine start and end tile pos
        let (start_pos, end_pos) = Self::determine_start_and_end_tile_pos(&self.map);
        let start_test_tile = TestTile::new(start_pos);

        // Add start test tile to queue
        test_tile_queue.push_back(start_test_tile);

        // Test move to tiles
        while let Some(test_tile) = test_tile_queue.pop_front() {
            Self::move_to_tile(test_tile, &mut self.map, &mut test_tile_queue, &mut test_tile_finished, &end_pos, ignore_slopes);
        }

        let max_cost = test_tile_finished.iter().map(|test_tile| test_tile.cost).max().unwrap();
        return max_cost - 1; // -1 to remove start tile step
    }

    pub fn parse(file: &str) -> TrailMap {
        let lines = aoc_helper::read_lines(file);
        let map = lines.iter().map(|line| line.chars().map(|c| Tile::parse(c)).collect()).collect();

        TrailMap { map: map }
    }
}