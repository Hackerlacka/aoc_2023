use std::collections::VecDeque;

enum Direction {
    Up,
    Left,
    Down,
    Right
}

struct Tile {
    c: char,
}

struct DigInstruction {
    direction: Direction,
    meters: u64
    // TODO: color
}

pub struct DigPlan {
    instructions: Vec<DigInstruction>
}

impl Direction {
    fn new(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'L' => Direction::Left,
            'D' => Direction::Down,
            'R' => Direction::Right,
            _ => panic!("Unexpected char: {}", c)
        }
    }
}

impl DigInstruction {
    fn parse(line: &str) -> DigInstruction {
        let mut split_line = line.split(" ");

        let direction = Direction::new(split_line.next().unwrap().chars().next().unwrap());
        let meters = split_line.next().unwrap().parse::<u64>().unwrap();

        DigInstruction { direction: direction, meters: meters }
    }
}

impl DigPlan {
    fn count_non_visited_tiles(map: &mut Vec<Vec<Tile>>) -> u64 {
        let mut sum = 0;

        for line in map.iter() {
            for tile in line.iter() {
                if tile.c != '*' {
                    sum += 1;
                }
            }
        }

        return sum;
    }

    fn visit_tile(tile: &mut Tile, pos: (usize, usize), height: usize, width: usize, queue: &mut VecDeque<(usize, usize)>) {
        if tile.c == '#' || tile.c == '*' {
            return;
        }

        // Mark visited tiles with star
        tile.c = '*';

        // A bit inefficient, but add all directions :D
        if pos.0 > 0 {
            queue.push_back((pos.0 - 1, pos.1));
        }
        if pos.0 + 1 < height {
            queue.push_back((pos.0 + 1, pos.1));
        }
        if pos.1 > 0 {
            queue.push_back((pos.0, pos.1 - 1));
        }
        if pos.1 + 1 < width {
            queue.push_back((pos.0, pos.1 + 1));
        }
    }

    fn determine_capcity(map: &mut Vec<Vec<Tile>>) -> u64 {
        // Old good flood fill (requires an empty ring around map though)

        let height = map.len();
        let width = map.first().unwrap().len();

        let start_pos = (0, 0);
        let mut queue = VecDeque::new();

        queue.push_back(start_pos);

        while let Some(pos) = queue.pop_front() {
            Self::visit_tile(&mut map[pos.0][pos.1], pos, height, width, &mut queue);
        }

        return Self::count_non_visited_tiles(map);
    }

    fn expand_map(map: &mut Vec<Vec<Tile>>) {
        let height = map.len();
        let width = map.first().unwrap().len();

        let up = map.first().unwrap().iter().any(|tile| tile.c == '#');
        let down = map[height - 1].iter().any(|tile| tile.c == '#');
        let left = map.iter().map(|line| line.first().unwrap()).any(|tile| tile.c == '#');
        let right = map.iter().map(|line| &line[width - 1]).any(|tile| tile.c == '#');

        if up {
            let top_vec: Vec<Tile> = (0..width).into_iter().map(|_| Tile { c: '.'}).collect();
            map.insert(0, top_vec);
            //height = map.len();
        }
        
        if down {
            let top_vec: Vec<Tile> = (0..width).into_iter().map(|_| Tile { c: '.'}).collect();
            map.push(top_vec);
            //height = map.len();
        }

        for line_vec in map.iter_mut() {
            if left {
                line_vec.insert(0, Tile { c: '.' })
            }
            if right {
                line_vec.push( Tile { c: '.' })
            }
        }
    }
    
    fn create_map(trench: &Vec<(usize, usize)>, upper_bounds_inclusive: &(usize, usize)) -> Vec<Vec<Tile>> {
        let mut map = Vec::new();
        for _ in 0..upper_bounds_inclusive.0 + 1 {
            let mut line_vec = Vec::new();
            for _ in 0..upper_bounds_inclusive.1 + 1 {
                line_vec.push(Tile { c: '.' })
            }
            map.push(line_vec);
        }

        // Mark trench
        for coordinates in trench.iter() {
            map[coordinates.0][coordinates.1].c = '#';
        }

        return map;
    }

