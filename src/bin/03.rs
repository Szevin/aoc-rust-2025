advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    input
    .lines()
    .map(|line| line.chars())
    .map(|chars| chars.into_iter().map(|char| char.to_digit(10).unwrap()).collect::<Vec<u32>>())
    .map(|digits| {

        let mut first_digit = 0;
        let mut first_digit_pos = 0;
        let mut second_digit = 0;

        let mut i = 0;
        while i < digits.len() - 1 {
            let current_digit = digits[i];

            if current_digit > first_digit {
                first_digit = current_digit;
                first_digit_pos = i;
            }

            i += 1;
        }

        i = first_digit_pos + 1;
        while i < digits.len() {
            let current_digit = digits[i];

            if current_digit > second_digit {
                second_digit = current_digit;
            }

            i += 1;
        }

        first_digit * 10 + second_digit
    })
    // .inspect(|joltage| {dbg!(joltage);})
    .sum::<u32>()
    .into()
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
