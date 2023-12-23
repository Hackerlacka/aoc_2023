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
    // TODO: Could add "enter_from" for optimzation (or just check previously visited tile)
    pos: Pos,
    cost: u64,
    visited_pos: Vec<Pos>,
    previously_visited_pos: Vec<Rc<Vec<Pos>>>
}

struct Tile {
    tile_type: TileType
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
    fn move_to_tile(test_tile: TestTile, map: &mut Vec<Vec<Tile>>, test_tile_queue: &mut VecDeque<TestTile>, test_tile_finished: &mut Vec<TestTile>, end_pos: &Pos) {
        // TODO: Check if been at tile before

        // TODO: Add self to visited tiles

        // TODO: Increase cost
        
        // TODO: Check if at finish tile, if so add to test_tile_finished and return

        // TODO: Get tile

        // TODO: Add tile to visited tiles

        // TODO: Determine next steps + add them to test_tile_queue (Rc to visited paths)
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

    pub fn find_longest_hike(&mut self) -> u64 {
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
            Self::move_to_tile(test_tile, &mut self.map, &mut test_tile_queue, &mut test_tile_finished, &end_pos);
        }

        let max_cost = test_tile_finished.iter().map(|test_tile| test_tile.cost).max().unwrap();
        return max_cost;
    }

    pub fn parse(file: &str) -> TrailMap {
        let lines = aoc_helper::read_lines(file);
        let map = lines.iter().map(|line| line.chars().map(|c| Tile::parse(c)).collect()).collect();

        TrailMap { map: map }
    }
}