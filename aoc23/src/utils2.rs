use std::collections::{HashMap, VecDeque};

use crate::utils::{TrailMap, Tile, TileType};

type Pos = (usize, usize);

struct Node {
    pos: Pos,
    vertices: HashMap<Pos, usize>, // Note: Only keep the most expensive path between nodes!
}

pub struct TrailGraph {
    nodes: HashMap<Pos, Node>,
    start_pos: Pos,
    end_pos: Pos
}

impl Node {
    fn lookup_node<'a>(pos: &Pos, simple_nodes: &'a mut VecDeque<Node>, real_nodes: &'a mut HashMap<Pos, Node>) -> &'a mut Node {
        if real_nodes.contains_key(pos) {
            return real_nodes.get_mut(pos).unwrap();
        }

        for node in simple_nodes.iter_mut() {
            if node.pos == *pos {
                return node;
            }
        }

        panic!("Could not find node!");
    }

    fn merge_into_others(&mut self, simple_nodes: &mut VecDeque<Node>, real_nodes: &mut HashMap<Pos, Node>) {
        if self.vertices.len() != 2 {
            panic!("Vertices not of expected length!"); // TODO: Might need to fix
        }

        let mut it = self.vertices.iter();

        let vertice_1 = it.next().unwrap();
        let vertice_2 = it.next().unwrap();

        // Remove old connections
        let cost_1: usize;
        let node_1_pos: Pos;
        {
            let node_1 = Self::lookup_node(vertice_1.0, simple_nodes, real_nodes);
            cost_1 = node_1.vertices.remove(&self.pos).unwrap();
            node_1_pos = node_1.pos.clone();
        }

        let cost_2: usize;
        let node_2_pos: Pos;
        {
            let node_2 = Self::lookup_node(vertice_2.0, simple_nodes, real_nodes);
            cost_2 = node_2.vertices.remove(&self.pos).unwrap();
            node_2_pos = node_2.pos.clone();
        }

        // Add new connections
        let cost = cost_1 + cost_2;
        {
            let node_1 = Self::lookup_node(vertice_1.0, simple_nodes, real_nodes);
            let current_cost = node_1.vertices.get(&node_2_pos);
            if current_cost.is_some() {
                // Keep only the most expensive path between nodes
                if *current_cost.unwrap() < cost {
                    node_1.vertices.insert(node_2_pos, cost);    
                }
            } else {
                node_1.vertices.insert(node_2_pos, cost);
            }
        }
        {
            let node_2 = Self::lookup_node(vertice_2.0, simple_nodes, real_nodes);
            let current_cost = node_2.vertices.get(&node_1_pos);
            if current_cost.is_some() {
                // Keep only the most expensive path between nodes
                if *current_cost.unwrap() < cost {
                    node_2.vertices.insert(node_1_pos, cost);    
                }
            } else {
                node_2.vertices.insert(node_1_pos, cost);
            }
        }
    }

    fn connect(&mut self, other: &mut Self) {
        self.vertices.insert(other.pos.clone(), 1);
        other.vertices.insert(self.pos.clone(), 1);
    }

    fn new_empty(pos: Pos) -> Node {
        Node { pos: pos , vertices: HashMap::new() }
    }
}

impl TrailGraph {
    fn compress_graph(nodes: &mut HashMap<Pos, Node>, height: usize) {
        let mut simple_nodes = VecDeque::new();

        let mut real_nodes = HashMap::new();
        for (pos, node) in nodes.drain() {
            // Add start and end nodes to "real nodes"
            if pos.0 == 0 || pos.0 + 1 == height {
                real_nodes.insert(pos, node);
                continue;
            }

            if node.vertices.len() <= 2 {
                simple_nodes.push_back(node);
            } else {
                real_nodes.insert(pos, node);
            }
        }

        while let Some(mut simple_node) = simple_nodes.pop_front() {
            simple_node.merge_into_others(&mut simple_nodes, &mut real_nodes);
        }

        // Return real nodes
        nodes.extend(real_nodes.drain());
    }

    fn try_connect(node: &mut Node, pos_other: Pos, nodes: &mut HashMap<Pos, Node>) {
        let other_node_wrapped = nodes.get_mut(&pos_other);
        if other_node_wrapped.is_some() {
            let other_node = other_node_wrapped.unwrap();
            node.connect(other_node);
        }
    }

    fn create_node(map: &Vec<Vec<Tile>>, pos: Pos, nodes: &mut HashMap<Pos, Node>) {
        let mut node = Node::new_empty(pos.clone());

        // Check tile type of node
        let tile = &map[pos.0][pos.1];
        match tile.tile_type {
            TileType::Forest => return,
            _ => ()
        }

        if pos.0 > 0 {
            Self::try_connect(&mut node, (pos.0 - 1, pos.1), nodes);
        }
        if pos.1 > 0 {
            Self::try_connect(&mut node, (pos.0, pos.1 - 1), nodes);
        }

        nodes.insert(pos, node);
    }

    fn visit_node_rec(cost: usize, node: &Node, nodes: &HashMap<Pos, Node>, visited_nodes: &mut HashMap<Pos, bool>, end_pos: &Pos, highest_cost: &mut usize) {
        *visited_nodes.get_mut(&node.pos).unwrap() = true;

        // If at end node, compare cost and return
        if node.pos == *end_pos {
            if cost > *highest_cost {
                *highest_cost = cost;
            }
            *visited_nodes.get_mut(&node.pos).unwrap() = false;
            return;
        }

        // Visit other nodes
        for (other_node_pos, cost_to_other) in node.vertices.iter() {
            let other_node = nodes.get(other_node_pos).unwrap();
            if visited_nodes[&other_node.pos] {
                continue;
            }

            Self::visit_node_rec(cost + cost_to_other, other_node, nodes, visited_nodes, end_pos, highest_cost);
        }

        // Return back
        *visited_nodes.get_mut(&node.pos).unwrap() = false;
    }



    pub fn find_longest_path(&self) -> u64 {
        let mut highest_cost = 0;

        let mut visited_nodes = HashMap::new();
        for (pos, _) in self.nodes.iter() {
            visited_nodes.insert(pos.clone(), false);
        }

        let start_node = self.nodes.get(&self.start_pos).unwrap();
        Self::visit_node_rec(0, start_node, &self.nodes, &mut visited_nodes, &self.end_pos, &mut highest_cost);

        return highest_cost as u64;
    }

    pub fn parse(file: &str) -> TrailGraph {
        let trail_map = TrailMap::parse(file);
        let mut nodes = HashMap::new();

        for (y, line_vec) in trail_map.map.iter().enumerate() {
            for (x, tile) in line_vec.iter().enumerate() {
                match tile.tile_type {
                    TileType::Forest => continue,
                    _ => {
                        let pos = (y, x);
                        Self::create_node(&trail_map.map, pos, &mut nodes);
                    }
                }
            }
        }

        Self::compress_graph(&mut nodes, trail_map.map.len());

        println!("Nodes in graph: {}", nodes.len());

        let (start_pos, end_pos) = TrailMap::determine_start_and_end_tile_pos(&trail_map.map);

        TrailGraph { nodes: nodes, start_pos: start_pos, end_pos: end_pos }
    }
}