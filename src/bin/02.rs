#[aoc::main(02)]
fn main(input: &str) -> (u64, u64) {
    let (part_one, part_two) = rayon::join(|| part_one(input), || part_two(input));

    (part_one, part_two)
}

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

fn create_range_iterator(input: &str) -> impl Iterator<Item = u64> {
    input
        .split(',')
        .map(parse_range)
        .flat_map(|range| range.start..=range.end)
}

fn is_valid_id(id: u64, max_repeats: Option<usize>) -> bool {
    let digits = id.to_string();
    let len = digits.len();

    // Try all possible pattern lengths
    for pat_len in 1..=len / 2 {
        // Pattern length must divide the total length
        if !len.is_multiple_of(pat_len) {
            continue;
        }

        let repeats = len / pat_len;
        // Must repeat at least twice to be considered a repeating pattern
        if repeats < 2 {
            continue;
        }

        // If there is a max_repeats, enforce it
        if let Some(limit) = max_repeats
            && repeats > limit
        {
            continue;
        }

        let pattern = &digits[..pat_len];
        let mut ok = true;
        let mut pos = pat_len;

        // Check if the entire string is that pattern repeated
        while pos < len {
            let end = pos + pat_len;
            if &digits[pos..end] != pattern {
                ok = false;
                break;
            }
            pos = end;
        }

        // Found a forbidden repeating pattern
        if ok {
            return false;
        }
    }

    // No forbidden repeating pattern found
    true
}

fn part_one(input: &str) -> u64 {
    create_range_iterator(input)
        .filter(|&id| !is_valid_id(id, Some(2)))
        .sum()
}

fn part_two(input: &str) -> u64 {
    create_range_iterator(input)
        .filter(|&id| !is_valid_id(id, None))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_INPUT), 1227775554);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_INPUT), 4174379265);
    }
}
