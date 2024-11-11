advent_of_code::solution!(2);

enum Direction {
    Forward,
    Down,
    Up,
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<(Direction, u32)> = input
        .lines()
        .map(|l| {
            let split = l.split(" ").collect::<Vec<&str>>();
            let split_direction = split[0];
            let split_val: u32 = split[1].parse().unwrap();
            match split_direction {
                "forward" => (Direction::Forward, split_val),
                "down" => (Direction::Down, split_val),
                "up" => (Direction::Up, split_val),
                _ => (Direction::Forward, 0),
            }
        })
        .collect();

    let mut depth = 0;
    let mut horizontal_pos = 0;

    for line in lines {
        match line.0 {
            Direction::Forward => horizontal_pos += line.1,
            Direction::Down => depth += line.1,
            Direction::Up => depth -= line.1,
        }
    }

    Some(depth * horizontal_pos)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<(Direction, u32)> = input
        .lines()
        .map(|l| {
            let split = l.split(" ").collect::<Vec<&str>>();
            let split_direction = split[0];
            let split_val: u32 = split[1].parse().unwrap();
            match split_direction {
                "forward" => (Direction::Forward, split_val),
                "down" => (Direction::Down, split_val),
                "up" => (Direction::Up, split_val),
                _ => (Direction::Forward, 0),
            }
        })
        .collect();

    let mut depth = 0;
    let mut aim = 0;
    let mut horizontal_pos = 0;

    for line in lines {
        match line.0 {
            Direction::Forward => {
                depth += line.1 * aim;
                horizontal_pos += line.1
            }
            Direction::Down => aim += line.1,
            Direction::Up => aim -= line.1,
        }
    }

    Some(depth * horizontal_pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
