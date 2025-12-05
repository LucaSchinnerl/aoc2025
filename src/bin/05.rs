#[aoc::main(05)]
fn main(input: &str) -> (u64, u64) {
    let (part_one, part_two) = rayon::join(|| part_one(input), || part_two(input));

    (part_one, part_two)
}

#[derive(Clone, Copy, Debug)]
struct Range {
    start: u64,
    end: u64,
}

fn parse_input(input: &str) -> (Vec<Range>, Vec<u64>) {
    let (ranges, numbers) = input.split_once("\n\n").unwrap();

    // Collect all ranges
    let parsed_ranges = ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            Range {
                start: start.parse().unwrap(),
                end: end.parse().unwrap(),
            }
        })
        .collect();

    let parsed_numbers = numbers.lines().map(|line| line.parse().unwrap()).collect();

    (parsed_ranges, parsed_numbers)
}

fn part_one(input: &str) -> u64 {
    let (ranges, numbers) = parse_input(input);

    let mut res = 0;
    for number in numbers {
        for range in &ranges {
            if number >= range.start && number <= range.end {
                res += 1;
                break;
            }
        }
    }

    res
}

fn part_two(input: &str) -> u64 {
    // how many fresh ids are there?
    let (mut ranges, _numbers) = parse_input(input);
    ranges.sort_by_key(|r| r.start);

    let mut merged_ranges: Vec<Range> = Vec::new();

    for range in ranges {
        if let Some(last) = merged_ranges.last_mut() {
            // Check for overlap or adjacency.
            // Since we are counting the number of integers covered,
            // if range.start <= last.end + 1, they form a contiguous block.
            if range.start <= last.end.saturating_add(1) {
                last.end = last.end.max(range.end);
            } else {
                merged_ranges.push(range);
            }
        } else {
            merged_ranges.push(range);
        }
    }

    merged_ranges
        .into_iter()
        .map(|range| range.end - range.start + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 3);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 14);
    }
}
