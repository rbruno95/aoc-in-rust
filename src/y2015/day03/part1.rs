use std::collections::HashSet;

pub fn calculate_number_of_houses(instructions: &str) -> usize {
    let mut set = HashSet::new();

    let mut pos = (0, 0);
    set.insert(pos);

    instructions.chars().for_each(|c| {
        match c {
            '>' => pos.1 += 1,
            '<' => pos.1 -= 1,
            '^' => pos.0 += 1,
            'v' => pos.0 -= 1,
            _ => panic!("Wrong Input"),
        };
        if !set.contains(&pos) {
            set.insert(pos);
        }
    });

    set.len()
}

#[test]
fn test_calculate_number_of_houses() {
    let tests = [(">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)];

    for test in tests {
        assert_eq!(calculate_number_of_houses(&test.0), test.1);
    }
}

#[test]
fn test_calculate_number_of_houses_with_input() {
    let instructions = std::fs::read_to_string("src/y2015/day03/input.txt")
        .expect("Should have been able to read the file");

    assert_eq!(calculate_number_of_houses(&instructions), 2565)
}
