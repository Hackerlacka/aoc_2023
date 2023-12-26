use std::collections::HashMap;

use rand::Rng;
use rand::rngs::ThreadRng;

#[derive(Debug, Clone)]
struct Node {
    name: String,
    connections: HashMap<String, usize>,
    merged_nodes: usize
}

pub struct WireMap {
    nodes: HashMap<String, Node>
}

impl Node {
    fn merge_nodes(mut a: Node, mut b: Node, nodes: &mut HashMap<String, Node>) {
        let new_name = format!("{},{}", a.name, b.name);

        // Remove links between a and b
        a.connections.remove(&b.name);
        b.connections.remove(&a.name);

        let mut new_node = Node::new_empty(&new_name);
        new_node.merged_nodes = 1 + a.merged_nodes + b.merged_nodes;
        
        // Update links to new node
        for (_, node) in nodes.iter_mut() {
            let conns_with_a = *node.connections.get(&a.name).unwrap_or(&0);
            let conns_with_b = *node.connections.get(&b.name).unwrap_or(&0);

            if conns_with_a > 0 {
                node.connections.remove(&a.name);
            }
            if conns_with_b > 0 {
                node.connections.remove(&b.name);
            }

            let conns_sum = conns_with_a + conns_with_b;
            if conns_sum > 0 {
                node.connections.insert(new_name.clone(), conns_sum);
                new_node.connections.insert(node.name.clone(), conns_sum);
            }
        }

        nodes.insert(new_name, new_node);
    }

    fn new_empty(name: &str) -> Node {
        Node { name: name.to_owned(), connections: HashMap::new(), merged_nodes: 0 }
    }

    fn append_or_create_connection(node: &mut Node, connection_name: &str) {
        let val = node.connections.get_mut(connection_name);
        if val.is_some() {
            *val.unwrap() += 1;
        } else {
            node.connections.insert(connection_name.to_owned(), 1);
        }
    }

    fn parse(line: &str, nodes: &mut HashMap<String, Node>) -> usize {
        let mut split_line = line.split(": ");
        let name = split_line.next().unwrap();

        // Check if node already exists
        let mut node_wrapped = nodes.get_mut(name);
        let node: &mut Node;
        if node_wrapped.is_none() {
            nodes.insert(name.to_owned(), Node::new_empty(name));
            node_wrapped = nodes.get_mut(name);
        }
        node = node_wrapped.unwrap();

        // Add connected nodes
        let connections: Vec<&str> = split_line.next().unwrap().split(" ").collect();
        for connection in connections.iter() {
            Self::append_or_create_connection(node, *connection);
        }

        // Create connected nodes if non-existing
        for connection in connections.iter() {
            // Get connected node (or create it)
            let mut connected_node_wrapped = nodes.get_mut(*connection);
            if connected_node_wrapped.is_none() {
                nodes.insert((*connection).to_owned(), Node::new_empty(*connection));
                connected_node_wrapped = nodes.get_mut(*connection);
            }
            let connected_node = connected_node_wrapped.unwrap();

            // Add the connection to it as well
            Self::append_or_create_connection(connected_node, name);
        }

        return connections.len();
    }
}

impl WireMap {
    pub fn print(&self) {
        println!("Nodes: {}", self.nodes.len());
        for (_, node) in self.nodes.iter() {
            println!("{:?}", node);
        }
    }

    fn pop_random_node_from_other(node: &Node, nodes: &mut HashMap<String, Node>, rng: &mut ThreadRng) -> Node {
        let index = rng.gen_range(0..node.connections.len());
        let random_other_name = node.connections.keys().skip(index).next().unwrap().to_owned();
        let random_other_node = nodes.remove(&random_other_name).unwrap();

        return random_other_node;
    }

    fn pop_random_node(nodes: &mut HashMap<String, Node>, rng: &mut ThreadRng) -> Node {
        let index = rng.gen_range(0..nodes.len());
        let rand_key = nodes.keys().skip(index).next().unwrap().to_owned();

        return nodes.remove(&rand_key).unwrap();
    }

    fn cut_until_two_nodes(mut nodes: HashMap<String, Node>, n: usize) -> Option<(usize, usize)> {
        let mut rng = rand::thread_rng();

        while nodes.len() > 2 {
            let node_1 = Self::pop_random_node(&mut nodes, &mut rng);
            let node_2 = Self::pop_random_node_from_other(&node_1, &mut nodes, &mut rng);

            // There could be more than one connection between these nodes
            Node::merge_nodes(node_1, node_2, &mut nodes);
        }

        let mut it = nodes.into_values();
        let node_1 = it.next().unwrap();
        let connections_remaining = *node_1.connections.iter().next().unwrap().1;
        if connections_remaining != n {
            //println!("Connections remaining: {}", connections_remaining);
            return None;
        }

        let group_1_len = node_1.merged_nodes + 1;
        let group_2_len = it.next().unwrap().merged_nodes + 1;

        let res = Some((group_1_len, group_2_len));

        println!("Res is: {:?}", res);

        return res;
    }

    fn find_n_minimum_cut(&self, n: usize) -> (usize, usize) {
        let mut result = None;
        while result.is_none() {
            let nodes = self.nodes.clone();
            result = Self::cut_until_two_nodes(nodes, n);
        }

        return result.unwrap();
    }

    pub fn divide_into_two_groups(&self) -> usize {
        let (group1_len, group2_len) = Self::find_n_minimum_cut(&self, 3);

        return group1_len * group2_len;
    }

    pub fn parse(file: &str) -> WireMap {
        let lines = aoc_helper::read_lines(file);
        let mut nodes = HashMap::new();

        let mut sum = 0;
        for line in lines.into_iter() {
            sum += Node::parse(&line, &mut nodes);
        }

        println!("Connections: {}", sum);

        WireMap { nodes: nodes }
    }
}