use failure::Error;
use std::fs;

type MemCell = u32;

const OPCODE_ADD: MemCell = 1;
const OPCODE_MUL: MemCell = 2;
const OPCODE_HALT: MemCell = 99;

struct Computer {
    pc: usize,
    mem: Vec<MemCell>,
    halted: bool,
}

impl Computer {
    fn new(mem: Vec<MemCell>) -> Self {
        Computer {
            pc: 0,
            mem,
            halted: false,
        }
    }

    fn opcode(&self) -> MemCell {
        self.mem[self.pc]
    }

    fn operand(&self, i: usize) -> MemCell {
        self.mem[self.pc + i]
    }

    fn exec_instruction(&mut self) {
        if self.halted {
            return;
        }

        match self.opcode() {
            OPCODE_ADD => {
                let src1 = self.operand(1) as usize;
                let src2 = self.operand(2) as usize;
                let dst = self.operand(3) as usize;
                self.mem[dst] = self.mem[src1] + self.mem[src2];
                self.pc += 4
            }
            OPCODE_MUL => {
                let src1 = self.operand(1) as usize;
                let src2 = self.operand(2) as usize;
                let dst = self.operand(3) as usize;
                self.mem[dst] = self.mem[src1] * self.mem[src2];
                self.pc += 4
            }
            OPCODE_HALT => self.halted = true,
            _ => panic!("Invalid opcode: {}", self.opcode()),
        }
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

    fn core_dump(&self) {
        println!("{:?}", self.mem)
    }
}

fn load_program(prog: &str) -> Vec<MemCell> {
    prog.split(',')
        .map(|i| i.parse::<MemCell>().unwrap())
        .collect()
}

fn main() -> Result<(), Error> {
    let mut program = Computer::new(load_program(&fs::read_to_string("data.txt")?));
    program.write_mem(1, 12);
    program.write_mem(2, 2);
    program.exec().core_dump();

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
