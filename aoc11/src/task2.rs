use crate::utils::SkyImage;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let sky_image = SkyImage::parse("../input/11_1_example_input.txt");
        let shortest_paths_1 = sky_image.find_shortest_paths(10);
        let sum_1 = shortest_paths_1.iter().map(|p| p.1).sum::<usize>();

        assert_eq!(sum_1, 1030);

        let shortest_paths_2 = sky_image.find_shortest_paths(100);
        let sum_2 = shortest_paths_2.iter().map(|p| p.1).sum::<usize>();

        assert_eq!(sum_2, 8410);
    }
}

pub fn run_task() {
    let sky_image = SkyImage::parse("input/11_1_input.txt");
    let shortest_paths = sky_image.find_shortest_paths(1000000);
    let sum = shortest_paths.iter().map(|p| p.1).sum::<usize>();

    println!("Sum is {}", sum);
}