use regex::Regex;

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
    fn intersection(&self, other: &Self) -> Option<FPos> {
        let x_factor = self.m - other.m;

        if x_factor == 0.0 { // TODO: dangerous comparison!
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
        } else { // TODO: Did not cover "equal to" case
            Condition::GTE(p1.0)
        };

        let y_condition = if p1.1 > p2.1 {
            Condition::LTE(p1.1)
        } else { // TODO: Did not cover "equal to" case
            Condition::GTE(p1.1)
        };

        LinearEq { m: m, b: b, x_condition: x_condition, y_condition: y_condition }
    }

    fn from_hailstone(hailstone: &Hailstone ) -> LinearEq {
        let p1 = &hailstone.start_pos;
        let v = &hailstone.velocity;
        let p2 = (p1.0 + v.0, p1.1 + v.1, p1.2 + v.2);

        LinearEq::from_two_points(p1, &p2)
    }
}

impl Hailstone {
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
        // TODO: Need to be careful with float equal comparison?
        if p.0 >= start.0 as f64 && p.0 <= end.0 as f64 &&
           p.1 >= start.1 as f64 && p.1 <= end.1 as f64 {

            //println!("{:?}", p);
            return true;
        }

        return false;
    }

    fn find_future_intersections(linear_eqs: &Vec<LinearEq>, i: usize, linear_eq: &LinearEq) -> Vec<FPos>{
        let mut intersections = Vec::new();
        for (j, linear_eq_other) in linear_eqs.iter().enumerate() {
            if j <= i { // TODO: Any better way to skip ahead? Slices?
                continue;
            }

            let intersection = linear_eq.future_intersection(linear_eq_other);
            if intersection.is_some() {
                intersections.push(intersection.unwrap());
            }
        }

        return intersections;
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