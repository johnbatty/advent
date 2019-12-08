use failure::Error;
use std::fs;
use itertools::Itertools;
use std::collections::VecDeque;

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
    last_output: i32,
    input_queue: VecDeque<i32>,
    output_queue: VecDeque<i32>,
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
    fn new(mem: Vec<MemCell>) -> Self {
        
        Computer {
            ip: 0,
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

    fn inject_input(&mut self, v: i32) {
        self.input_queue.push_back(v);
    }

    fn read_input(&mut self) -> Option<i32> {
        self.input_queue.pop_front()
    }

    fn write_output(&mut self, v: i32) {
        println!("write_output {}", v);
        self.last_output = v;
        self.output_queue.push_back(v);
    }

    fn read_output(&mut self) -> Option<i32> {
        self.output_queue.pop_front()
    }

    fn exec_instruction(&mut self) {
        let inst = self.instruction();

        match inst.opcode() {
            Opcode::Input => {
                match self.read_input() {
                    Some(input) => {
                        let src = input;
                        let dst = self.raw_param(1) as usize;
                        self.mem[dst] = src as MemCell;
                        self.ip += 2;   
                    },
                    None => {},
                }
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

    fn exec(&mut self) {
        while !self.halted {
            self.exec_instruction()
        }
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

    let mut computers: Vec<Computer> = phases
        .iter()
        .map(|phase| {
            let mut c = Computer::new(program.clone());
            c.inject_input(*phase);
            c
        })
        .collect();

    let mut output = Some(0);
    let mut computers_are_running = true;
    while computers_are_running {
        computers_are_running = false;
        for computer in &mut computers {
            match output {
                Some(v) => {
                    println!("Inject {:?}", &v);
                    computer.inject_input(v);
                }
                None => {}
            }
            computer.exec_instruction();
            output = computer.read_output();
            if !computer.halted {
                computers_are_running = true;
            }
        }
    }



    // for phase in phases {
    //     let input_signal = output_signal;
    //     let input_queue = vec![*phase, input_signal];
    //     println!("input_queue: {:?}", &input_queue);
    //     let mut computer = Computer::new(program.clone());
    //     assert_eq!(computer.output_queue.len(), 1);
    //     output_signal = computer.output_queue[0];
    //     println!("output_signal:{}", output_signal);
    // }
 
    // computer.exec();


    computers[4].last_output
}

fn maximise_thruster_power(program: &Vec<MemCell>) -> (i32, Vec<i32>) {
    let mut max_power = 0;
    let mut best_phases = vec![];
    for phases in (5..=9).permutations(NUM_AMPS as usize) {
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
            ("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5", 139629729),
            ("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",18216)
        ];

        for (code, expected_power) in &test_data {
            let program = load_program(code);
            let (max_power, phases) = maximise_thruster_power(&program);
            println!("max_power:{} phases:{:?}", max_power, &phases);
            assert_eq!(max_power, *expected_power);
        }
    }
}