    fn determine_upper_bounds(trench: &Vec<(usize, usize)>) -> (usize, usize) {
        let mut upper_bounds = (0, 0);
        for coordinates in trench.iter() {
            if coordinates.0 > upper_bounds.0 {
                upper_bounds.0 = coordinates.0;
            }
            if coordinates.1 > upper_bounds.1 {
                upper_bounds.1 = coordinates.1
            }
        }

        return upper_bounds;
    }

    fn determine_lower_bounds(trench: &Vec<(isize, isize)>) -> (isize, isize) {
        let mut lower_bounds = (isize::MAX, isize::MAX);
        for coordinates in trench.iter() {
            if coordinates.0 < lower_bounds.0 {
                lower_bounds.0 = coordinates.0;
            }
            if coordinates.1 < lower_bounds.1 {
                lower_bounds.1 = coordinates.1
            }
        }

        return lower_bounds;
    }

    fn convert_trench_to_usize(mut trench: Vec<(isize, isize)>) -> Vec<(usize, usize)> {
        let lower_bounds = Self::determine_lower_bounds(&trench);

        // Bump all coordinates up by the negative lower bounds
        for coordinates in trench.iter_mut() {
            if lower_bounds.0.is_negative() {
                coordinates.0 -= lower_bounds.0;
            }
            if lower_bounds.1.is_negative() {
                coordinates.1 -= lower_bounds.1;
            }
        }

        trench.iter().map(|coordinates| (coordinates.0 as usize, coordinates.1 as usize)).collect()
    }

    fn dig_out_trench(instructions: &Vec<DigInstruction>) -> Vec<(usize, usize)> {
        let mut coordinates: (isize, isize) = (0, 0);
        let mut trench = Vec::new();

        // Push initial coordinates
        trench.push(coordinates);

        for instruction in instructions.iter() {
            for _ in 0..instruction.meters {
                match instruction.direction {
                    Direction::Up => coordinates.0 -= 1,
                    Direction::Left => coordinates.1 -= 1,
                    Direction::Down => coordinates.0 += 1,
                    Direction::Right => coordinates.1 += 1
                }
                trench.push(coordinates);
            }
        }

        return Self::convert_trench_to_usize(trench);
    }

    fn shoelace_formula(trench: &Vec<(usize, usize)>) -> u64 {
        let mut sum: i64 = 0;

        for i in 0..trench.len() {
            let prev: (usize, usize);
            if i == 0 {
                prev = *trench.last().unwrap();
            } else {
                prev = trench[i - 1];
            }
            let current = trench[i];
            let next: (usize, usize);
            if i == trench.len() - 1 {
                next = *trench.first().unwrap();
            } else {
                next = trench[i + 1];
            }

            sum += current.0 as i64 * (next.1 as i64 - prev.1 as i64)
        }

        return (sum / 2).abs() as u64;
    }

    fn dig_out_trench_part_2(instructions: &Vec<DigInstruction>) -> Vec<(usize, usize)> {
        let mut coordinates: (isize, isize) = (0, 0);
        let mut trench = Vec::new();

        // Push initial coordinates
        trench.push(coordinates);

        // TODO: Need to find the "outside" coordinates...!

        for instruction in instructions.iter() {
            for _ in 0..(instruction.meters + 1) { // + 1 to compensate for coordinate system?
                match instruction.direction {
                    Direction::Up => coordinates.0 -= 1,
                    Direction::Left => coordinates.1 -= 1,
                    Direction::Down => coordinates.0 += 1,
                    Direction::Right => coordinates.1 += 1
                }
                trench.push(coordinates);
            }
        }

        return Self::convert_trench_to_usize(trench);
    }

    pub fn test_part_2(&self) {
        let mut trench = Self::dig_out_trench_part_2(&self.instructions);
        //trench.remove(trench.len() - 1); // TODO: restore?
        let area = Self::shoelace_formula(&trench);

        println!("Area is: {}", area);
    }

    pub fn get_lava_capacity(&self) -> u64 {
        let trench = Self::dig_out_trench(&self.instructions);
        let upper_bounds = Self::determine_upper_bounds(&trench);
        let mut map = Self::create_map(&trench, &upper_bounds);
        Self::expand_map(&mut map); // Note: can always cut it back into size so that trench is valid

        return Self::determine_capcity(&mut map);
    }

    pub fn parse(file: &str) -> DigPlan {
        let lines = aoc_helper::read_lines(file);
        let instructions = lines.iter().map(|line| DigInstruction::parse(line)).collect();

        return DigPlan { instructions: instructions };
    }
}