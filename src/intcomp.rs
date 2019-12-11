use std::collections::VecDeque;
use std::fs;
use std::str;

#[derive(Debug, PartialEq)]
enum Instruction {
    Add(OperandMode, OperandMode, OperandMode),
    Multiply(OperandMode, OperandMode, OperandMode),
    Input(OperandMode),
    Output(OperandMode),
    JumpIfTrue(OperandMode, OperandMode),
    JumpIfFalse(OperandMode, OperandMode),
    LessThan(OperandMode, OperandMode, OperandMode),
    Equals(OperandMode, OperandMode, OperandMode),
    AdjustRelativeBase(OperandMode),
    Halt,
}

impl Instruction {
    fn parse(instruction: i64) -> Instruction {
        let opcode = instruction % 100;

        match opcode {
            1 => Instruction::Add(
                OperandMode::from_digit(get_digit(instruction, 3)),
                OperandMode::from_digit(get_digit(instruction, 4)),
                OperandMode::from_digit(get_digit(instruction, 5)),
            ),

            2 => Instruction::Multiply(
                OperandMode::from_digit(get_digit(instruction, 3)),
                OperandMode::from_digit(get_digit(instruction, 4)),
                OperandMode::from_digit(get_digit(instruction, 5)),
            ),

            3 => Instruction::Input(OperandMode::from_digit(get_digit(instruction, 3))),
            4 => Instruction::Output(OperandMode::from_digit(get_digit(instruction, 3))),

            5 => Instruction::JumpIfTrue(
                OperandMode::from_digit(get_digit(instruction, 3)),
                OperandMode::from_digit(get_digit(instruction, 4)),
            ),

            6 => Instruction::JumpIfFalse(
                OperandMode::from_digit(get_digit(instruction, 3)),
                OperandMode::from_digit(get_digit(instruction, 4)),
            ),

            7 => Instruction::LessThan(
                OperandMode::from_digit(get_digit(instruction, 3)),
                OperandMode::from_digit(get_digit(instruction, 4)),
                OperandMode::from_digit(get_digit(instruction, 5)),
            ),

            8 => Instruction::Equals(
                OperandMode::from_digit(get_digit(instruction, 3)),
                OperandMode::from_digit(get_digit(instruction, 4)),
                OperandMode::from_digit(get_digit(instruction, 5)),
            ),

            9 => {
                Instruction::AdjustRelativeBase(OperandMode::from_digit(get_digit(instruction, 3)))
            }

            99 => Instruction::Halt,
            _ => panic!("Invalid opcode! ({})", opcode),
        }
    }
}

pub struct Intcomp {
    memory: Vec<i64>,
    ip: usize,
    bp: usize,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
}

