#[aoc::main(04)]
fn main(input: &str) -> (u32, u32) {
    solve(input)
}

struct Coordinate {
    x: usize,
    y: usize,
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn create_window_indices(mid_point: Coordinate, input: &[Vec<char>]) -> Vec<Coordinate> {
    // Find all coordinates around the mid_point, in a 3x3 grid, excluding the mid_point. Checks bounds around the edges of the input.
    let rows = input.len();
    let cols = input[0].len();

    (-1..=1)
        .flat_map(|dy| (-1..=1).map(move |dx| (dx, dy)))
        .filter(|&(dx, dy)| dx != 0 || dy != 0)
        .filter_map(|(dx, dy)| {
            let x = mid_point.x.checked_add_signed(dx)?;
            let y = mid_point.y.checked_add_signed(dy)?;
            if x < cols && y < rows {
                Some(Coordinate { x, y })
            } else {
                None
            }
        })
        .collect()
}

fn solve(input: &str) -> (u32, u32) {
    let mut grid = parse_input(input);
    // let mut dbg_output = grid.clone();
    let mut result = (0, 0);

    let mut iteration = 1;
    loop {
        let mut coordinates_to_remove = Vec::new();
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == '.' {
                    continue;
                }
                let indices: Vec<Coordinate> = create_window_indices(Coordinate { x, y }, &grid);
                // Count number of @ in the window
                let count = indices.iter().filter(|&c| grid[c.y][c.x] == '@').count();
                if count < 4 {
                    coordinates_to_remove.push(Coordinate { x, y });
                    // dbg_output[y][x] = 'x';
                }
            }
        }
        if coordinates_to_remove.is_empty() {
            break;
        }
        for coordinate in &coordinates_to_remove {
            grid[coordinate.y][coordinate.x] = '.';
        }
        if iteration == 1 {
            result.0 += coordinates_to_remove.len() as u32;
        }
        result.1 += coordinates_to_remove.len() as u32;
        iteration += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part_one() {
        assert_eq!(solve(EXAMPLE_INPUT).0, 13);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve(EXAMPLE_INPUT).1, 43);
    }
}
