#[aoc::main(04)]
fn main(input: &str) -> (u32, u32) {
    solve(input)
}

/// A flattened grid with padding to avoid boundary checks.
/// The grid is padded with a 1-cell border of '.' (inactive) cells.
struct Grid {
    /// Flattened grid data (padding included).
    cells: Vec<u8>,
    /// Active neighbor counts for each cell.
    neighbor_counts: Vec<u8>,
    /// Width of the grid including padding.
    stride: usize,
    /// Original height of the input grid.
    inner_rows: usize,
    /// Original width of the input grid.
    inner_cols: usize,
    /// Pre-calculated relative offsets for the 8 neighbors.
    neighbor_offsets: [isize; 8],
}

impl Grid {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let inner_rows = lines.len();
        let inner_cols = lines[0].len();
        // Stride is the width including 1-cell padding on left and right
        let stride = inner_cols + 2;
        let total_size = (inner_rows + 2) * stride;

        let mut cells = vec![b'.'; total_size];
        let neighbor_counts = vec![0u8; total_size];

        // Parse input into the center of the padded grid
        for (r, line) in lines.iter().enumerate() {
            let row_start = (r + 1) * stride + 1;
            for (c, ch) in line.bytes().enumerate() {
                cells[row_start + c] = ch;
            }
        }

        // Pre-calculate offsets for neighbors (top-left to bottom-right)
        let stride_isize = stride as isize;
        let neighbor_offsets = [
            -stride_isize - 1,
            -stride_isize,
            -stride_isize + 1,
            -1,
            1,
            stride_isize - 1,
            stride_isize,
            stride_isize + 1,
        ];

        Self {
            cells,
            neighbor_counts,
            stride,
            inner_rows,
            inner_cols,
            neighbor_offsets,
        }
    }

    /// Returns true if the cell at the given index is active ('@').
    #[inline(always)]
    fn is_active(&self, index: usize) -> bool {
        self.cells[index] == b'@'
    }

    /// Marks the cell at the given index as inactive ('.').
    #[inline(always)]
    fn deactivate(&mut self, index: usize) {
        self.cells[index] = b'.';
    }

    /// Calculates initial neighbor counts for all active cells and returns
    /// a queue of cells that should die in the first iteration (count < 4).
    fn initialize_counts_and_queue(&mut self) -> Vec<usize> {
        let mut queue = Vec::with_capacity(self.inner_rows * self.inner_cols / 4);

        // Iterate only over the inner (valid) grid area
        for r in 1..=self.inner_rows {
            for c in 1..=self.inner_cols {
                let index = r * self.stride + c;
                if self.is_active(index) {
                    let mut count = 0;
                    for &offset in &self.neighbor_offsets {
                        // Safe unchecked access because of padding
                        let neighbor_index = (index as isize + offset) as usize;
                        if self.is_active(neighbor_index) {
                            count += 1;
                        }
                    }
                    self.neighbor_counts[index] = count;
                    if count < 4 {
                        queue.push(index);
                    }
                }
            }
        }
        queue
    }
}

fn solve(input: &str) -> (u32, u32) {
    let mut grid = Grid::new(input);
    let mut queue = grid.initialize_counts_and_queue();

    let part_one = queue.len() as u32;
    let mut part_two = part_one;

    // Process the queue of dying cells
    let mut head = 0;
    while head < queue.len() {
        let current_index = queue[head];
        head += 1;

        grid.deactivate(current_index);

        // Notify neighbors of the death
        for &offset in &grid.neighbor_offsets {
            let neighbor_index = (current_index as isize + offset) as usize;

            // If neighbor is active, decrement its count
            if grid.is_active(neighbor_index) {
                grid.neighbor_counts[neighbor_index] -= 1;

                // If neighbor drops below threshold (exactly to 3), it dies next
                if grid.neighbor_counts[neighbor_index] == 3 {
                    queue.push(neighbor_index);
                    part_two += 1;
                }
            }
        }
    }

    (part_one, part_two)
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
