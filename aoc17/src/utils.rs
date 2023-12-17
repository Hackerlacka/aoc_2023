use std::collections::{VecDeque, HashMap};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    North = 0,
    West = 1,
    South = 2,
    East = 3
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct EnterBlockInfo {
    direction: Direction,
    single_direction_step_count: i32
}

struct Block {
    heat_loss: u64,
    heat_loss_map: HashMap<EnterBlockInfo, u64> // Maps "enter block info" to accumulated_heat_loss
}

struct TestBlock {
    pos: (usize, usize),
    accumulated_heat_loss: u64,
    enter_block_info: EnterBlockInfo,
    path: Vec<((usize, usize), Direction)>
}

pub struct HeatLossMap {
    map: Vec<Vec<Block>>,
    height: usize,
    width: usize
}

const DIRECTION_COUNT: u32 = 4;
const SINGLE_DIRECTION_STEP_COUNT_MAX: i32 = 3;

const ULTRA_CRUCIBLE_SINGLE_DIR_STEP_COUNT_MIN: i32 = 4;
const ULTRA_CRUCIBLE_SINGLE_DIR_STEP_COUNT_MAX: i32 = 10;


impl Direction {
    fn from_direction(direction: Direction, left_or_right: Direction) -> Direction {
        let mut direction_number: i32;

        if left_or_right == Direction::West {
            direction_number = (direction as i32 + 1) % (DIRECTION_COUNT as i32);
        } else if left_or_right == Direction::East {
            direction_number = (direction as i32 - 1) % (DIRECTION_COUNT as i32);
        } else {
            panic!("Direction is not left or right (west or east)");
        }

        // Modulo can return negative number
        if direction_number < 0 {
            direction_number += 4; 
        }

        // TODO: Yikes!
        match direction_number {
            0 => Direction::North,
            1 => Direction::West,
            2 => Direction::South,
            3 => Direction::East,
            _ => panic!("Direction number out of bounds: {}", direction_number)
        }
    }
}

impl Block {
    fn new(heat_loss: u64) -> Block {
        Block { heat_loss: heat_loss, heat_loss_map: HashMap::new() }
    }
}

impl TestBlock {
    fn create_new_relative_to(original: &TestBlock, direction: Direction, height: usize, width: usize) -> Option<TestBlock> {
        // Determine new position
        let new_direction: Direction;
        if direction == Direction::North { // Forwards
            new_direction = original.enter_block_info.direction;
        } else { // Left or right
            new_direction = Direction::from_direction(original.enter_block_info.direction, direction);
        }

        // Check bounds of new position
        let mut new_pos = original.pos;
        match new_direction {
            Direction::North => {
                if original.pos.0 <= 0 {
                    return None;
                }
                new_pos.0 -= 1;
            },
            Direction::West => {
                if original.pos.1 <= 0 {
                    return None;
                }
                new_pos.1 -= 1;
            },
            Direction::South => {
                if original.pos.0 + 1 >= height {
                    return None;
                }
                new_pos.0 += 1;
            },
            Direction::East => {
                if original.pos.1 + 1 >= width {
                    return None;
                }
                new_pos.1 += 1;
            }
        }

        // Create new test block
        let mut test_block = TestBlock::new(new_pos, new_direction);
        test_block.accumulated_heat_loss = original.accumulated_heat_loss;
        if direction == Direction::North { // Inherit single_direction_step_count if going in same dir. This is increased when the move happens
            test_block.enter_block_info.single_direction_step_count = original.enter_block_info.single_direction_step_count;
        }
        test_block.path = original.path.clone();

        return Some(test_block);
    }

    fn new(pos: (usize, usize), direction: Direction) -> TestBlock {
        let enter_block_info = EnterBlockInfo { direction: direction, single_direction_step_count: 0 };
        TestBlock { pos: pos, accumulated_heat_loss: 0, enter_block_info: enter_block_info, path: Vec::new() }
    }
}

// Normal crucible
// At most three blocks in one direction
// Can't reverse direction either
impl HeatLossMap {
    fn single_dir_step_count_max(ultra_crucible: bool) -> i32 {
        if ultra_crucible {
            ULTRA_CRUCIBLE_SINGLE_DIR_STEP_COUNT_MAX
        } else {
            SINGLE_DIRECTION_STEP_COUNT_MAX
        }
    }

    fn single_dir_step_count_min(ultra_crucible: bool) -> i32 {
        if ultra_crucible {
            ULTRA_CRUCIBLE_SINGLE_DIR_STEP_COUNT_MIN
        } else {
            0
        }
    }

    fn get_block_mut(&mut self, pos: &(usize, usize)) -> &mut Block {
        &mut self.map[pos.0][pos.1]
    }

    fn get_block(&self, pos: &(usize, usize)) -> &Block {
        &self.map[pos.0][pos.1]
    }

    fn push_back_if_some(queue: &mut VecDeque<TestBlock>, test_block: Option<TestBlock>) {
        if let Some(tmp) = test_block {
            queue.push_back(tmp);
        }
    }

