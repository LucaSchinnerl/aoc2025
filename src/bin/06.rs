#[aoc::main(06)]
fn main(input: &str) -> (u64, u64) {
    let (part_one, part_two) = rayon::join(|| part_one(input), || part_two(input));

    (part_one, part_two)
}

enum Instruction {
    Add,
    Multiply,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        if line.contains("+") || line.contains("*") {
            for instruction in line.split_whitespace() {
                instructions.push(if instruction == "+" {
                    Instruction::Add
                } else {
                    Instruction::Multiply
                });
            }
        }
    }
    instructions
}

fn parse_part_one_input(input: &str) -> Vec<Vec<u64>> {
    let first_line = input.lines().find(|l| !l.trim().is_empty()).unwrap();
    let num_columns = first_line.split_whitespace().count();
    let mut grid = vec![Vec::new(); num_columns];

    for line in input.lines() {
        if line.contains("+") || line.contains("*") {
            continue;
        }
        for (j, num) in line.split_whitespace().enumerate() {
            grid[j].push(num.parse::<u64>().unwrap());
        }
    }
    grid
}

fn parse_part_two_input(input: &str) -> Vec<Vec<u64>> {
    let mut col_parse = Vec::<Vec<u64>>::new();
    col_parse.push(Vec::new());
    for i in 0..input.lines().next().unwrap().len() {
        let mut string_number = String::new();
        for line in input.lines() {
            if line.contains("*") || line.contains("+") {
                continue;
            }
            string_number.push_str(line[i..i + 1].trim());
        }
        if string_number.is_empty() {
            col_parse.push(Vec::new());
        } else {
            col_parse
                .last_mut()
                .unwrap()
                .push(string_number.parse::<u64>().unwrap());
        }
    }
    col_parse.retain(|v| !v.is_empty());
    col_parse
}

fn calculate_result(instructions: &[Instruction], grid: &[Vec<u64>]) -> u64 {
    let mut res = 0;
    for (instruction, values) in instructions.iter().zip(grid.iter()) {
        let mut current_value = 0;
        match instruction {
            Instruction::Add => {
                current_value += values.iter().sum::<u64>();
            }
            Instruction::Multiply => {
                current_value += values.iter().product::<u64>();
            }
        }
        res += current_value;
    }
    res
}

fn part_one(input: &str) -> u64 {
    let instructions = parse_instructions(input);
    let grid = parse_part_one_input(input);
    calculate_result(&instructions, &grid)
}

fn part_two(input: &str) -> u64 {
    let instructions = parse_instructions(input);
    let grid = parse_part_two_input(input);
    calculate_result(&instructions, &grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 4277556);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 3263827);
    }
}