impl Intcomp {
    pub fn execute(&mut self) {
        loop {
            let instruction = Instruction::parse(self.memory[self.ip]);

            match instruction {
                Instruction::Add(operand1_mode, operand2_mode, target_mode) => {
                    let operand1 = self.get_value(operand1_mode, self.ip + 1);
                    let operand2 = self.get_value(operand2_mode, self.ip + 2);
                    let target = self.get_target(target_mode, self.ip + 3);

                    self.extend_memory(target);
                    self.memory[target] = operand1 + operand2;
                    self.ip += 4;
                }

                Instruction::Multiply(operand1_mode, operand2_mode, target_mode) => {
                    let operand1 = self.get_value(operand1_mode, self.ip + 1);
                    let operand2 = self.get_value(operand2_mode, self.ip + 2);
                    let target = self.get_target(target_mode, self.ip + 3);

                    self.extend_memory(target);
                    self.memory[target] = operand1 * operand2;
                    self.ip += 4;
                }

                Instruction::Input(target_mode) => {
                    let target = self.get_target(target_mode, self.ip + 1);

                    let input = match self.input.pop_front() {
                        Some(input) => input,

                        // Terminate and await input.
                        None => return,
                    };

                    self.extend_memory(target);
                    self.memory[target] = input;
                    self.ip += 2;
                }

                Instruction::Output(output_mode) => {
                    let output = self.get_value(output_mode, self.ip + 1);

                    self.output.push_back(output);
                    self.ip += 2;
                }

                Instruction::JumpIfTrue(operand_mode, jump_to_mode) => {
                    let operand = self.get_value(operand_mode, self.ip + 1);
                    let jump_to = self.get_value(jump_to_mode, self.ip + 2);

                    if operand != 0 {
                        self.ip = jump_to as usize;
                    } else {
                        self.ip += 3;
                    }
                }

                Instruction::JumpIfFalse(operand_mode, jump_to_mode) => {
                    let operand = self.get_value(operand_mode, self.ip + 1);
                    let jump_to = self.get_value(jump_to_mode, self.ip + 2);

                    if operand == 0 {
                        self.ip = jump_to as usize;
                    } else {
                        self.ip += 3;
                    }
                }

                Instruction::LessThan(operand1_mode, operand2_mode, target_mode) => {
                    let operand1 = self.get_value(operand1_mode, self.ip + 1);
                    let operand2 = self.get_value(operand2_mode, self.ip + 2);
                    let target = self.get_target(target_mode, self.ip + 3);

                    self.extend_memory(target);
                    self.memory[target] = if operand1 < operand2 { 1 } else { 0 };
                    self.ip += 4;
                }

                Instruction::Equals(operand1_mode, operand2_mode, target_mode) => {
                    let operand1 = self.get_value(operand1_mode, self.ip + 1);
                    let operand2 = self.get_value(operand2_mode, self.ip + 2);
                    let target = self.get_target(target_mode, self.ip + 3);

                    self.extend_memory(target);
                    self.memory[target] = if operand1 == operand2 { 1 } else { 0 };
                    self.ip += 4;
                }

                Instruction::AdjustRelativeBase(operand_mode) => {
                    let operand = self.get_value(operand_mode, self.ip + 1);

                    if operand < 0 {
                        let operand = i64::abs(operand);

                        self.bp -= operand as usize
                    } else {
                        self.bp += operand as usize
                    }

                    self.ip += 2;
                }

                Instruction::Halt => return,
            }
        }
    }

    fn extend_memory(&mut self, address: usize) {
        if address >= self.memory.len() {
            self.memory.resize_with(address + 1, Default::default);
        }
    }

    /// get_target differs from get_value in that targets cannot be in Immediate mode.
    fn get_target(&mut self, operand_mode: OperandMode, operand_address: usize) -> usize {
        match operand_mode {
            OperandMode::Position => {
                let address = self.memory[operand_address] as usize;

                self.extend_memory(address);

                address
            }

            OperandMode::Immediate => panic!("cannot write to immediate operand"),

            OperandMode::Relative => {
                let operand = self.memory[operand_address];

                let address = if operand < 0 {
                    let operand = i64::abs(operand);

                    self.bp - operand as usize
                } else {
                    self.bp + operand as usize
                };

                self.extend_memory(address);

                address
            }
        }
    }

    fn get_value(&mut self, operand_mode: OperandMode, operand_address: usize) -> i64 {
        match operand_mode {
            OperandMode::Position => {
                let address = self.memory[operand_address] as usize;

                self.extend_memory(address);

                self.memory[address]
            }

            OperandMode::Immediate => self.memory[operand_address],

            OperandMode::Relative => {
                let operand = self.memory[operand_address];

                let address = if operand < 0 {
                    let operand = i64::abs(operand);

                    self.bp - operand as usize
                } else {
                    self.bp + operand as usize
                };

                self.extend_memory(address);

                self.memory[address]
            }
        }
    }

    pub fn is_halted(&self) -> bool {
        match Instruction::parse(self.memory[self.ip]) {
            Instruction::Halt => true,
            _ => false,
        }
    }

