use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;

use regex::Regex;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
    z: usize
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Brick {
    start: Pos,
    end: Pos
}

pub struct BrickSnapshot {
    bricks: Vec<Brick>
}

impl Pos {
    fn from_strs(x: &str, y: &str, z: &str) -> Pos {
        Pos { x: x.parse().unwrap(), y: y.parse().unwrap(), z: z.parse().unwrap() }
    }
}

impl Brick {
    fn update_start_z(&mut self, new_z: usize) {
        let z_diff = self.end.z - self.start.z;
        self.start.z = new_z;
        self.end.z = new_z + z_diff;
    }

    fn get_surface_coords(&self) -> Vec<Pos> {
        // "Each brick is made up of a single straight line of cubes"
        let mut surface_coords = Vec::new();
        if self.end.x - self.start.x > 0 {
            for x in self.start.x..(self.end.x + 1) {
                surface_coords.push(Pos { x: x, y: self.start.y, z: self.start.z });
            }
        } else if self.end.y - self.start.y > 0 {
            for y in self.start.y..(self.end.y + 1) {
                surface_coords.push(Pos { x: self.start.x, y: y, z: self.start.z });
            }
        } else {
            surface_coords.push(self.start);
        }

        return surface_coords;
    }

    fn parse(line: &str) -> Brick {
        let re = Regex::new(r"([0-9]+),([0-9]+),([0-9]+)~([0-9]+),([0-9]+),([0-9]+)").unwrap();
        for (_, [sx, sy, sz, ex, ey, ez]) in re.captures_iter(line).map(|c| c.extract()) {
            let start = Pos::from_strs(sx, sy, sz);
            let end = Pos::from_strs(ex, ey, ez);

            return Brick { start: start, end: end };
        }

        panic!("Could not parse Brick from: {}", line);
    }
}

impl BrickSnapshot {
    fn print(&self) {
        for brick in self.bricks.iter() {
            println!("{:?}", brick);
        }
    }

    fn determine_non_disintegral_bricks(dependencies: &HashMap<Brick, Vec<Brick>>) -> Vec<Brick> {
        let mut non_disintegral_bricks = HashSet::new();

        for value in dependencies.values() {
            if value.len() == 1 {
                non_disintegral_bricks.insert(value.first().unwrap().clone());
            }
        }

        return non_disintegral_bricks.into_iter().collect();
    }

    fn determine_disintegration_count_priv(bricks: &Vec<Brick>, dependencies: &HashMap<Brick, Vec<Brick>>) -> u64 {
        let mut brick_carriers = HashSet::new();
        for value in dependencies.values() {
            value.iter().for_each(|brick| { brick_carriers.insert(brick.clone()); });
        }

        let mut disintegratable_bricks = HashSet::new();

        // Add brick carriers that could possibly be disintegrated
        for value in dependencies.values() {
            if value.len() > 1 {
                value.iter().for_each(|brick| { disintegratable_bricks.insert(brick.clone()); });
            }
        }

        // Remove brick carriers that cannot be disintegrated
        for value in dependencies.values() {
            if value.len() == 1 {
                disintegratable_bricks.remove(value.first().unwrap());
            }
        }

        // Add the bricks that are not carriers
        for brick in bricks.iter() {
            if !brick_carriers.contains(brick) {
                disintegratable_bricks.insert(brick.clone());
            }
        }

        return disintegratable_bricks.len() as u64;
    }

    fn find_possible_collision_coords(top_settled_bricks: &HashMap<Pos, Brick>, surface_coords: &Vec<Pos>) -> Vec<Pos> {
        let mut possible_collision_coords = Vec::new();

        for surface_coord in surface_coords.iter() {
            for settled_pos in top_settled_bricks.keys() {
                if surface_coord.x == settled_pos.x && surface_coord.y == settled_pos.y {
                    possible_collision_coords.push(settled_pos.clone());
                }
            }
        }

        return possible_collision_coords;
    }

