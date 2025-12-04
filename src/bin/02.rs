#[aoc::main(02)]
fn main(input: &str) -> (u64, u64) {
    solve(input)
}

use rayon::prelude::*;
use std::collections::HashSet;

struct Range {
    start: u64,
    end: u64,
}

fn parse_range(range: &str) -> Range {
    // input in the format of "328412-412772"
    let (start, end) = range.split_once('-').unwrap();
    Range {
        start: start.parse().unwrap(),
        end: end.parse().unwrap(),
    }
}

fn get_digits(n: u64) -> u32 {
    if n == 0 { 1 } else { n.ilog10() + 1 }
}

fn solve(input: &str) -> (u64, u64) {
    let ranges: Vec<Range> = input.split(',').map(parse_range).collect();

    ranges
        .par_iter()
        .map(|range| {
            let mut p1_sum = 0u64;
            let mut p2_sum = 0u64;
            let start_len = get_digits(range.start);
            let end_len = get_digits(range.end);

            let mut found_in_range = HashSet::new();

            for l_total in start_len..=end_len {
                // Iterate over possible pattern lengths.
                // A valid pattern length l_pat must divide l_total
                // and result in at least 2 repetitions (l_total / l_pat >= 2).
                // This implies l_pat <= l_total / 2.
                for l_pat in 1..=(l_total / 2) {
                    if l_total % l_pat != 0 {
                        continue;
                    }

                    // Calculate multiplier: (10^l_total - 1) / (10^l_pat - 1)
                    let num = 10u64.pow(l_total) - 1;
                    let den = 10u64.pow(l_pat) - 1;
                    let multiplier = num / den;

                    // Determine the range of pattern values P such that
                    // range.start <= P * multiplier <= range.end
                    // P >= ceil(range.start / multiplier)
                    // P <= floor(range.end / multiplier)
                    let p_min_calc = range.start.div_ceil(multiplier);
                    let p_max_calc = range.end / multiplier;

                    // P must be a valid l_pat-digit number (no leading zeros).
                    // 10^(l_pat-1) <= P <= 10^l_pat - 1
                    let p_min_digits = 10u64.pow(l_pat - 1);
                    let p_max_digits = 10u64.pow(l_pat) - 1;

                    let p_start = p_min_calc.max(p_min_digits);
                    let p_end = p_max_calc.min(p_max_digits);

                    if p_start > p_end {
                        continue;
                    }

                    for p in p_start..=p_end {
                        let v = p * multiplier;

                        // We might find the same number multiple times with different pattern lengths.
                        // e.g., 1111 from 1 (k=4) and 11 (k=2).
                        // We only count it once per range query.
                        if found_in_range.insert(v) {
                            p2_sum += v;

                            // Part 1 condition: "made only of some sequence of digits repeated twice"
                            // This means the number can be split into two identical halves.
                            // This requires total length to be even.
                            if l_total % 2 == 0 {
                                let half_pow = 10u64.pow(l_total / 2);
                                if v / half_pow == v % half_pow {
                                    p1_sum += v;
                                }
                            }
                        }
                    }
                }
            }
            (p1_sum, p2_sum)
        })
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_one() {
        assert_eq!(solve(EXAMPLE_INPUT).0, 1227775554);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve(EXAMPLE_INPUT).1, 4174379265);
    }
}
