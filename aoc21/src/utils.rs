use std::collections::{HashSet, VecDeque};

pub struct FarmMap {
    map: Vec<Vec<char>>
}

impl FarmMap {
    fn calculate_unique_pos(steps: u64, unique_plots_reached: &Vec<HashSet<(isize, isize)>>) -> u64 {
        let mut sum = 0;

        let mod_res = steps % 2;
        for tmp_step in 0..(steps + 1) {
            if tmp_step % 2 != mod_res {
                continue;
            }
            sum += unique_plots_reached[tmp_step as usize].len();
        }

        return sum as u64;
    }

    fn find_start_position(map: &Vec<Vec<char>>) -> (isize, isize) {
        for (y, line) in map.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == 'S' {
                    return (y as isize, x as isize);
                }
            }
        } 
        panic!("Could not find start position");
    }

    fn positive_modulo(a: isize, b: usize) -> usize {
        let mut res = a % (b as isize);
        if res < 0 {
            res += b as isize;
        }

        return res as usize;
    }

    fn move_pos(map: &Vec<Vec<char>>, pos: &(isize, isize), step: u64, unique_plots_reached: &Vec<HashSet<(isize, isize)>>, next_queue: &mut VecDeque<(isize, isize)>, infinite: bool) {
        let height = map.len();
        let width = map.first().unwrap().len();

        let mut positions_to_test = Vec::new();

        if !infinite {
            if pos.0 > 0 {
                positions_to_test.push((pos.0 - 1, pos.1));
            }
            if pos.0 + 1 < height as isize {
                positions_to_test.push((pos.0 + 1, pos.1));
            }
            if pos.1 > 0 {
                positions_to_test.push((pos.0, pos.1 - 1));
            }
            if pos.1 + 1 < width as isize {
                positions_to_test.push((pos.0, pos.1 + 1));
            }
        } else {
            positions_to_test.push((pos.0 - 1, pos.1));
            positions_to_test.push((pos.0 + 1, pos.1));
            positions_to_test.push((pos.0, pos.1 - 1));
            positions_to_test.push((pos.0, pos.1 + 1));
        }

        // Check for rocks
        for i in (0..positions_to_test.len()).rev() {
            let tmp_pos = positions_to_test[i];
            if map[Self::positive_modulo(tmp_pos.0, height)][Self::positive_modulo(tmp_pos.1, width)] == '#' {
                positions_to_test.remove(i);
            }
        }

        // Start checking if farm has been reached before
        if step >= 1 {
            'outer: for i in (0..positions_to_test.len()).rev() {
                let tmp_pos = positions_to_test[i];

                let mod_res = (step + 1) % 2;
                for tmp_step in 0..step {
                    if tmp_step % 2 != mod_res {
                        continue;
                    }

                    // If tmp_pos already exists, remove it and check the next one
                    if unique_plots_reached[tmp_step as usize].contains(&tmp_pos) {
                        positions_to_test.remove(i);
                        continue 'outer;
                    }
                }
            }
        }

        positions_to_test.into_iter().for_each(|tmp_pos| next_queue.push_back(tmp_pos));
    }

    fn calculate_unique_plots_reached(&self, steps: u64, infinite: bool) -> Vec<HashSet<(isize, isize)>> {
        let start_pos = Self::find_start_position(&self.map);

        let mut unique_plots_reached = Vec::new();
        (0..(steps + 1)).for_each(|_| unique_plots_reached.push(HashSet::new()));

        let mut queue = VecDeque::new();
        queue.push_back(start_pos);
        for step in 0..(steps + 1) {
            let mut next_queue = VecDeque::new();

            while let Some(pos) = queue.pop_front() {
                if !unique_plots_reached[step as usize].insert(pos) {
                    continue;
                }
                Self::move_pos(&self.map, &pos, step, &unique_plots_reached, &mut next_queue, infinite);
            }

            // Transfer queue items for next step
            queue.append(&mut next_queue);
        }

        return unique_plots_reached;
    }

    pub fn calculate_garden_plots_reached_opt(&self, steps: u64) -> u64 {
        let map_len = self.map.len() as u64;
        let half_map_len = map_len / 2 as u64;

        // Reachable tiles (ignoring rocks) (2n + 1)^2 / 2
        // +1 for center tile

        // Farm plots reached, T(x) = ax^2 + bx + c
        // Gather three sample points
        let sample_points = vec![half_map_len, half_map_len + map_len, half_map_len + 2 * map_len];
        let plots_reached_sp: Vec<u64> = sample_points.iter().map(|sp| {
            self.calculate_garden_plots_reached(*sp, true)
        }).collect();

        // Derive constants a, b and c
        // P(0) = a * 0^2 + b * 0 + c == c
        let c = plots_reached_sp[0];

        // b = (4P(1) - 3P(0) - P(2)) / 2
        let b = (4 * plots_reached_sp[1] - 3 * plots_reached_sp[0] - plots_reached_sp[2]) / 2;

        // a = P(1) - P(0) - b
        let a = plots_reached_sp[1] - plots_reached_sp[0] - b;

        // x is the number of whole map lens
        let x = (steps - half_map_len) / map_len;

        let result = a * x.pow(2) + b * x + c;

        return result;
    }

    pub fn calculate_garden_plots_reached(&self, steps: u64, infinite: bool) -> u64 {
        let unique_plots_reached = self.calculate_unique_plots_reached(steps, infinite);

        return Self::calculate_unique_pos(steps, &unique_plots_reached);
    }

    pub fn parse(file: &str) -> FarmMap {
        let lines = aoc_helper::read_lines(file);
        let map: Vec<Vec<char>> = lines.into_iter().map(|line| line.chars().collect()).collect();

        FarmMap { map: map }
    }
}