    fn settle_brick(brick: &mut Brick, top_settled_bricks: &mut HashMap<Pos, Brick>) -> Option<Vec<Brick>> {
        let mut surface_coords = brick.get_surface_coords();
        let possible_collision_coords = Self::find_possible_collision_coords(top_settled_bricks, &surface_coords);

        let collision_z = possible_collision_coords.iter().map(|coord| coord.z).max().unwrap_or(0);

        // Determine which bricks this brick is dependent on
        let mut brick_dependencies = HashSet::new();
        for possible_collision_coord in possible_collision_coords.into_iter() {
            if let Some(other_brick) = top_settled_bricks.remove(&possible_collision_coord) {
                // Keep as dependent brick if z is at collision z height
                if possible_collision_coord.z == collision_z {
                    brick_dependencies.insert(other_brick);
                }
            }
        }

        // Update brick z coordinates
        brick.update_start_z(collision_z + 1);

        // Update surface coordinates to become "roof" surface coordinates
        surface_coords.iter_mut().for_each(|surface_coord| surface_coord.z = brick.end.z);

        // Add "roof" surface coordinates
        for surface_coord in surface_coords.into_iter() {
            top_settled_bricks.insert(surface_coord, brick.clone());
        }

        if brick_dependencies.len() > 0 {
            return Some(brick_dependencies.into_iter().collect());
        }

        return None;
    }

    fn settle_bricks(bricks: &mut Vec<Brick>) -> HashMap<Brick, Vec<Brick>> {
        let mut dependencies: HashMap<Brick, Vec<Brick>> = HashMap::new();
        let mut top_settled_bricks: HashMap<Pos, Brick> = HashMap::new();

        let mut it = bricks.iter_mut();
        while let Some(brick) = it.next() {
            if let Some(brick_dependencies) = Self::settle_brick(brick, &mut top_settled_bricks) {
                dependencies.insert(brick.clone(), brick_dependencies);
            }
        }

        return dependencies;
    }

    fn sort_bricks_on_z(bricks: &mut Vec<Brick>) {
        bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z));
    }

    pub fn determine_bricks_that_would_fall(&mut self) -> u64 {
        Self::sort_bricks_on_z(&mut self.bricks);

        let dependencies = Self::settle_bricks(&mut self.bricks);

        let non_disintegral_bricks = Self::determine_non_disintegral_bricks(&dependencies);

        let mut dependencies_vec = Vec::new();
        dependencies.into_iter().for_each(|(dep_brick, bricks)| dependencies_vec.push((dep_brick, bricks)));
        dependencies_vec.sort_by(|a, b| a.0.start.z.cmp(&b.0.start.z));

        let mut sum: u64 = 0;
        for disintegrating_brick in non_disintegral_bricks.iter() {
            let mut fallers: Vec<Brick> = Vec::new();
            fallers.push(disintegrating_brick.clone());
            
            for (dep_brick, bricks) in dependencies_vec.iter() {
                if bricks.len() == 1 {
                    for brick in bricks {
                        if fallers.contains(brick) {
                            fallers.push(dep_brick.clone());
                        }
                    }
                } else if bricks.len() > 1 {
                    // If all carriers of this brick exists in fallers, add this one too
                    if bricks.iter().map(|brick| fallers.contains(brick)).all(|b| b) {
                        fallers.push(dep_brick.clone());
                    }
                }
            }

            sum += fallers.len() as u64 - 1;
        }

        return sum;
    }

    pub fn determine_disintegration_count(&mut self) -> u64 {
        Self::sort_bricks_on_z(&mut self.bricks);

        let dependencies = Self::settle_bricks(&mut self.bricks);

        return Self::determine_disintegration_count_priv(&self.bricks, &dependencies);
    }

    pub fn parse(file: &str) -> BrickSnapshot {
        let lines = aoc_helper::read_lines(file);
        let bricks = lines.iter().map(|line| Brick::parse(line)).collect();

        BrickSnapshot { bricks: bricks }
    }
}