    pub fn new(intitial_memory: &[i64]) -> Intcomp {
        Intcomp {
            memory: intitial_memory.to_vec(),
            ip: 0,
            bp: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    pub fn read_memory(&self, address: usize) -> i64 {
        self.memory[address]
    }

    pub fn receive_output(&mut self) -> Option<i64> {
        self.output.pop_front()
    }

    pub fn send_input(&mut self, input: i64) {
        self.input.push_back(input);
    }

    pub fn write_memory(&mut self, address: usize, value: i64) {
        self.memory[address] = value;
    }
}

#[derive(Debug, PartialEq)]
enum OperandMode {
    Position,
    Immediate,
    Relative,
}

impl OperandMode {
    fn from_digit(digit: u8) -> OperandMode {
        match digit {
            0 => OperandMode::Position,
            1 => OperandMode::Immediate,
            2 => OperandMode::Relative,
            _ => panic!("Invalid address type! ({})", digit),
        }
    }
}

/// Get the nth digit from the right.
fn get_digit(number: i64, digit: u32) -> u8 {
    let base = i64::pow(10, digit - 1);

    ((number / base) % 10) as u8
}

pub fn read_program(path: &str) -> Vec<i64> {
    fs::read(path)
        .expect("could not read program file")
        .split(|byte| byte == &b',')
        .map(|number| str::from_utf8(&number).unwrap().parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_parse_supports_add() {
        match Instruction::parse(1) {
            Instruction::Add(_, _, _) => {}
            _ => panic!("expected Add"),
        }
    }

    #[test]
    fn instruction_parse_supports_multiply() {
        match Instruction::parse(2) {
            Instruction::Multiply(_, _, _) => {}
            _ => panic!("expected Multiply"),
        }
    }

    #[test]
    fn instruction_parse_supports_input() {
        match Instruction::parse(3) {
            Instruction::Input(_) => {}
            _ => panic!("expected Input"),
        }
    }

    #[test]
    fn instruction_parse_supports_output() {
        match Instruction::parse(4) {
            Instruction::Output(_) => {}
            _ => panic!("expected Output"),
        }
    }

    #[test]
    fn instruction_parse_supports_halt() {
        assert_eq!(Instruction::Halt, Instruction::parse(99));
    }

    #[test]
    #[should_panic]
    fn instruction_parse_panics_on_unsupported_opcode() {
        Instruction::parse(0);
    }

    #[test]
    fn instruction_parse_supports_address() {
        match Instruction::parse(1) {
            Instruction::Add(OperandMode::Position, _, _) => {}
            _ => panic!("expected Position"),
        }
    }

    #[test]
    fn instruction_parse_supports_immediate() {
        match Instruction::parse(101) {
            Instruction::Add(OperandMode::Immediate, _, _) => {}
            _ => panic!("expected Immediate"),
        }
    }

    #[test]
    fn intcomp_execute_halts() {
        let initializer = vec![99];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.execute();
    }

    #[test]
    fn intcomp_execute_can_add() {
        let initializer = vec![1, 0, 0, 0, 99];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.execute();

        assert_eq!(2, intcomp.read_memory(0));
    }

    #[test]
    fn intcomp_execute_can_multiply() {
        let initializer = vec![2, 0, 0, 0, 99];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.execute();

        assert_eq!(4, intcomp.read_memory(0));
    }

    #[test]
    fn intcomp_execute_can_input() {
        let initializer = vec![3, 0, 99];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.send_input(1);
        intcomp.execute();

        assert_eq!(1, intcomp.read_memory(0));
    }

    #[test]
    fn intcomp_execute_can_await_input() {
        let initializer = vec![3, 0, 99];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.execute();

        assert_eq!(3, intcomp.read_memory(0));

        intcomp.send_input(1);
        intcomp.execute();

        assert_eq!(1, intcomp.read_memory(0));
    }

    #[test]
    fn intcomp_execute_can_output() {
        let initializer = vec![104, 1, 99];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.execute();

        assert_eq!(1, intcomp.receive_output().expect("no output available"));
    }

    #[test]
    fn intcomp_execute_can_jump_if_true() {
        let initializer = vec![1105, 1, 4, 99, 1102, 0, 0, 0, 99];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.execute();

        assert_eq!(0, intcomp.read_memory(0));
    }

    #[test]
    fn intcomp_execute_can_jump_if_false() {
        let initializer = vec![1106, 0, 4, 99, 1102, 0, 0, 0, 99];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.execute();

        assert_eq!(0, intcomp.read_memory(0));
    }

    #[test]
    fn intcomp_execute_can_compare_less_than() {
        let initializer = vec![1107, 0, 1, 0, 99];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.execute();

        assert_eq!(1, intcomp.read_memory(0));
    }

    #[test]
    fn intcomp_execute_can_compare_equals() {
        let initializer = vec![1108, 0, 0, 0, 99];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.execute();

        assert_eq!(1, intcomp.read_memory(0));
    }

    #[test]
    fn intcomp_execute_can_adjust_relative_base() {
        let initializer = vec![109, 4, 204, 0, 99];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.execute();

        assert_eq!(99, intcomp.receive_output().expect("no output available"));
    }

    #[test]
    fn intcomp_execute_can_extend_memory_for_position_operand() {
        let initializer = vec![4, 10, 99];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.execute();

        assert_eq!(0, intcomp.receive_output().expect("no output available"));
    }

    #[test]
    fn intcomp_execute_can_extend_memory_for_relative_operand() {
        let initializer = vec![204, 10, 99];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.execute();

        assert_eq!(0, intcomp.receive_output().expect("no output available"));
    }

    #[test]
    #[should_panic]
    fn intcomp_execute_panics_on_unrecognized_opcode() {
        let initializer = vec![0, 0, 0, 0, 99];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.execute();
    }

    #[test]
    fn intcomp_execute_works_1() {
        let initializer = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.execute();

        assert_eq!(3500, intcomp.read_memory(0));
    }

    #[test]
    fn intcomp_execute_works_2() {
        let initializer = vec![2, 3, 0, 3, 99];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.execute();

        assert_eq!(6, intcomp.read_memory(3));
    }

    #[test]
    fn intcomp_execute_works_3() {
        let initializer = vec![2, 4, 4, 5, 99, 0];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.execute();

        assert_eq!(9801, intcomp.read_memory(5));
    }

    #[test]
    fn intcomp_execute_works_4() {
        let initializer = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.execute();

        assert_eq!(30, intcomp.read_memory(0));
    }

    #[test]
    fn intcomp_execute_works_5() {
        let initializer = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.execute();

        let mut output = Vec::new();

        while let Some(number) = intcomp.receive_output() {
            output.push(number);
        }

        assert_eq!(initializer, output);
    }

    #[test]
    fn intcomp_execute_works_6() {
        let initializer = vec![109, 1, 9, 2, 204, -6, 99];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.execute();

        assert_eq!(204, intcomp.receive_output().expect("no output available"));
    }

    #[test]
    fn intcomp_execute_works_7() {
        let initializer = vec![109, 1, 109, 9, 204, -6, 99];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.execute();

        assert_eq!(204, intcomp.receive_output().expect("no output available"));
    }

    #[test]
    fn intcomp_execute_works_8() {
        let initializer = vec![109, 1, 209, -1, 204, -106, 99];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.execute();

        assert_eq!(204, intcomp.receive_output().expect("no output available"));
    }

    #[test]
    fn intcomp_execute_works_9() {
        let initializer = vec![109, 1, 3, 3, 204, 2, 99];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.send_input(111);
        intcomp.execute();

        assert_eq!(111, intcomp.receive_output().expect("no output available"));
    }

    #[test]
    fn intcomp_execute_works_10() {
        let initializer = vec![109, 1, 203, 2, 204, 2, 99];
        let mut intcomp = Intcomp::new(&initializer);

        intcomp.send_input(222);
        intcomp.execute();

        assert_eq!(222, intcomp.receive_output().expect("no output available"));
    }

    #[test]
    fn intcomp_is_halted_detects_halt() {
        let initializer = vec![99];
        let intcomp = Intcomp::new(&initializer);

        assert_eq!(true, intcomp.is_halted());
    }

    #[test]
    fn intcomp_is_halted_detects_non_halt() {
        let initializer = vec![3, 0, 99];
        let intcomp = Intcomp::new(&initializer);

        assert_eq!(false, intcomp.is_halted());
    }

    #[test]
    fn operand_mode_from_digit_works() {
        assert_eq!(OperandMode::Position, OperandMode::from_digit(0));
        assert_eq!(OperandMode::Immediate, OperandMode::from_digit(1));
        assert_eq!(OperandMode::Relative, OperandMode::from_digit(2));
    }

    #[test]
    #[should_panic]
    fn operand_mode_from_digit_panics_on_unrecognized_digit() {
        OperandMode::from_digit(9);
    }

    #[test]
    fn get_digit_works() {
        assert_eq!(8, get_digit(56789, 2));
    }
}
