use std::collections::{HashSet, VecDeque};
use std::fs::OpenOptions;
use std::io::Write;

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

    // TODO: Move to aoc_helper?
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

    // TODO: Remove?
    fn get_all_garden_plots_reached_count(&self, steps: u64) -> Vec<u64> {
        let unique_plots_reached = self.calculate_unique_plots_reached(steps, true);

        let mut res: Vec<_> = Vec::new();
        for step in 0..(steps + 1) {
            res.push(Self::calculate_unique_pos(step, &unique_plots_reached));
        }

        return res;
    }

    pub fn output_garden_plots_reached_difference(&self, steps: u64) {
        let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open("aoc21_part2.txt")
        .unwrap();

        let unique_plots_reached = self.calculate_unique_plots_reached(steps, true);

        let counts: Vec<usize> = unique_plots_reached.iter().map(|set| set.len()).collect();

        let width_height = self.map.len();

        let mut diffs = Vec::new();
        for (i, count) in counts.iter().enumerate() {
            let diff: i64;
            if i < width_height {
                diff = *count as i64;
            } else {
                diff = *count as i64 - counts[i - width_height] as i64;
            }
            diffs.push(diff);

            if let Err(e) = writeln!(file, "{}", diff) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }

        // let garden_plots_reached_count = self.get_all_garden_plots_reached_count(steps);
        // let width_height = self.map.len();
        // let mut diffs = Vec::new();
        // for (i, count) in garden_plots_reached_count.iter().enumerate() {
        //     let diff: i64;
        //     if i == 0 {
        //         diff = *count as i64;
        //     } else {
        //         diff = *count as i64 - garden_plots_reached_count[i - 1] as i64;
        //     }
        //     diffs.push(diff);

        //     if let Err(e) = writeln!(file, "{}", diff) {
        //         eprintln!("Couldn't write to file: {}", e);
        //     }
        // }

        // if let Err(e) = writeln!(file, "") {
        //     eprintln!("Couldn't write to file: {}", e);
        // }

        // let mut diff_diffs = Vec::new();
        // for (i, tmp_diff) in diffs.iter().enumerate() {
        //     let diff: i64;
        //     if i < width_height {
        //         diff = *tmp_diff;
        //     } else {
        //         diff = tmp_diff - diffs[i - width_height];
        //     }
        //     diff_diffs.push(diff);

        //     if let Err(e) = writeln!(file, "{}", diff) {
        //         eprintln!("Couldn't write to file: {}", e);
        //     }
        // }

        

    }


    fn calculate_a_b_numbers_n(counts: &Vec<usize>, diffs: &Vec<i64>, pattern_start: usize, n: usize, steps_modulus: u64) -> (i64, i64) {
        let a_start_index = pattern_start - n;
        let a_all_numbers = &counts[a_start_index..(a_start_index + n)];
        let a_partial_sum: i64 = a_all_numbers.iter().enumerate().map(|(i, val)| {
            let j = a_start_index + i; // TODO: If n is odd, one of these need to add +1
            if j % 2 == steps_modulus as usize {
                *val as i64
            } else {
                0
            }
        }).sum();

        let b_all_numbers = &diffs[pattern_start..(pattern_start + n)];
        let b_partial_sum: i64 = b_all_numbers.iter().enumerate().map(|(i, val)| {
            let j = pattern_start + i;
            if j % 2 == steps_modulus as usize {
                *val as i64
            } else {
                0
            }
        }).sum();

        return (a_partial_sum, b_partial_sum);
    }

    fn find_pattern_start_n(values: &Vec<i64>, n: usize) -> usize {
        let mut pattern_start = 0;
        for i in 0..values.len() {
            if i + n >= values.len() {
                panic!("Found no pattern!");
            }

            // TODO: Choosing n here would perhaps be expensive?
            if (0..5).map(|j| values[i + j] == values[i + j + n]).all(|b| b) {
                println!("Possible pattern found at i: {} + {}", i, n);
                pattern_start = i;
                break;
            }
        }

        return pattern_start;
    }

    fn calculate_diff_n(values: &Vec<usize>, n: usize) -> Vec<i64> {
        let mut diffs = Vec::new();

        for (i, count) in values.iter().enumerate() {
            let diff: i64;
            if i < n {
                diff = *count as i64;
            } else {
                diff = *count as i64 - values[i - n] as i64;
            }
            diffs.push(diff);
        }

        return diffs;
    }

    pub fn calculate_garden_plots_reached_opt(&self, steps: u64) -> u64 {
        let steps_calculated = 750; // TODO: Not sure how to make this adjust after "steps"
        let unique_plots_reached = self.calculate_unique_plots_reached(steps_calculated, true);

        // Step 0 == ..., step 1 == ..., ..., step n = ...
        let counts: Vec<usize> = unique_plots_reached.iter().map(|set| set.len()).collect(); 
        
        // Map width is same as height
        let width_height = self.map.len();
        
        // Calculate diffs between every n elements (n = width_height)
        let diffs = Self::calculate_diff_n(&counts, width_height);

        // Find pattern in the diffs
        let pattern_start = Self::find_pattern_start_n(&diffs, width_height);

        // To know garden plots reached for "steps" (large) we need all unique_plots_reached on the way, that are on the same modulus (even or odd)
        let steps_modulus = steps % 2;

        let (a_partial_sum, b_partial_sum) = Self::calculate_a_b_numbers_n(&counts, &diffs, pattern_start, width_height, steps_modulus);
        
        let steps_remaining = steps - pattern_start as u64 + 1; // +1 to include the step we can aiming for
        let repetitions = (steps_remaining / width_height as u64) as i64;

        // Using "natural numbers" formula i.e. sum k=1..n for k = n(n + 1) / 2
        let sum_repeat = repetitions * a_partial_sum + repetitions * (repetitions + 1) / 2 * b_partial_sum;
        println!("Sum repeat: {}", sum_repeat);

        // 18319050 - current result
        // 16733044 - correct output

        // let steps_remaining = steps - (pattern_start as u64 - 1);
        // let repetitions = steps_remaining / width_height as u64;
        // let repetitions_remainder = steps_remaining % width_height as u64;

        // println!("Repetitions: {}, remainder: {}", repetitions, repetitions_remainder);
        
        // // Using "natural numbers" formula i.e. sum k=1..n for k = n(n + 1) / 2
        // let right_sum = repetitions * a_sum + repetitions * (repetitions + 1) / 2 * b_sum;
        // let left_sum: u64 = counts.iter().enumerate().map(|(i, c)| {
        //     if i >= pattern_start {
        //         return 0;
        //     }

        //     if (i % 2 + 1) as u64 == mod_steps { // TODO: correct?
        //         return *c as u64;
        //     } else {
        //         return 0;
        //     }

        // }).sum();

        // // TODO: have not even added remainder
        // let a_partial_sum: u64 = a_all.iter().enumerate().map(|(i, a_single)| {
        //     if i as u64 >= repetitions_remainder {
        //         return 0;
        //     }

        //     if ((a_start_index + i + 1) % 2) as u64 == mod_steps { // TODO: correct?
        //         *a_single as u64
        //     } else {
        //         0
        //     }
        // }).sum();

        // let b_partial_sum: u64 = b_all.iter().enumerate().map(|(i, b_single)| {
        //     if i as u64 >= repetitions_remainder {
        //         return 0;
        //     }

        //     if ((pattern_start + i + 1) % 2) as u64 == mod_steps { // TODO: correct?
        //         *b_single as u64
        //     } else {
        //         0
        //     }
        // } ).sum();

        // let remainder_sum = (repetitions + 1) * a_partial_sum + (repetitions + 1) * (repetitions + 1 + 1) / 2 * b_partial_sum;

        // let sum = left_sum + right_sum + remainder_sum;

        // println!("Sum is: {}", sum);
        // 18320514
        // 15123714
        // 16733044 - correct output

        return 0; // TODO: Fix
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