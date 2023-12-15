use std::collections::VecDeque;

struct Lens {
    label: String,
    focal_length: u64
}

struct LensBox {
    box_number: usize,
    lens_queue: VecDeque<Lens>
}

pub struct InitSequence {
    steps: Vec<String>
}

impl Lens {
    fn calculate_focusing_power(&self, box_number: usize, slot_number: usize, ) -> u64 {
        ((box_number + 1) * slot_number) as u64 * self.focal_length
    }
}

impl LensBox {
    fn calculcate_focusing_power(&self) -> u64 {
        let mut sum = 0;

        for (i, lens) in self.lens_queue.iter().enumerate() {
            sum += lens.calculate_focusing_power(self.box_number, i + 1);
        }

        return sum;
    }

    fn perform_dash_op(&mut self, label: &str) {
        let mut i: isize = -1;
        for (j, lens) in self.lens_queue.iter().enumerate() {
            if label == lens.label {
                i = j as isize;
                break;
            }
        }

        if i > -1 {
            self.lens_queue.remove(i as usize);
        }
    }

    fn perform_equal_op(&mut self, label: &str, focal_length: u64) {
        let mut i: isize = -1;
        for (j, lens) in self.lens_queue.iter().enumerate() {
            if label == lens.label {
                i = j as isize;
                break;
            }
        }

        if i > -1 {
            self.lens_queue[i as usize].focal_length = focal_length;
        } else {
            self.lens_queue.push_back(Lens { label: label.to_string(), focal_length: focal_length })
        }
    }

    fn empty(box_number: usize) -> LensBox {
        LensBox { lens_queue: VecDeque::new(), box_number: box_number }
    }
}

impl InitSequence {
    fn hash_char(mut current_val: u64, c: char) -> u64 {
        current_val += c as u64;
        current_val *= 17;
        current_val %= 256;

        return current_val;
    }

    fn hash_step(s: &str) -> u64 {
        let mut current_val = 0;

        for c in s.chars() {
            current_val = Self::hash_char(current_val, c);
        }

        return current_val;
    }

    fn perform_step(boxes: &mut Vec<LensBox>, step: &str) {
        let is_dash_op = step.contains('-');
        let label: &str;
        let mut focal_len: u64 = 0;

        if is_dash_op {
            label = step.split('-').next().unwrap();
        } else {
            let mut split = step.split('=');
            label = split.next().unwrap();
            focal_len = split.next().unwrap().parse().unwrap();
        }

        let box_number = Self::hash_step(label) as usize;
        let b = &mut boxes[box_number];

        if is_dash_op {
            b.perform_dash_op(label);
        } else {
            if focal_len == 0 || focal_len > 9 {
                panic!("Unexpected folcal length!");
            }
            b.perform_equal_op(label, focal_len);
        }
    }

    pub fn calculate_focusing_power(&self) -> u64 {
        let mut boxes = Vec::new();
        for i in 0..256 {   
            boxes.push(LensBox::empty(i));
        }

        self.steps.iter().for_each(|step| Self::perform_step(&mut boxes, step));

        let focusing_power = boxes.iter().map(|lens_box| lens_box.calculcate_focusing_power()).sum::<u64>();

        return focusing_power;
    }

    pub fn hash_steps(&self) -> Vec<u64> {
        self.steps.iter().map(|step| Self::hash_step(&step)).collect()
    }

    pub fn parse(file: &str) -> InitSequence {
        let line = aoc_helper::read_lines(file).first().unwrap().clone();
        let steps: Vec<String> = line.split(',').map(|s| s.to_owned()).collect();

        InitSequence { steps: steps }
    }
}