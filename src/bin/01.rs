advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<u32> = input.lines().map(|l| l.parse::<u32>().unwrap()).collect();
    let mut prev_line = 0;
    let mut increments = 0;
    for line in lines {
        if line > prev_line && prev_line != 0 {
            increments += 1;
        }
        prev_line = line;
    }
    Some(increments)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<u32> = input.lines().map(|l| l.parse::<u32>().unwrap()).collect();
    let mut iter = lines.windows(3);
    let mut prev_window = 0;
    let mut increments = 0;
    while let Some(x) = iter.next() {
        let mut window_total = 0;
        for num in x {
            window_total += num;
        }
        if window_total > prev_window && prev_window != 0 {
            increments += 1;
        }
        prev_window = window_total;
    }
    Some(increments)
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
