advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut prev_line = 0;
    let mut increments = 0;
    for line in lines {
        let line_value = match line.parse() {
            Ok(v) => v,
            Err(_) => 0,
        };
        if line_value > prev_line && prev_line != 0 {
            increments += 1;
        }
        prev_line = line_value;
    }
    Some(increments)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
