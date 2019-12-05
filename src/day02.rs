use std::cmp;
use std::io::{self, BufRead};
use std::str;

fn execute(memory: &mut Vec<i32>) {
    let mut ip = 0;

    loop {
        if ip >= memory.len() {
            panic!(
                "Instruction pointer ran off the end of memory! ({} >= {})",
                ip,
                memory.len()
            );
        }

        let opcode = memory[ip];

        if opcode == 99 {
            return;
        }

        while ip + 3 >= memory.len() {
            memory.push(0);
        }

        let source1 = memory[ip + 1] as usize;
        let source2 = memory[ip + 2] as usize;
        let target = memory[ip + 3] as usize;
        let result;

        while cmp::max(source1, source2) >= memory.len() {
            memory.push(0);
        }

        if opcode == 1 {
            result = memory[source1] + memory[source2];
        } else if opcode == 2 {
            result = memory[source1] * memory[source2];
        } else {
            panic!("Invalid opcode! ({})", opcode);
        }

        while target >= memory.len() {
            memory.push(0);
        }

        memory[target] = result;
        ip += 4; // skip past opcode and all three operands
    }
}

pub fn part1() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut memory = handle
        .split(b',')
        .map(|number| {
            str::from_utf8(&number.unwrap())
                .unwrap()
                .parse::<i32>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    memory[1] = 12;
    memory[2] = 2;

    execute(&mut memory);

    println!("Value at position 0: {}", memory[0]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_halts() {
        let mut memory = vec![99];

        execute(&mut memory);
    }

    #[test]
    fn execute_can_add() {
        let mut memory = vec![1, 0, 0, 0, 99];

        execute(&mut memory);

        assert_eq!(2, memory[0]);
    }

    #[test]
    fn execute_can_multiply() {
        let mut memory = vec![2, 0, 0, 0, 99];

        execute(&mut memory);

        assert_eq!(4, memory[0]);
    }

    #[test]
    #[should_panic]
    fn execute_panics_on_unrecognized_opcode() {
        let mut memory = vec![3, 0, 0, 0, 99];

        execute(&mut memory);
    }

    #[test]
    fn execute_extends_memory_for_operand1() {
        let mut memory = vec![1, 5, 0, 0, 99];

        execute(&mut memory);

        assert_eq!(6, memory.len());
    }

    #[test]
    fn execute_extends_memory_for_operand2() {
        let mut memory = vec![1, 0, 5, 0, 99];

        execute(&mut memory);

        assert_eq!(6, memory.len());
    }

    #[test]
    fn execute_extends_memory_for_operand3() {
        let mut memory = vec![1, 0, 0, 5, 99];

        execute(&mut memory);

        assert_eq!(6, memory.len());
    }

    #[test]
    fn execute_works_1() {
        let mut memory = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        execute(&mut memory);

        assert_eq!(3500, memory[0]);
    }

    #[test]
    fn execute_works_2() {
        let mut memory = vec![2, 3, 0, 3, 99];

        execute(&mut memory);

        assert_eq!(6, memory[3]);
    }

    #[test]
    fn execute_works_3() {
        let mut memory = vec![2, 4, 4, 5, 99, 0];

        execute(&mut memory);

        assert_eq!(9801, memory[5]);
    }

    #[test]
    fn execute_works_4() {
        let mut memory = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];

        execute(&mut memory);

        assert_eq!(30, memory[0]);
    }
}
