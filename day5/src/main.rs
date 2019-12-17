use std::convert::TryFrom;
use std::error::Error;
use std::fs;

trait Digit {
    fn digit(&self, n: i32) -> i32;
}

impl Digit for i32 {
    fn digit(&self, n: i32) -> i32 {
        let mut x = *self;

        for _ in 0..n {
            x /= 10;
        }

        x % 10
    }
}

#[derive(Debug, Copy, Clone)]
enum ParameterMode {
    Position,
    Immediate,
}

impl From<i32> for ParameterMode {
    fn from(n: i32) -> Self {
        match n {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            unknown => unreachable!("unknown mode: {}", unknown),
        }
    }
}

struct Intcode {
    mem: Vec<i32>,
    input: Vec<i32>,
}

impl Intcode {
    fn load(program: &str, input: Vec<i32>) -> Self {
        let program = program.split(',').map(|opcode| opcode.trim().parse().unwrap()).collect();

        Intcode {
            mem: program,
            input,
        }
    }

    fn execute(&mut self) -> Vec<i32> {
        let mut output = vec![];

        let mut pc = 0;

        loop {
            let instr = self.mem[pc];
            pc += 1;

            let opcode = instr % 100;
            let modes = [ instr.digit(2).into(), instr.digit(3).into(), instr.digit(4).into()];

            match opcode {
                1 => {
                    let arg0 = self.read(self.mem[pc], modes[0]);
                    let arg1 = self.read(self.mem[pc + 1], modes[1]);
                    let dst = usize::try_from(self.mem[pc + 2]).unwrap();

                    self.mem[dst] = arg0 + arg1;
                    pc += 3;
                }
                2 => {
                    let arg0 = self.read(self.mem[pc], modes[0]);
                    let arg1 = self.read(self.mem[pc + 1], modes[1]);
                    let dst = usize::try_from(self.mem[pc + 2]).unwrap();

                    self.mem[dst] = arg0 * arg1;
                    pc += 3;
                }
                3 => {
                    let dst = usize::try_from(self.mem[pc]).unwrap();
                    self.mem[dst] = self.input.remove(0);
                    pc += 1;
                }
                4 => {
                    output.push(self.read(self.mem[pc], modes[0]));
                    pc += 1;
                }
                99 => break,
                unknown_opcode => unreachable!("unknown opcode: {}", unknown_opcode),
            }
        }

        output
    }

    fn read(&self, addr: i32, mode: ParameterMode) -> i32 {
        match mode {
            ParameterMode::Position => self.mem[usize::try_from(addr).unwrap()],
            ParameterMode::Immediate => addr,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let program = fs::read_to_string("inputs/day5.txt")?;
    let mut computer = Intcode::load(&program, vec![1]);
    let output = computer.execute();

    assert!(output[..output.len() - 1].iter().all(|&code| code == 0));

    println!("part 1: {}", output.last().unwrap());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Digit, Intcode};

    #[test]
    fn day2_test_case_1() {
        let mut computer = Intcode::load("1,0,0,0,99", vec![]);
        computer.execute();
        assert_eq!(computer.mem, &[2, 0, 0, 0, 99]);
    }

    #[test]
    fn day2_test_case_2() {
        let mut computer = Intcode::load("2,3,0,3,99", vec![]);
        computer.execute();

        assert_eq!(computer.mem, &[2, 3, 0, 6, 99]);
    }

    #[test]
    fn day2_test_case_3() {
        let mut computer = Intcode::load("2,4,4,5,99,0", vec![]);
        computer.execute();
        assert_eq!(computer.mem, &[2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn day2_test_case_4() {
        let mut computer = Intcode::load("1,1,1,4,99,5,6,0,99", vec![]);
        computer.execute();
        assert_eq!(computer.mem, &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn digit() {
        assert_eq!(123.digit(0), 3);
        assert_eq!(123.digit(1), 2);
        assert_eq!(123.digit(2), 1);
    }

    #[test]
    fn example1() {
        let mut computer = Intcode::load("3,0,4,0,99", vec![1337]);
        let output = computer.execute();
        assert_eq!(output, vec![1337]);
    }

    #[test]
    fn example2() {
        let mut computer = Intcode::load("1002,4,3,4,33", vec![]);
        computer.execute();
        assert_eq!(computer.mem[4], 99);
    }

    #[test]
    fn example3() {
        let mut computer = Intcode::load("1101,100,-1,4,0", vec![]);
        computer.execute();
        assert_eq!(computer.mem[4], 99);
    }
}
