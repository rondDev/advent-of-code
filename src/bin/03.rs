advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let len = lines.clone().count();
    let line_len = lines.clone().next().unwrap().len();

    let mut counts: Vec<usize> = vec![0; line_len];
    lines.for_each(|l| {
        for (i, lc) in l.chars().enumerate() {
            if lc == '1' {
                counts[i] = counts[i] + 2;
            }
        }
    });
    let mut res_str = String::new();
    let mut res2_str = String::new();
    for count in counts.clone() {
        if count >= len {
            res_str = res_str + "1";
            res2_str = res2_str + "0";
        } else {
            res_str = res_str + "0";
            res2_str = res2_str + "1";
        }
    }
    Some(bin_to_dec(&res_str) * bin_to_dec(&res2_str))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines: Vec<&str> = input.lines().to_owned().collect();
    let mut lines_copy = lines.clone();
    let line_len = lines.clone()[0].len();

    for i in 0..line_len {
        let mut ones_count = 0;
        let mut zeroes_count = 0;
        for line in &lines {
            if line.chars().collect::<Vec<char>>()[i] == '1' {
                ones_count += 1;
            } else {
                zeroes_count += 1;
            }
        }
        lines = lines
            .into_iter()
            .filter(|&x| {
                if ones_count != 0 && zeroes_count != 0 {
                    if ones_count > zeroes_count {
                        x.chars().collect::<Vec<char>>()[i] == '1'
                    } else if ones_count == zeroes_count {
                        x.chars().collect::<Vec<char>>()[i] == '1'
                    } else {
                        x.chars().collect::<Vec<char>>()[i] == '0'
                    }
                } else {
                    x == x
                }
            })
            .collect::<Vec<&str>>();

        ones_count = 0;
        zeroes_count = 0;

        for line in &lines_copy {
            if line.chars().collect::<Vec<char>>()[i] == '1' {
                ones_count += 1;
            } else {
                zeroes_count += 1;
            }
        }

        lines_copy = lines_copy
            .into_iter()
            .filter(|&x| {
                if ones_count != 0 && zeroes_count != 0 {
                    if ones_count > zeroes_count {
                        x.chars().collect::<Vec<char>>()[i] == '0'
                    } else if ones_count == zeroes_count {
                        x.chars().collect::<Vec<char>>()[i] == '0'
                    } else {
                        x.chars().collect::<Vec<char>>()[i] == '1'
                    }
                } else {
                    x == x
                }
            })
            .collect::<Vec<&str>>();
    }
    Some(bin_to_dec(lines[0]) * bin_to_dec(lines_copy[0]))
}

fn bin_to_dec(input: &str) -> u32 {
    u32::from_str_radix(input, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(198));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(230));
    }
}
