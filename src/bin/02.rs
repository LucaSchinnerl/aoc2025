#[aoc::main(02)]
fn main(input: &str) -> (u64, u64) {
    solve(input)
}

use rayon::prelude::*;

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

fn repeating_flags_from_bytes(bytes: &[u8]) -> (bool, bool) {
    let len = bytes.len();
    let mut any_repetition = false;
    let mut within_limit_repetition = false;
    // Fast path for repeats == 2 (part 1 condition)
    if len.is_multiple_of(2) {
        let mid = len / 2;
        if bytes[..mid] == bytes[mid..] {
            return (true, true);
        }
    }
    // Try all possible pattern lengths (divisors of len).
    // We keep scanning even if we find a repetition with repeats > 2,
    // because part one only counts if any divisor yields repeats <= 2.
    // We can stop at len/3 because len/2 already handled by fast path above.
    for pat_len in 1..=len / 3 {
        if !len.is_multiple_of(pat_len) {
            continue;
        }
        let repeats = len / pat_len;
        let pattern = &bytes[..pat_len];
        let mut pos = pat_len;
        let mut ok = true;
        while pos < len {
            let end = pos + pat_len;
            if &bytes[pos..end] != pattern {
                ok = false;
                break;
            }
            pos = end;
        }
        if ok {
            any_repetition = true;
            if repeats <= 2 {
                within_limit_repetition = true;
                break;
            }
        }
    }
    (any_repetition, within_limit_repetition)
}

fn repeating_flags(id: u64) -> (bool, bool) {
    // No heap allocation for formatting the number
    let mut buf = itoa::Buffer::new();
    let s = buf.format(id);
    repeating_flags_from_bytes(s.as_bytes())
}

fn solve(input: &str) -> (u64, u64) {
    let ranges: Vec<Range> = input.split(',').map(parse_range).collect();
    ranges
        .par_iter()
        .map(|range| {
            let mut p1 = 0u64;
            let mut p2 = 0u64;
            for id in range.start..=range.end {
                let (any, within_limit) = repeating_flags(id);
                if any {
                    p2 += id;
                    if within_limit {
                        p1 += id;
                    }
                }
            }
            (p1, p2)
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
