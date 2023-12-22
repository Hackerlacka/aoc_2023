use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use regex::Regex;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
    z: usize
}

#[derive(Debug, Eq, PartialEq, Hash)]
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
        if self.start.x - self.end.x > 0 {
            for x in self.start.x..(self.end.x + 1) {
                surface_coords.push(Pos { x: x, y: self.start.y, z: self.start.z });
            }
        } else if self.start.y - self.end.y > 0 {
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

    fn determine_disintegration_count_priv(bricks: &Vec<Brick>, dependencies: &HashMap<&Brick, Vec<&Brick>>) -> u64 {

        return 0; // TODO: Change
    }

    // TODO: Is it not needed to add 'a to the top_settled_bricks Pos? Feels not complete to have it on the HashMap...
    fn find_possible_collision_coords(top_settled_bricks: &HashMap<Pos, &Brick>, surface_coords: &Vec<Pos>) -> Vec<Pos> {
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

    fn settle_brick<'a>(brick: &'a mut Brick, top_settled_bricks: &mut HashMap<Pos, &'a Brick>) -> Option<Vec<&'a Brick>> {
        let mut surface_coords = brick.get_surface_coords();
        let possible_collision_coords = Self::find_possible_collision_coords(top_settled_bricks, &surface_coords);

        let collision_z = possible_collision_coords.iter().map(|coord| coord.z).max().unwrap_or(0);

        // Determine which bricks this brick is dependent on
        let mut brick_dependencies = Vec::new();
        for possible_collision_coord in possible_collision_coords.into_iter() {
            if let Some(other_brick) = top_settled_bricks.remove(&possible_collision_coord) {
                // Keep as dependent brick if z is at collision z height
                if possible_collision_coord.z == collision_z {
                    brick_dependencies.push(other_brick);
                }
            }
        }

        // Update brick z coordinates
        brick.update_start_z(collision_z + 1);

        // Update surface coordinates to become "roof" surface coordinates
        surface_coords.iter_mut().for_each(|surface_coord| surface_coord.z = brick.end.z);

        // Add "roof" surface coordinates
        for surface_coord in surface_coords.into_iter() {
            top_settled_bricks.insert(surface_coord, brick);
        }

        if brick_dependencies.len() > 0 {
            return Some(brick_dependencies);
        }

        return None;
    }

    fn settle_bricks(bricks: &mut Vec<Brick>) -> HashMap<&Brick, Vec<&Brick>> {
        let mut dependencies: HashMap<&Brick, Vec<&Brick>> = HashMap::new();
        let mut top_settled_bricks: HashMap<Pos, &Brick> = HashMap::new();

        let mut it = bricks.iter_mut();
        while let Some(brick) = it.next() {
            if let Some(brick_dependencies) = Self::settle_brick(brick, &mut top_settled_bricks) {
                dependencies.insert(brick, brick_dependencies);
            }
        }

        return dependencies;
    }

    fn sort_bricks_on_z(bricks: &mut Vec<Brick>) {
        bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z));
    }

    pub fn determine_disintegration_count(&mut self) -> u64 {
        Self::sort_bricks_on_z(&mut self.bricks);

        self.print(); // TODO: remove

        let dependencies = Self::settle_bricks(&mut self.bricks);

        return Self::determine_disintegration_count_priv(&self.bricks, &dependencies);
    }

    pub fn parse(file: &str) -> BrickSnapshot {
        let lines = aoc_helper::read_lines(file);
        let bricks = lines.iter().map(|line| Brick::parse(line)).collect();

        // let bricks_2: Vec<Rc<RefCell<Brick>>> = lines.iter().map(|line| Rc::new(RefCell::new(Brick::parse(line)))).collect();
        // bricks_2.first().unwrap()

        BrickSnapshot { bricks: bricks }
    }
}