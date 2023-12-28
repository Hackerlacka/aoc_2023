use regex::Regex;
use float_cmp::approx_eq;

pub type Pos = (i64, i64, i64);
type FPos = (f64, f64, f64);

enum Condition {
    GTE(i64),
    LTE(i64)
}

struct LinearEq {
    m: f64, // Slope
    b: f64,
    x_condition: Condition,
    y_condition: Condition
}

#[derive(Debug)]
struct Hailstone {
    start_pos: Pos,
    velocity: Pos
}

pub struct HailstoneMap {
    hail: Vec<Hailstone>
}

impl Condition {
    fn is_fulfilled(&self, value: f64) -> bool {
        match self {
            Condition::GTE(condition_val) => value >= *condition_val as f64,
            Condition::LTE(condition_val) => value <= *condition_val as f64
        }
    }
}

impl LinearEq {
    fn compare_intersections(a: &FPos, b: &FPos) -> bool {
        let ulps = 1000; // Note: Not sure what a reasonable value is here
        return approx_eq!(f64, a.0, b.0, ulps = ulps) && approx_eq!(f64, a.1, b.1, ulps = ulps) && approx_eq!(f64, a.2, b.2, ulps = ulps);
    }

    fn intersection(&self, other: &Self) -> Option<FPos> {
        let x_factor = self.m - other.m;

        if x_factor == 0.0 { // Note: dangerous comparison!
            //println!("X factor is equal!");
            return None
        }

        let x = (other.b - self.b) / x_factor;
        let y = self.m * x + self.b;

        Some((x, y, 0f64))
    }

    fn future_intersection(&self, other: &Self) -> Option<FPos> {
        let intersection = self.intersection(other);

        if intersection.is_none() {
            return None;
        }

        let intersection_unwrapped = intersection.unwrap();

        if self.x_condition.is_fulfilled(intersection_unwrapped.0) && self.y_condition.is_fulfilled(intersection_unwrapped.1) &&
           other.x_condition.is_fulfilled(intersection_unwrapped.0) && other.y_condition.is_fulfilled(intersection_unwrapped.1) {
            return Some(intersection_unwrapped);
        }

        return None;
    }

    fn from_two_points(p1: &Pos, p2: &Pos) -> LinearEq {
        let m = (p2.1 - p1.1) as f64 / ((p2.0 - p1.0) as f64); // slope
        let b = (p1.1 as f64) - m * (p1.0 as f64);

        let x_condition = if p1.0 > p2.0 {
            Condition::LTE(p1.0)
        } else { // Note: Did not cover "equal to" case
            Condition::GTE(p1.0)
        };

        let y_condition = if p1.1 > p2.1 {
            Condition::LTE(p1.1)
        } else { // Note: Did not cover "equal to" case
            Condition::GTE(p1.1)
        };

        LinearEq { m: m, b: b, x_condition: x_condition, y_condition: y_condition }
    }

    fn from_hailstone_adjusted(hailstone: &Hailstone, velocity_adjustment: &Pos) -> LinearEq {
        let p1 = &hailstone.start_pos;
        let v = &hailstone.velocity;
        let va = velocity_adjustment;
        let p2 = (p1.0 + v.0 - va.0, p1.1 + v.1 - va.1, p1.2 + v.2 - va.2);

        LinearEq::from_two_points(p1, &p2)
    }

    fn from_hailstone(hailstone: &Hailstone) -> LinearEq {
        let p1 = &hailstone.start_pos;
        let v = &hailstone.velocity;
        let p2 = (p1.0 + v.0, p1.1 + v.1, p1.2 + v.2);

        LinearEq::from_two_points(p1, &p2)
    }
}

impl Hailstone {
    fn get_time_to_intersection(&self, intersection: &Pos, velocity_adjustment: &Pos) -> u64 {
        let x_vel = self.velocity.0 - velocity_adjustment.0;
        if x_vel != 0 {
            return ((intersection.0 - self.start_pos.0) / x_vel) as u64;
        }
        panic!("X adjusted velocity is 0");
    }

    fn parse(line: &str) -> Hailstone {
        let re = Regex::new(r"([-]*[0-9]+)").unwrap();
        let values: Vec<i64> = re.captures_iter(line).map(|c| c.extract()).map(|(_, [val])| val.parse::<i64>().unwrap()).collect();
        let start_pos = (values[0], values[1], values[2]);
        let velocity = (values[3], values[4], values[5]);

        Hailstone { start_pos: start_pos, velocity: velocity }
    }
}

impl HailstoneMap {
    fn is_within_area(p: &FPos, start: &Pos, end: &Pos) -> bool {
        if p.0 >= start.0 as f64 && p.0 <= end.0 as f64 &&
           p.1 >= start.1 as f64 && p.1 <= end.1 as f64 {
            return true;
        }

        return false;
    }

    fn find_future_intersections(linear_eqs: &Vec<LinearEq>, i: usize, linear_eq: &LinearEq) -> Vec<FPos>{
        let mut intersections = Vec::new();
        for (j, linear_eq_other) in linear_eqs.iter().enumerate() {
            if j <= i { // Note: Any better way to skip ahead? Slices?
                continue;
            }

            let intersection = linear_eq.future_intersection(linear_eq_other);
            if intersection.is_some() {
                intersections.push(intersection.unwrap());
            }
        }

        return intersections;
    }

    fn get_z_velocity(a: &Hailstone, b: &Hailstone, velocity_adjustment: &Pos, intersection: &Pos) -> i64 {
        let time_a = a.get_time_to_intersection(intersection, velocity_adjustment) as i64;
        let time_b = b.get_time_to_intersection(intersection, velocity_adjustment) as i64;

        if time_a == time_b {
            panic!("Time a == time b");
        }

        return (a.start_pos.2 - b.start_pos.2 + time_a * a.velocity.2 - time_b * b.velocity.2) / (time_a - time_b);
    }

