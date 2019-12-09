use failure::Error;
use std::fs;
use itertools::Itertools;
use std::collections::VecDeque;

type Int = i64;
const MEM_SIZE: usize = 1_000_000;

enum Opcode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    RelativeBase,
    Halt,
}

#[derive(Debug, Eq, PartialEq)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

struct Computer {
    ip: usize,  // instruction pointer
    rb: Int,    // relative base
    mem: Vec<Int>,
    halted: bool,
    last_output: Int,
    input_queue: VecDeque<Int>,
    output_queue: VecDeque<Int>,
}

struct Instruction {
    v: Int,
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
            9 => RelativeBase,
            99 => Halt,
            _ => panic!("Invalid opcode: {}", self.v),
        }
    }

    fn param_mode(&self, param_index: Int) -> ParamMode {
        use ParamMode::*;
        match self.v / 10_i64.pow((param_index + 1) as u32) % 10 {
            0 => Position,
            1 => Immediate,
            2 => Relative,
            _ => panic!(
                "Invalid parameter mode. v:{} param_index:{}",
                self.v, param_index
            ),
        }
    }
}

impl Computer {
    fn new(mem: Vec<Int>) -> Self {       
        Computer {
            ip: 0,
            rb: 0,
            mem,
            halted: false,
            last_output: 0,
            input_queue: VecDeque::new(),
            output_queue: VecDeque::new(),
        }
    }

    fn instruction(&self) -> Instruction {
        Instruction {
            v: self.mem[self.ip],
        }
    }

    fn raw_param(&self, i: Int) -> Int {
        self.mem[self.ip + i as usize]
    }

    fn param(&self, inst: &Instruction, i: Int) -> Int {
        match inst.param_mode(i) {
            ParamMode::Immediate => self.raw_param(i),
            ParamMode::Position => self.mem[self.raw_param(i) as usize],
            ParamMode::Relative => self.mem[(self.rb + self.raw_param(i)) as usize],
        }
    }

    fn write_param(&mut self, inst: &Instruction, i: Int, v: Int) {
        match inst.param_mode(i) {
            ParamMode::Immediate => panic!("Immediate mode not supported for writes!"),
            ParamMode::Position => {
                let p = self.raw_param(i);
                self.mem[p as usize] = v;
            },
            ParamMode::Relative => {
                let p = self.raw_param(i);
                self.mem[(self.rb + p) as usize] = v;
            },
        }
    }


    fn inject_input(&mut self, v: Int) {
        self.input_queue.push_back(v);
    }

    fn read_input(&mut self) -> Option<Int> {
        self.input_queue.pop_front()
    }

    fn write_output(&mut self, v: Int) {
        println!("OUTPUT: {}", v);
        self.last_output = v;
        self.output_queue.push_back(v);
    }

    fn read_output(&mut self) -> Option<Int> {
        self.output_queue.pop_front()
    }

    fn exec_instruction(&mut self) {
        println!("--ip:{}--rb:{}------", self.ip, self.rb);
        let inst = self.instruction();
        println!(">> {}", inst.v);

        match inst.opcode() {
            Opcode::Input => {
                if let Some(input) = self.read_input() {
                    let src = input;
                    //let dst = self.param(&inst, 1) as usize;
                    println!("Input {}", src);
                    //self.mem[dst] = src;
                    self.write_param(&inst, 1, src);
                    self.ip += 2;   
                }
            }
            Opcode::Output => {
                let src = self.param(&inst, 1);
                println!("Output {}", src);
                self.write_output(src);
                self.ip += 2;
            }
            Opcode::Add => {
                let src1 = self.param(&inst, 1);
                let src2 = self.param(&inst, 2);
                println!("Add    {} + {}", src1, src2);
                self.write_param(&inst, 3, src1 + src2);
                self.ip += 4;
            }
            Opcode::Mul => {
                let src1 = self.param(&inst, 1);
                let src2 = self.param(&inst, 2);
                println!("Mul    {} * {}", src1, src2);
                self.write_param(&inst, 3, src1 * src2);
                self.ip += 4;
            }
            Opcode::JumpIfTrue => {
                let src = self.param(&inst, 1);
                let dst = self.param(&inst, 2) as usize;
                println!("JumpIfTrue  {} [{}]", src, dst);
                if src != 0 {
                    self.ip = dst;
                } else {
                    self.ip += 3;
                }
            }
            Opcode::JumpIfFalse => {
                let src = self.param(&inst, 1);
                let dst = self.param(&inst, 2) as usize;
                println!("JumpIfFalse {} [{}]", src, dst);
                if src == 0 {
                    self.ip = dst;
                } else {
                    self.ip += 3;
                }
            }
            Opcode::LessThan => {
                let src1 = self.param(&inst, 1);
                let src2 = self.param(&inst, 2);
                println!("LessThan  {} < {}", src1, src2);
                self.write_param(&inst, 3, if src1 < src2 { 1 } else { 0 });
                self.ip += 4;
            }
            Opcode::Equals => {
                let src1 = self.param(&inst, 1);
                let src2 = self.param(&inst, 2);
                println!("Equals  {} == {}", src1, src2);
                self.write_param(&inst, 3, if src1 == src2 { 1 } else { 0 });
                self.ip += 4;
            }
            Opcode::RelativeBase => {
                let src = self.param(&inst, 1);
                println!("RelativeBase {}", src);
                self.rb += src;
                self.ip += 2;
            }
            Opcode::Halt => {
                println!("Halt");
                self.halted = true;
            }
        }
    }

    fn run(&mut self) {
        while !self.halted {
            self.exec_instruction()
        }
    }
}

fn load_program(prog: &str) -> Vec<Int> {
    let mut mem = prog.split(',')
        .map(|i| i.parse::<Int>().unwrap())
        .collect::<Vec<Int>>();

    mem.resize(MEM_SIZE, 0);
    mem
}

fn main() -> Result<(), Error> {
    let program = load_program(&fs::read_to_string("data.txt")?);
    let mut c = Computer::new(program.into());
    c.inject_input(1);
    c.run();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_mode() {
        assert_eq!(Instruction{v: 100}.param_mode(1), ParamMode::Immediate);
        assert_eq!(Instruction{v: 100}.param_mode(2), ParamMode::Position);
        assert_eq!(Instruction{v: 100}.param_mode(3), ParamMode::Position);
        assert_eq!(Instruction{v: 1000}.param_mode(1), ParamMode::Position);
        assert_eq!(Instruction{v: 1100}.param_mode(2), ParamMode::Immediate);
        assert_eq!(Instruction{v: 1100}.param_mode(3), ParamMode::Position);
    }

    #[test]
    fn test_cpu() {
        let test_data = [
            ("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",),
            ("1102,34915192,34915192,7,4,7,99,0",),
            ("104,1125899906842624,99",),
        ];

        for (code,) in &test_data {
            let program = load_program(code);
            let mut c = Computer::new(program.into());
            c.run();
        }
    }
}
