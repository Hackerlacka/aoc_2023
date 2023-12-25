use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Node {
    name: String,
    connections: HashSet<String>
}

pub struct WireMap {
    nodes: HashMap<String, Node>
}

impl Node {
    fn new_empty(name: &str) -> Node {
        Node { name: name.to_owned(), connections: HashSet::new() }
    }

    fn parse(line: &str, nodes: &mut HashMap<String, Node>) {
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
            node.connections.insert((*connection).to_owned());
        }

        // Create connected nodes if non-existing
        for connection in connections.iter() {
            // Get connected node (or create it)
            let mut connected_node_wrapped = nodes.get_mut(*connection);
            if connected_node_wrapped.is_none() {
                nodes.insert((*connection).to_owned(), Node::new_empty(*connection));
                connected_node_wrapped = nodes.get_mut(*connection);
            }

            // Add the connection to it as well
            connected_node_wrapped.unwrap().connections.insert(name.to_owned());
        }
    }
}

impl WireMap {
    pub fn print(&self) {
        println!("Nodes: {}", self.nodes.len());
        for (_, node) in self.nodes.iter() {
            println!("{:?}", node);
        }
    }

    pub fn parse(file: &str) -> WireMap {
        let lines = aoc_helper::read_lines(file);
        let mut nodes = HashMap::new();

        for line in lines.into_iter() {
            Node::parse(&line, &mut nodes);
        }

        WireMap { nodes: nodes }
    }
}