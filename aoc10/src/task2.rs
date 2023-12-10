use crate::utils::{PipeSketch, Tile};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        // ../input/10_2_example_input_2.txt
        // ../input/10_1_input.txt
        let mut pipe_sketch_1 = PipeSketch::parse("../input/10_2_example_input_4.txt");
        let main_loop_1 = pipe_sketch_1.get_main_loop();

        //PipeSketch::print_overlay(&pipe_sketch_1.map, &main_loop_1);

        println!("");

        let main_loop_1_exp = Tile::expand_tiles(&main_loop_1);
        pipe_sketch_1.create_expanded_map();
        //PipeSketch::print_map(&pipe_sketch_1.expanded_map);

        println!("");

        PipeSketch::add_overlay_to_exp_map(&mut pipe_sketch_1, &main_loop_1_exp);
        pipe_sketch_1.fill_reachable_tiles_ext_map();

        PipeSketch::print_map(&pipe_sketch_1.expanded_map);

        pipe_sketch_1.shrink_expanded_map();

        println!("");
        PipeSketch::print_map(&pipe_sketch_1.map);

        let empty_spaces = pipe_sketch_1.count_empty_spaces();

        println!("Empty spaces: {}", empty_spaces);

    }
}

pub fn run() {
    let mut pipe_sketch_1 = PipeSketch::parse("input/10_1_input.txt");
    let main_loop_1 = pipe_sketch_1.get_main_loop();

    let main_loop_1_exp = Tile::expand_tiles(&main_loop_1);
    pipe_sketch_1.create_expanded_map();

    PipeSketch::add_overlay_to_exp_map(&mut pipe_sketch_1, &main_loop_1_exp);
    pipe_sketch_1.fill_reachable_tiles_ext_map();

    PipeSketch::print_map(&pipe_sketch_1.expanded_map);
    
    pipe_sketch_1.shrink_expanded_map();

    println!("");
    PipeSketch::print_map(&pipe_sketch_1.map);

    let empty_spaces = pipe_sketch_1.count_empty_spaces();

    println!("Empty spaces: {}", empty_spaces);
}

pub fn run_task() {
    use std::thread;
    const STACK_SIZE: usize = 16 * 1024 * 1024;

    // Spawn thread with explicit stack size
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();


}