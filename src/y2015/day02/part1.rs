pub fn calculate_wrapping_paper_needed(dimensions: &str) -> u64 {
    let [l, w, h]: [u64; 3] = dimensions
        .split('x')
        .map(|dimension| {
            dimension
                .parse::<u64>()
                .expect("Each dimension should be able to parse to u64")
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("The dimensions should have the pattern NxNxN where N is a number");

    let sides = [l * w, l * h, w * h];

    sides.iter().sum::<u64>() * 2
        + sides
            .iter()
            .min()
            .expect("The sides array shouldn't be empty")
}

#[test]
fn test_calculate_wrapping_paper_needed() {
    let tests = [("2x3x4", 58), ("1x1x10", 43)];

    for test in tests {
        assert_eq!(calculate_wrapping_paper_needed(&test.0), test.1);
    }
}

pub fn calculate_all_wrapping_paper_needed(all_dimensions: &str) -> u64 {
    all_dimensions
        .split('\n')
        .map(|dimensions| calculate_wrapping_paper_needed(&dimensions))
        .sum()
}

#[test]
fn test_calculate_all_wrapping_paper_needed() {
    let all_dimensions = std::fs::read_to_string("src/y2015/day02/input.txt")
        .expect("Should have been able to read the file");

    assert_eq!(
        calculate_all_wrapping_paper_needed(&all_dimensions),
        1588178
    )
}
