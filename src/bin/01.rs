#[aoc::main(01)]
fn main(input: &str) -> (i32, i32) {
    let (part_one, part_two) = rayon::join(|| part_one(input), || part_two(input));

    (part_one, part_two)
}

enum Direction {
    Left,
    Right,
}

struct Move {
    direction: Direction,
    steps: i32,
}

fn parse_move(line: &str) -> Move {
    let (direction, steps) = line.split_at(1);
    let direction = match direction {
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("Invalid direction: {}", direction),
    };
    let steps = steps.parse::<i32>().unwrap();
    Move { direction, steps }
}

fn apply_move(position: i32, m: &Move) -> i32 {
    match m.direction {
        Direction::Left => position - m.steps,
        Direction::Right => position + m.steps,
    }
}

fn count_wraps(position: i32, m: &Move) -> i32 {
    match m.direction {
        Direction::Right => (position + m.steps).div_euclid(100) - position.div_euclid(100),
        Direction::Left => {
            (position - 1).div_euclid(100) - (position - m.steps - 1).div_euclid(100)
        }
    }
}

fn part_one(input: &str) -> i32 {
    let starting_position = 50;
    input
        .lines()
        .map(parse_move)
        .fold((starting_position, 0), |(position, result), m| {
            let new_position = apply_move(position, &m);
            let new_result = if new_position % 100 == 0 {
                result + 1
            } else {
                result
            };
            (new_position, new_result)
        })
        .1
}

fn part_two(input: &str) -> i32 {
    // check every time the dial passes 0, not only hits 0
    let starting_position = 50;
    input
        .lines()
        .map(parse_move)
        .fold((starting_position, 0), |(position, result), m| {
            let new_position = apply_move(position, &m);
            let new_result = result + count_wraps(position, &m);
            (new_position, new_result)
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_INPUT), 3);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_INPUT), 6);
    }
}
