advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    input
    .split(",")
    .map(|range| {
        let (r1, r2) = range.split_once("-").unwrap();
        (r1.parse::<u64>().unwrap(), r2.parse::<u64>().unwrap())
    })
    .map(|(start, end)| {

        let mut found_numbers = Vec::new();
        let mut curr = start;

        while curr <= end {

            let digits = curr.checked_ilog10().unwrap_or(0) + 1;
            if digits % 2 == 0 {
                let str_curr = curr.to_string();
                let (first_half, second_half) = str_curr.split_at(str_curr.len() / 2);

                if first_half == second_half {
                    found_numbers.push(curr);
                }
            }

            curr += 1;
        };

        found_numbers
    })
    .flatten()
    // .inspect(|item| {dbg!(item);})
    .sum::<u64>().into()
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
