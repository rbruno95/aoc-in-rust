pub fn calculate_ribbon_needed(dimensions: &str) -> u64 {
    let mut dimensions: [u64; 3] = dimensions
        .split('x')
        .map(|dimension| {
            dimension
                .parse::<u64>()
                .expect("Each dimension should be able to parse to u64")
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("The dimensions should have the pattern NxNxN where N is a number");

    dimensions.sort();

    dimensions.iter().product::<u64>() + 2 * dimensions.iter().take(2).sum::<u64>()
}

#[test]
fn test_calculate_ribbon_needed() {
    let tests = [("2x3x4", 34), ("1x1x10", 14)];

    for test in tests {
        assert_eq!(calculate_ribbon_needed(&test.0), test.1);
    }
}

pub fn calculate_all_ribbon_needed(all_dimensions: &str) -> u64 {
    all_dimensions
        .split('\n')
        .map(|dimensions| calculate_ribbon_needed(&dimensions))
        .sum()
}

#[test]
fn test_calculate_all_ribbon_needed() {
    let all_dimensions = std::fs::read_to_string("src/y2015/day02/input.txt")
        .expect("Should have been able to read the file");

    assert_eq!(calculate_all_ribbon_needed(&all_dimensions), 3783758)
}
