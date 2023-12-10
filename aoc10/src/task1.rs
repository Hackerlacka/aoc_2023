use crate::utils::PipeSketch;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let pipe_sketch_1 = PipeSketch::parse("../input/10_1_example_input_1_hard.txt");
        let main_loop_1 = pipe_sketch_1.get_main_loop();

        let max_distance_1 = (main_loop_1.len() - 1).div_ceil(2);

        assert_eq!(max_distance_1, 4);

        let pipe_sketch_2 = PipeSketch::parse("../input/10_1_example_input_2_hard.txt");
        let main_loop_2 = pipe_sketch_2.get_main_loop();

        let max_distance_2 = (main_loop_2.len() - 1).div_ceil(2);

        assert_eq!(max_distance_2, 8);
    }
}

pub fn run_task() {
    let pipe_sketch = PipeSketch::parse("input/10_1_input.txt");
    let main_loop = pipe_sketch.get_main_loop();

    let max_distance = (main_loop.len() - 1).div_ceil(2);

    println!("Max distance is: {}", max_distance);
}