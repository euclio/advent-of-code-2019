use std::error::Error;
use std::fs;

fn execute_program(memory: &mut [usize]) {
    let mut pc = 0;

    loop {
        match memory[pc] {
            1 => {
                memory[memory[pc + 3]] = memory[memory[pc + 1]] + memory[memory[pc + 2]];
            }
            2 => {
                memory[memory[pc + 3]] = memory[memory[pc + 1]] * memory[memory[pc + 2]];
            }
            99 => break,
            unknown_opcode => unreachable!("unknown opcode: {}", unknown_opcode)
        }

        pc += 4;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("inputs/day2.txt")?;

    let program = input
        .split(',')
        .map(|opcode| opcode.trim().parse())
        .collect::<Result<Vec<_>, _>>()?;

    let mut memory = program.clone();

    memory[1] = 12;
    memory[2] = 2;

    execute_program(&mut memory);

    println!("part 1: {}", memory[0]);

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut memory = program.clone();

            memory[1] = noun;
            memory[2] = verb;

            execute_program(&mut memory);

            if memory[0] == 19_690_720 {
                println!("part 2: {}", 100 * noun + verb);
                break 'outer;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::execute_program;

    #[test]
    fn test_case_1() {
        let mut memory = [1, 0, 0, 0, 99];
        execute_program(&mut memory);
        assert_eq!(&memory, &[2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_case_2() {
        let mut memory = [2, 3, 0, 3, 99];
        execute_program(&mut memory);
        assert_eq!(&memory, &[2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_case_3() {
        let mut memory = [2, 4, 4, 5, 99, 0];
        execute_program(&mut memory);
        assert_eq!(&memory, &[2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_case_4() {
        let mut memory = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        execute_program(&mut memory);
        assert_eq!(&memory, &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
