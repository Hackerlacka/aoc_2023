use crate::utils2::TrailGraph;

#[cfg(test)]
mod tests {

    use crate::utils2::TrailGraph;

    use super::*;

    #[test]
    fn test_example_input() {
        let mut trail_graph = TrailGraph::parse("../input/23_1_example_input.txt");
        let longest_path = trail_graph.find_longest_path();
        
        assert_eq!(longest_path, 154);
    }
}

pub fn run_task() {
    let mut trail_graph = TrailGraph::parse("input/23_1_input.txt");
    let longest_path = trail_graph.find_longest_path();

    println!("Longest path is: {}", longest_path);
}