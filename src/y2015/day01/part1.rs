pub fn find_floor(instructions: &str) -> i32 {
    instructions
        .chars()
        .map(|char| match char {
            '(' => 1,
            ')' => -1,
            _ => panic!("Wrong Input"),
        })
        .sum()
}

#[test]
fn sample_test_find_floor() {
    let tests = [
        ("(())", 0),
        ("()()", 0),
        ("(((", 3),
        ("(()(()(", 3),
        ("))(((((", 3),
        ("())", -1),
        ("))(", -1),
        (")))", -3),
        (")())())", -3),
    ];

    for test in tests {
        assert_eq!(find_floor(test.0), test.1);
    }
}

#[test]
fn test_find_floor_with_input() {
    let instructions = std::fs::read_to_string("src/y2015/day01/input.txt")
        .expect("Should have been able to read the file");

    assert_eq!(find_floor(&instructions), 280)
}
