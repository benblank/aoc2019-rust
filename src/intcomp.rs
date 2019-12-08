use std::io::{stdin, stdout, Write};

#[derive(Debug, PartialEq)]
enum AddressType {
    Address,
    Immediate,
}

impl AddressType {
    fn from_digit(digit: i32) -> AddressType {
        match digit {
            0 => AddressType::Address,
            1 => AddressType::Immediate,
            _ => panic!("Invalid address type! ({})", digit),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Add(AddressType, AddressType, AddressType),
    Multiply(AddressType, AddressType, AddressType),
    Input(AddressType),
    Output(AddressType),
    JumpIfTrue(AddressType, AddressType),
    JumpIfFalse(AddressType, AddressType),
    LessThan(AddressType, AddressType, AddressType),
    Equals(AddressType, AddressType, AddressType),
    Halt,
}

impl Instruction {
    fn parse(instruction: i32) -> Instruction {
        let opcode = instruction % 100;

        match opcode {
            1 => Instruction::Add(
                AddressType::from_digit(get_digit(instruction, 3)),
                AddressType::from_digit(get_digit(instruction, 4)),
                AddressType::Address,
            ),

            2 => Instruction::Multiply(
                AddressType::from_digit(get_digit(instruction, 3)),
                AddressType::from_digit(get_digit(instruction, 4)),
                AddressType::Address,
            ),

            3 => Instruction::Input(AddressType::Address),
            4 => Instruction::Output(AddressType::from_digit(get_digit(instruction, 3))),

            5 => Instruction::JumpIfTrue(
                AddressType::from_digit(get_digit(instruction, 3)),
                AddressType::from_digit(get_digit(instruction, 4)),
            ),

            6 => Instruction::JumpIfFalse(
                AddressType::from_digit(get_digit(instruction, 3)),
                AddressType::from_digit(get_digit(instruction, 4)),
            ),

            7 => Instruction::LessThan(
                AddressType::from_digit(get_digit(instruction, 3)),
                AddressType::from_digit(get_digit(instruction, 4)),
                AddressType::Address,
            ),

            8 => Instruction::Equals(
                AddressType::from_digit(get_digit(instruction, 3)),
                AddressType::from_digit(get_digit(instruction, 4)),
                AddressType::Address,
            ),

            99 => Instruction::Halt,
            _ => panic!("Invalid opcode! ({})", opcode),
        }
    }
}

pub struct Intcomp {
    memory: Vec<i32>,
    ip: usize,
}

impl Intcomp {
    pub fn execute(&mut self) {
        loop {
            let instruction = Instruction::parse(self.memory[self.ip]);

            match instruction {
                Instruction::Add(operand1_type, operand2_type, _) => {
                    let operand1 = match operand1_type {
                        AddressType::Address => self.memory[self.memory[self.ip + 1] as usize],
                        AddressType::Immediate => self.memory[self.ip + 1],
                    };

                    let operand2 = match operand2_type {
                        AddressType::Address => self.memory[self.memory[self.ip + 2] as usize],
                        AddressType::Immediate => self.memory[self.ip + 2],
                    };

                    let target = self.memory[self.ip + 3] as usize;

                    self.memory[target] = operand1 + operand2;
                    self.ip += 4;
                }

                Instruction::Multiply(operand1_type, operand2_type, _) => {
                    let operand1 = match operand1_type {
                        AddressType::Address => self.memory[self.memory[self.ip + 1] as usize],
                        AddressType::Immediate => self.memory[self.ip + 1],
                    };

                    let operand2 = match operand2_type {
                        AddressType::Address => self.memory[self.memory[self.ip + 2] as usize],
                        AddressType::Immediate => self.memory[self.ip + 2],
                    };

                    let target = self.memory[self.ip + 3] as usize;

                    self.memory[target] = operand1 * operand2;
                    self.ip += 4;
                }

                Instruction::Input(_) => {
                    let target = self.memory[self.ip + 1] as usize;
                    let mut input = String::new();

                    print!(": ");
                    stdout().flush().unwrap();

                    stdin()
                        .read_line(&mut input)
                        .expect("failed to read from stdin");

                    self.memory[target] = input.trim().parse::<i32>().unwrap();
                    self.ip += 2;
                }

                Instruction::Output(operand_type) => {
                    let operand = match operand_type {
                        AddressType::Address => self.memory[self.memory[self.ip + 1] as usize],
                        AddressType::Immediate => self.memory[self.ip + 1],
                    };

                    println!("{}", operand);

                    self.ip += 2;
                }

                Instruction::JumpIfTrue(operand_type, jump_to_type) => {
                    let operand = match operand_type {
                        AddressType::Address => self.memory[self.memory[self.ip + 1] as usize],
                        AddressType::Immediate => self.memory[self.ip + 1],
                    };

                    let jump_to = match jump_to_type {
                        AddressType::Address => self.memory[self.memory[self.ip + 2] as usize],
                        AddressType::Immediate => self.memory[self.ip + 2],
                    } as usize;

                    if operand != 0 {
                        self.ip = jump_to;
                    } else {
                        self.ip += 3;
                    }
                }

                Instruction::JumpIfFalse(operand_type, jump_to_type) => {
                    let operand = match operand_type {
                        AddressType::Address => self.memory[self.memory[self.ip + 1] as usize],
                        AddressType::Immediate => self.memory[self.ip + 1],
                    };

                    let jump_to = match jump_to_type {
                        AddressType::Address => self.memory[self.memory[self.ip + 2] as usize],
                        AddressType::Immediate => self.memory[self.ip + 2],
                    } as usize;

                    if operand == 0 {
                        self.ip = jump_to;
                    } else {
                        self.ip += 3;
                    }
                }

                Instruction::LessThan(operand1_type, operand2_type, _) => {
                    let operand1 = match operand1_type {
                        AddressType::Address => self.memory[self.memory[self.ip + 1] as usize],
                        AddressType::Immediate => self.memory[self.ip + 1],
                    };

                    let operand2 = match operand2_type {
                        AddressType::Address => self.memory[self.memory[self.ip + 2] as usize],
                        AddressType::Immediate => self.memory[self.ip + 2],
                    };

                    let target = self.memory[self.ip + 3] as usize;

                    self.memory[target] = if operand1 < operand2 { 1 } else { 0 };
                    self.ip += 4;
                }

                Instruction::Equals(operand1_type, operand2_type, _) => {
                    let operand1 = match operand1_type {
                        AddressType::Address => self.memory[self.memory[self.ip + 1] as usize],
                        AddressType::Immediate => self.memory[self.ip + 1],
                    };

                    let operand2 = match operand2_type {
                        AddressType::Address => self.memory[self.memory[self.ip + 2] as usize],
                        AddressType::Immediate => self.memory[self.ip + 2],
                    };

                    let target = self.memory[self.ip + 3] as usize;

                    self.memory[target] = if operand1 == operand2 { 1 } else { 0 };
                    self.ip += 4;
                }

                Instruction::Halt => return,
            }
        }
    }

    pub fn new(intitial_memory: &[i32]) -> Intcomp {
        Intcomp {
            memory: intitial_memory.to_vec(),
            ip: 0,
        }
    }

    pub fn read_memory(&self, address: usize) -> i32 {
        self.memory[address]
    }

    pub fn write_memory(&mut self, address: usize, value: i32) {
        self.memory[address] = value;
    }
}

/// Get the nth digit from the right.
fn get_digit(number: i32, digit: u8) -> i32 {
    let base = i32::pow(10, digit as u32 - 1);

    (number / base) % 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_type_from_digit_works() {
        assert_eq!(AddressType::Address, AddressType::from_digit(0));
        assert_eq!(AddressType::Immediate, AddressType::from_digit(1));
    }

    #[test]
    #[should_panic]
    fn address_type_from_digit_panics_on_unrecognized_digit() {
        AddressType::from_digit(9);
    }

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
            Instruction::Add(AddressType::Address, _, _) => {}
            _ => panic!("expected Address"),
        }
    }

    #[test]
    fn instruction_parse_supports_immediate() {
        match Instruction::parse(101) {
            Instruction::Add(AddressType::Immediate, _, _) => {}
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
    fn get_digit_works() {
        assert_eq!(8, get_digit(56789, 2));
    }
}
