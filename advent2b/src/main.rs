use failure::Error;
use std::fs;

type MemCell = u32;

const OPCODE_ADD: MemCell = 1;
const OPCODE_MUL: MemCell = 2;
const OPCODE_HALT: MemCell = 99;

struct Computer {
    ip: usize,
    mem: Vec<MemCell>,
    halted: bool,
}

impl Computer {
    fn new(mem: Vec<MemCell>) -> Self {
        Computer {
            ip: 0,
            mem,
            halted: false,
        }
    }

    fn opcode(&self) -> MemCell {
        self.mem[self.ip]
    }

    fn param(&self, i: usize) -> MemCell {
        self.mem[self.ip + i]
    }

    fn exec_instruction(&mut self) {
        if self.halted || self.opcode() == OPCODE_HALT {
            self.halted = true;
            return;
        }

        let src1 = self.param(1) as usize;
        let src2 = self.param(2) as usize;
        let dst = self.param(3) as usize;

        match self.opcode() {
            OPCODE_ADD => {
                self.mem[dst] = self.mem[src1] + self.mem[src2];
            }
            OPCODE_MUL => {
                self.mem[dst] = self.mem[src1] * self.mem[src2];
            }
            _ => panic!("Invalid opcode: {}", self.opcode()),
        }

        self.ip += 4
    }

    fn exec(mut self) -> Self {
        while !self.halted {
            self.exec_instruction()
        }
        self
    }

    fn write_mem(&mut self, pos: usize, val: MemCell) {
        self.mem[pos] = val;
    }

    fn read_mem(&mut self, pos: usize) -> MemCell {
        self.mem[pos]
    }

    fn _core_dump(&self) {
        println!("{:?}", self.mem)
    }
}

fn load_program(prog: &str) -> Vec<MemCell> {
    prog.split(',')
        .map(|i| i.parse::<MemCell>().unwrap())
        .collect()
}

fn main() -> Result<(), Error> {
    const TARGET_OUTPUT: MemCell = 19_690_720;

    let program = load_program(&fs::read_to_string("data.txt")?);

    'outer: for noun in 0..99 {
        for verb in 0..99 {
            let mut computer = Computer::new(program.clone());
            computer.write_mem(1, noun);
            computer.write_mem(2, verb);
            let mut computer = computer.exec();
            if computer.read_mem(0) == TARGET_OUTPUT {
                println!("noun: {}, verb:{}", noun, verb);
                println!("Answer: {}", (100 * noun) + verb);
                break 'outer;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu() {
        let test_data = [
            ("1,0,0,0,99", "2,0,0,0,99"),
            ("2,3,0,3,99", "2,3,0,6,99"),
            ("2,4,4,5,99,0", "2,4,4,5,99,9801"),
            ("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99"),
        ];

        for (start_state, end_state) in &test_data {
            assert_eq!(
                Computer::new(load_program(start_state)).exec().mem,
                load_program(end_state)
            );
        }
    }
}
