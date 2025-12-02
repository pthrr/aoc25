#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn day1_part2() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("data/input/day1.txt");
        let content = fs::read_to_string(path).unwrap();
        let add = |a, b| (a + b) % 100;
        let sub = |a, b| ((a - b) % 100 + 100) % 100;
        let mut state = 50;
        let mut result = 0;

        for line in content.lines() {
            let op = line.chars().next().unwrap();
            let num: i32 = line[1..].trim().parse().unwrap();
            let passes = match op {
                'R' => (state + num) / 100,
                'L' => match (state, num >= state) {
                    (0, _) => num / 100,
                    (_, true) => 1 + (num - state) / 100,
                    (_, false) => 0,
                },
                _ => 0,
            };
            state = match op {
                'R' => add(state, num),
                'L' => sub(state, num),
                _ => state,
            };
            result += passes;
        }

        println!("Result is {}", result);
        assert_eq!(result, 6616);
    }

    #[test]
    fn day1_part1() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("data/input/day1.txt");
        let content = fs::read_to_string(path).unwrap();
        let add = |a, b| (a + b) % 100;
        let sub = |a, b| ((a - b) % 100 + 100) % 100;
        let mut state = 50;
        let mut result = 0;

        for line in content.lines() {
            let op = line.chars().next().unwrap();
            let num: i32 = line[1..].trim().parse().unwrap();
            state = match op {
                'R' => add(state, num),
                'L' => sub(state, num),
                _ => state,
            };
            result = match state {
                0 => result + 1,
                _ => result,
            }
        }

        println!("Result is {}", result);
        assert_eq!(result, 1092);
    }
}
