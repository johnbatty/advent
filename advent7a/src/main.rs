use failure::Error;
use std::fs;
use itertools::Itertools;

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

#[derive(Debug, Eq, PartialEq)]
enum ParamMode {
    Position,
    Immediate,
}

struct Computer {
    ip: usize,
    mem: Vec<MemCell>,
    halted: bool,
    input_queue: Vec<i32>,
    output_queue: Vec<i32>,
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
        match self.v as i32 / 10_i32.pow((param_index + 1) as u32) % 10 {
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
            output_queue: vec![],
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
        self.input_queue.remove(0)
    }

    fn write_output(&mut self, v: i32) {
        self.output_queue.push(v);
        println!("write_output {}", v);
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

// 5 amplifiers.
fn run_thruster_amps(program: &Vec<MemCell>, phases: &Vec<i32>) -> i32 {
    let mut output_signal = 0;
    for phase in phases {
        let input_signal = output_signal;
        let input_queue = vec![*phase, input_signal];
        println!("input_queue: {:?}", &input_queue);
        let computer = Computer::new(program.clone(), input_queue);
        let computer = computer.exec();
        assert_eq!(computer.output_queue.len(), 1);
        output_signal = computer.output_queue[0];
        println!("output_signal:{}", output_signal);
    }
    output_signal
}

fn maximise_thruster_power(program: &Vec<MemCell>) -> (i32, Vec<i32>) {
    let mut max_power = 0;
    let mut best_phases = vec![];
    for phases in (0..=4).permutations(NUM_AMPS as usize) {
        println!("{:?}", phases);
        let output = run_thruster_amps(&program, &phases);
        println!("output: {}", output);
        if output > max_power {
            max_power = output;
            best_phases = phases.clone();
        }
    }
    (max_power, best_phases)
}

const NUM_AMPS: i32 = 5;
fn main() -> Result<(), Error> {
    let program = load_program(&fs::read_to_string("data.txt")?);

    let (max_power, phases) = maximise_thruster_power(&program); 
    println!("max_power:{}, phases:{:?}", max_power, &phases);

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
            ("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", 43210_i32),
            ("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0", 54321_i32),
            ("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0", 65210),
        ];

        for (code, expected_power) in &test_data {
            let program = load_program(code);
            let (max_power, phases) = maximise_thruster_power(&program);
            println!("max_power:{} phases:{:?}", max_power, &phases);
            assert_eq!(max_power, *expected_power);
        }
    }
}
