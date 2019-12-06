use failure::Error;
use std::fs;

type MemCell = i32;

enum Opcode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
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
    input_queue: Vec<i32>,
}

struct Instruction {
    v: MemCell,
}

impl Instruction {
    fn opcode(&self) -> Opcode {
        use Opcode::*;
        match self.v % 100 {
            1 => Add,
            2 => Mul,
            3 => Input,
            4 => Output,
            5 => JumpIfTrue,
            6 => JumpIfFalse,
            7 => LessThan,
            8 => Equals,
            99 => Halt,
            _ => panic!("Invalid opcode: {}", self.v),
        }
    }

    fn param_mode(&self, param_index: i32) -> ParamMode {
        use ParamMode::*;
        match self.v as i32 / 100_i32.pow(param_index as u32) % 10 {
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
    fn new(mem: Vec<MemCell>, input_queue: Vec<i32>) -> Self {
        Computer {
            ip: 0,
            mem,
            halted: false,
            input_queue,
        }
    }

    fn instruction(&self) -> Instruction {
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

    fn read_input(&mut self) -> i32 {
        self.input_queue.pop().unwrap()
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
                self.ip += 2;
            }
            Opcode::Output => {
                let src = self.param(&inst, 1);
                self.write_output(src);
                self.ip += 2;
            }
            Opcode::Add => {
                let src1 = self.param(&inst, 1);
                let src2 = self.param(&inst, 2);
                let dst = self.raw_param(3) as usize;
                self.mem[dst] = src1 + src2 as MemCell;
                self.ip += 4;
            }
            Opcode::Mul => {
                let src1 = self.param(&inst, 1);
                let src2 = self.param(&inst, 2);
                let dst = self.raw_param(3) as usize;
                self.mem[dst] = src1 * src2 as MemCell;
                self.ip += 4;
            }
            Opcode::JumpIfTrue => {
                let src = self.param(&inst, 1);
                let dst = self.param(&inst, 2) as usize;
                if src != 0 {
                    self.ip = dst;
                } else {
                    self.ip += 3;
                }
            }
            Opcode::JumpIfFalse => {
                let src = self.param(&inst, 1);
                let dst = self.param(&inst, 2) as usize;
                if src == 0 {
                    self.ip = dst;
                } else {
                    self.ip += 3;
                }
            }
            Opcode::LessThan => {
                let src1 = self.param(&inst, 1);
                let src2 = self.param(&inst, 2);
                let dst = self.raw_param(3) as usize;
                self.mem[dst] = if src1 < src2 { 1 } else { 0 };
                self.ip += 4;
            }
            Opcode::Equals => {
                let src1 = self.param(&inst, 1);
                let src2 = self.param(&inst, 2);
                let dst = self.raw_param(3) as usize;
                self.mem[dst] = if src1 == src2 { 1 } else { 0 };
                self.ip += 4;
            }
            Opcode::Halt => {
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
    let input_queue = vec![5];

    let computer = Computer::new(program.clone(), input_queue);
    computer.exec();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu() {}
}