    fn add_new_blocks_to_test(height: usize, width: usize, test_block: &TestBlock, queue: &mut VecDeque<TestBlock>, ultra_crucible: bool) {
        // Check if we can walk in the same direction
        if test_block.enter_block_info.single_direction_step_count < Self::single_dir_step_count_max(ultra_crucible) {
            Self::push_back_if_some(queue, TestBlock::create_new_relative_to(test_block, Direction::North, height, width));
        }

        // Left & right
        if test_block.enter_block_info.single_direction_step_count >= Self::single_dir_step_count_min(ultra_crucible) {
            Self::push_back_if_some(queue, TestBlock::create_new_relative_to(test_block, Direction::West, height, width));
            Self::push_back_if_some(queue, TestBlock::create_new_relative_to(test_block, Direction::East, height, width));
        }
    }

    fn enter_block(&mut self, mut test_block: TestBlock, queue: &mut VecDeque<TestBlock>, ultra_crucible: bool) {
        let block: &mut Block = self.get_block_mut(&test_block.pos);

        // Increase heat loss
        test_block.accumulated_heat_loss += block.heat_loss;

        // Check if block has been entered before, in the same direction and same single_direction_step_count
        if let Some(accumulated_heat_loss) = block.heat_loss_map.get(&test_block.enter_block_info) {
            if *accumulated_heat_loss <= test_block.accumulated_heat_loss {
                // There is already a better way!
                return;
            }
            // Note: Any offspring of the previously best "test block" could still be in the queue and slow down performance!
        }
        block.heat_loss_map.insert(test_block.enter_block_info, test_block.accumulated_heat_loss);

        test_block.path.push((test_block.pos, test_block.enter_block_info.direction));

        // Add new blocks to try/test
        Self::add_new_blocks_to_test(self.height, self.width, &test_block, queue, ultra_crucible);
    }

    // Ultra crucible
    // Move a minimum of four block in single direction before turn (even in the end!)
    // Move a maximum of ten blocks in single direction 

    pub fn find_lowest_heat_loss_path(&mut self, ultra_crucible: bool) -> u64 {
        let start_pos: (usize, usize) = (0, 0);
        let end_pos: (usize, usize) = (self.height - 1, self.width - 1);

        let mut lowest_heat_loss = u64::MAX;
        let mut lowest_head_loss_test_block = TestBlock::new(start_pos, Direction::East);
        let mut queue: VecDeque<TestBlock> = VecDeque::new();

        let mut start_test_block_east = TestBlock::new(start_pos, Direction::East);
        start_test_block_east.enter_block_info.single_direction_step_count = -1; // To compensate for instant +1
        let mut start_test_block_south = TestBlock::new(start_pos, Direction::South);
        start_test_block_south.enter_block_info.single_direction_step_count = -1;


        queue.push_back(start_test_block_east);
        queue.push_back(start_test_block_south);

        let start_block_heat_loss = self.get_block(&start_pos).heat_loss;
        let end_block_heat_loss = self.get_block(&end_pos).heat_loss;

        while let Some(mut test_block) = queue.pop_front() {
            // Increase step count here so (min/max step count) comparison becomes correct
            test_block.enter_block_info.single_direction_step_count += 1;

            // Don't walk to far in the same direction
            if test_block.enter_block_info.single_direction_step_count > Self::single_dir_step_count_max(ultra_crucible) {
                continue;
            }

            if test_block.pos == end_pos {
                // Remove start block heat loss and add end block heat loss
                test_block.accumulated_heat_loss += end_block_heat_loss - start_block_heat_loss;
                
                // Verify that the ultra crucible can stop here
                if test_block.enter_block_info.single_direction_step_count < Self::single_dir_step_count_min(ultra_crucible) {
                    continue;
                }

                test_block.path.push((test_block.pos, test_block.enter_block_info.direction));

                if test_block.accumulated_heat_loss < lowest_heat_loss {
                    lowest_heat_loss = test_block.accumulated_heat_loss;
                    lowest_head_loss_test_block = test_block;
                }

                continue;
            }
            
            self.enter_block(test_block, &mut queue, ultra_crucible);
        }
        
        // Note: for debugging, slows down performance pretty much!
        let mut it = lowest_head_loss_test_block.path.iter();
        let mut prev_path = it.next().unwrap();
        print!("{:?} ", prev_path.0);
        it.for_each(|path| {
            if prev_path.1 != path.1 {
                println!();
            }
            print!("{:?} ", path.0);
            prev_path = path;
        });
        println!();

        return lowest_heat_loss;
    }

    pub fn parse(file: &str) -> HeatLossMap {
        let line = aoc_helper::read_lines(file);
        let map = line.into_iter().map(|line| {
            line.chars().map(|c| {
                Block::new(c.to_digit(10).unwrap() as u64)
            }).collect::<Vec<Block>>()
        }).collect::<Vec<Vec<Block>>>();

        let height = map.len();
        let width = map.first().unwrap().len();

        HeatLossMap { map: map, height: height, width: width }
    }
}