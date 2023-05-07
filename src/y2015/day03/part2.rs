use std::collections::HashSet;

pub fn calculate_number_of_houses(instructions: &str) -> usize {
    let mut set = HashSet::new();

    let mut santa_pos = (0, 0);
    let mut robo_santa_pos = (0, 0);
    set.insert(santa_pos);

    let mut movement = |c, pos: &mut (i32, i32)| {
        match c {
            '>' => pos.1 += 1,
            '<' => pos.1 -= 1,
            '^' => pos.0 += 1,
            'v' => pos.0 -= 1,
            _ => panic!("Wrong Input"),
        };
        if !set.contains(pos) {
            set.insert(*pos);
        }
    };

    instructions.chars().enumerate().for_each(|(i, c)| {
        if i % 2 == 0 {
            movement(c, &mut santa_pos)
        } else {
            movement(c, &mut robo_santa_pos)
        }
    });

    set.len()
}

#[test]
fn test_calculate_number_of_houses() {
    let tests = [("^v", 3), ("^>v<", 3), ("^v^v^v^v^v", 11)];

    for test in tests {
        assert_eq!(calculate_number_of_houses(&test.0), test.1);
    }
}

#[test]
fn test_calculate_number_of_houses_with_input() {
    let instructions = std::fs::read_to_string("src/y2015/day03/input.txt")
        .expect("Should have been able to read the file");

    assert_eq!(calculate_number_of_houses(&instructions), 2639)
}
