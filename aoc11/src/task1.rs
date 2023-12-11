use crate::utils::SkyImage;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let sky_image = SkyImage::parse("../input/11_1_example_input.txt");
        let shortest_paths = sky_image.find_shortest_paths(2);
        let sum = shortest_paths.iter().map(|p| p.1).sum::<usize>();

        assert_eq!(sum, 374);
    }
}

pub fn run_task() {
    let sky_image = SkyImage::parse("input/11_1_input.txt");
    let shortest_paths = sky_image.find_shortest_paths(2);
    let sum = shortest_paths.iter().map(|p| p.1).sum::<usize>();

    println!("Sum is {}", sum);
}