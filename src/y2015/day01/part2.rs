pub fn find_basement(instructions: &str) -> usize {
    let mut floor = 0;
    for (index, instruction) in instructions.chars().enumerate() {
        match instruction {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Wrong Input"),
        }

        if floor == -1 {
            return index + 1;
        }
    }

    panic!("Santa never reached the basement")
}

#[test]
fn sample_test_find_basement() {
    let tests = [(")", 1), ("()())", 5)];

    for test in tests {
        assert_eq!(find_basement(test.0), test.1);
    }
}

#[test]
fn test_find_basement_with_input() {
    let instructions = std::fs::read_to_string("src/y2015/day01/input.txt")
        .expect("Should have been able to read the file");

    assert_eq!(find_basement(&instructions), 1797)
}