    fn verify_z_velocity(hail: &Vec<Hailstone>, velocity_adjustment: &Pos, intersection: &Pos) -> Option<i64> {
        let mut vel_z = None;
        let mut hail_it = hail.iter();
        let first_hail = hail_it.next().unwrap();
        while let Some(other_hail) = hail_it.next() {
            let tmp_vel_z = Self::get_z_velocity(first_hail, other_hail, velocity_adjustment, intersection);

            if vel_z.is_none() {
                vel_z = Some(tmp_vel_z);
            } else {
                if *vel_z.as_ref().unwrap() != tmp_vel_z {
                    return None;
                }
            }
        }

        return Some(vel_z.unwrap());
    }

    fn calc_rock_start_pos(hail: &Hailstone, velocity_adjustment: &Pos, intersection: &Pos) -> Pos {
        // r= rock, h = some hail
        // P(t, r) = P(0, r) + V(r) * t
        // P(t, h) = P(0, h) + V(h) * t
        // P(0,r) + V(r) * t_rh = P(0, h) + V(h) * t_rh
        // P(0,r) = P(0, h) + t_rh * (V(h) - V(r))
        let time_rh = hail.get_time_to_intersection(intersection, velocity_adjustment) as i64;

        let rx = hail.start_pos.0 + time_rh * (hail.velocity.0 - velocity_adjustment.0);
        let ry = hail.start_pos.1 + time_rh * (hail.velocity.1 - velocity_adjustment.1);
        let rz = hail.start_pos.2 + time_rh * (hail.velocity.2 - velocity_adjustment.2);

        return (rx, ry, rz);
    }

    fn test_velocities(hail: &Vec<Hailstone>, rock_x_vel: i64, rock_y_vel: i64) -> Option<Pos>{        
        // Adjust hail speeds on the fly
        let mut velocity_adjustment = (rock_x_vel, rock_y_vel, 0);
        let mut hail_it = hail.iter();
        let first_linear_eq = LinearEq::from_hailstone_adjusted(hail_it.next().unwrap(), &velocity_adjustment);

        // Intersect first hailstone with the rest
        let mut prev_intersection = None;
        while let Some(other_hail) = hail_it.next() {
            let other_linear_eq = LinearEq::from_hailstone_adjusted(other_hail, &velocity_adjustment);
            let intersection = first_linear_eq.future_intersection(&other_linear_eq);

            if intersection.is_none(){
                // Equations did not intersect, return!
                return None;
            } else if prev_intersection.is_none() {
                prev_intersection = intersection;
            } else {
                // Compare intersections
                if !LinearEq::compare_intersections(prev_intersection.as_ref().unwrap(), intersection.as_ref().unwrap()) {
                    // Intersections are not the same, return!
                    return None;
                }

                prev_intersection = intersection;
            }
        }

        // If we made it here, it means all xy-lines intersected!
        println!("All XY lines intersected for velocity {:?}", velocity_adjustment);
        let pi_unwrapped = prev_intersection.unwrap();
        let intersection = (pi_unwrapped.0 as i64, pi_unwrapped.1 as i64, pi_unwrapped.2 as i64);

        // Verify and calculate z velocity
        let z_vel = Self::verify_z_velocity(hail, &velocity_adjustment, &intersection);
        if z_vel.is_none() {
            println!("Verify z velocity failed");
            return None;
        }

        velocity_adjustment.2 = z_vel.unwrap();

        println!("Velocity {:?}", velocity_adjustment);

        let rock_start_pos = Self::calc_rock_start_pos(hail.first().unwrap(), &velocity_adjustment, &intersection);

        return Some(rock_start_pos)
    }

    fn brute_force_velocities(hail: &Vec<Hailstone>) -> Pos{
        let neg_vec = vec![-1, 1];
        let mut upper_bound = 0;
        loop {
            for x in 0..(upper_bound + 1) {
                // Note: Could probably add condition to avoid duplicate calculations

                for x_neg in neg_vec.iter() {
                    for y_neg in neg_vec.iter() {
                        let rock_x_vel = x * x_neg;
                        let rock_y_vel = (upper_bound - x) * y_neg;

                        let res = Self::test_velocities(hail, rock_x_vel, rock_y_vel);
                        if res.is_some() {
                            return res.unwrap();
                        }
                    }
                }
            }

            upper_bound += 1;
        }
    }

    pub fn find_rock_start_pos(&self) -> Pos {
        return Self::brute_force_velocities(&self.hail);
    }

    pub fn find_xy_intersections_in_area(&self, start: Pos, end: Pos) -> u64 {
        let linear_eqs: Vec<LinearEq> = self.hail.iter().map(|hailstone| LinearEq::from_hailstone(hailstone)).collect();

        let mut intersections_in_area = Vec::new();
        for (i, linear_eq) in linear_eqs.iter().enumerate() {
            let mut intersections = Self::find_future_intersections(&linear_eqs, i, linear_eq);
            intersections.retain(|intersection| Self::is_within_area(&intersection, &start, &end));

            intersections_in_area.append(&mut intersections);
        }

        return intersections_in_area.len() as u64;
    }

    pub fn parse(file: &str) -> HailstoneMap {
        let lines = aoc_helper::read_lines(file);
        let hail = lines.into_iter().map(|line| Hailstone::parse(&line)).collect();

        HailstoneMap { hail: hail }
    }
}