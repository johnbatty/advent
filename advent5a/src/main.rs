use failure::Error;
use std::fs;

type MemCell = i32;

const OPCODE_ADD: i32 = 1;
const OPCODE_MUL: i32 = 2;
const OPCODE_INPUT: i32 = 3;
const OPCODE_OUTPUT: i32 = 4;
const OPCODE_HALT: i32 = 99;

enum Opcode {
    Add,
    Mul,
    Input,
    Output,
    Halt,
}

enum ParamMode {
    Position,
    Immediate,
}

struct Computer {
    ip: usize,
    mem: Vec<MemCell>,
    halted: bool,
}

struct Instruction {
    v: MemCell,
}

impl Instruction {
    fn opcode(&self) -> Opcode {
        use Opcode::*;
        match self.v % 100 {
            OPCODE_ADD => Add,
            OPCODE_MUL => Mul,
            OPCODE_INPUT => Input,
            OPCODE_OUTPUT => Output,
            OPCODE_HALT => Halt,
            _ => panic!("Invalid opcode: {}", self.v),
        }
    }

    fn param_mode(&self, param_index: i32) -> ParamMode {
        use ParamMode::*;
        match ((self.v as i32 / 10) / 10_i32.pow(param_index as u32)) % 10 {
            0 => Position,
            1 => Immediate,
            _ => panic!(
                "Invalid parameter mode. v:{} param_index:{}",
                self.v, param_index
            ),
        }
    }
}

impl Computer {
    fn new(mem: Vec<MemCell>) -> Self {
        Computer {
            ip: 0,
            mem,
            halted: false,
        }
    }

    fn instruction(&self) -> Instruction {
        println!("[{}] New instruction: {}", self.ip, self.mem[self.ip]);
        Instruction {
            v: self.mem[self.ip],
        }
    }

    fn raw_param(&self, i: i32) -> i32 {
        self.mem[self.ip + i as usize]
    }

    fn indirect_param(&self, i: i32) -> i32 {
        self.mem[self.raw_param(i) as usize] as i32
    }

    fn param(&self, inst: &Instruction, i: i32) -> i32 {
        match inst.param_mode(i) {
            ParamMode::Immediate => self.raw_param(i),
            ParamMode::Position => self.indirect_param(i),
        }
    }

    fn read_input(&self) -> i32 {
        1
    }

    fn write_output(&self, v: i32) {
        println!("{}", v);
    }

    fn exec_instruction(&mut self) {
        let inst = self.instruction();

        match inst.opcode() {
            Opcode::Input => {
                let src = self.read_input();
                let dst = self.raw_param(1) as usize;
                self.mem[dst] = src as MemCell;
                println!("Input | {} -> [{}]", src, dst);
                self.ip += 2;
            }
            Opcode::Output => {
                let src1 = self.param(&inst, 1);
                println!("Output| {}", src1);
                self.write_output(src1);
                self.ip += 2;
            }
            Opcode::Add => {
                let src1 = self.param(&inst, 1);
                let src2 = self.param(&inst, 2);
                let dst = self.raw_param(3) as usize;
                println!("Add   | {} + {} -> [{}]", src1, src2, dst);
                self.mem[dst] = src1 + src2 as MemCell;
                self.ip += 4;
            }
            Opcode::Mul => {
                let src1 = self.param(&inst, 1);
                let src2 = self.param(&inst, 2);
                let dst = self.raw_param(3) as usize;
                println!("Mul   | {} * {} -> [{}]", src1, src2, dst);
                self.mem[dst] = src1 * src2 as MemCell;
                self.ip += 4;
            }
            Opcode::Halt => {
                println!("Halt  |");
                self.halted = true;
            }
        }
    }

    fn exec(mut self) -> Self {
        while !self.halted {
            self.exec_instruction()
        }
        self
    }
}

fn load_program(prog: &str) -> Vec<MemCell> {
    prog.split(',')
        .map(|i| i.parse::<MemCell>().unwrap())
        .collect()
}

fn main() -> Result<(), Error> {
    let program = load_program(&fs::read_to_string("data.txt")?);

    let computer = Computer::new(program.clone());
    computer.exec();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu() {}
}
