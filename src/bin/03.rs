use rayon::prelude::*;

#[aoc::main(03)]
fn main(input: &str) -> (u64, u64) {
    solve(input)
}

fn solve(input: &str) -> (u64, u64) {
    // Get largest two-digit number by picking the earliest index of the max digit
    // (from positions 0..len-2) as tens, and the max digit to its right as ones.
    // For example: 811111111111119 -> 8 and 9 => 89; 234234234234278 -> 7 and 8 => 78
    input
        .lines()
        .par_bridge()
        .map(|line| {
            let digits: Vec<u64> = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect();

            // Part 1
            let prefix = &digits[..digits.len() - 1];
            let tens_digit = *prefix.iter().max().unwrap();
            let idx_of_max_tens = prefix.iter().position(|&d| d == tens_digit).unwrap();
            let ones_digit = digits[idx_of_max_tens + 1..].iter().copied().max().unwrap();
            let part_one = tens_digit * 10 + ones_digit;

            // Part 2
            let target_len = 12usize.min(digits.len());
            let mut to_remove = digits.len() - target_len;
            let mut stack: Vec<u64> = Vec::with_capacity(digits.len());
            for &d in &digits {
                while to_remove > 0 && !stack.is_empty() && *stack.last().unwrap() < d {
                    stack.pop();
                    to_remove -= 1;
                }
                stack.push(d);
            }
            if stack.len() > target_len {
                stack.truncate(target_len);
            }
            let part_two = stack.into_iter().fold(0u64, |acc, digit| acc * 10 + digit);
            (part_one, part_two)
        })
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";
    #[test]
    fn test_part_one() {
        let (part_one, _part_two) = solve(EXAMPLE_INPUT);
        assert_eq!(part_one, 357);
    }

    #[test]
    fn test_part_two() {
        let (_, part_two) = solve(EXAMPLE_INPUT);
        assert_eq!(part_two, 3121910778619);
    }
}
