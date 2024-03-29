use crate::intcomp::{read_program, Intcomp};
use permutohedron::Heap;

const INPUT_PATH: &str = "day07.input.txt";

pub fn part1() {
    let initializer = read_program(INPUT_PATH);
    let mut max_signal = -1;

    for phase_settings in Heap::new(&mut vec![0, 1, 2, 3, 4]) {
        let mut signal = 0;

        for phase_setting in phase_settings {
            let mut intcomp = Intcomp::new(&initializer);

            intcomp.send_input(phase_setting);
            intcomp.send_input(signal);
            intcomp.execute();

            signal = intcomp.receive_output().expect("no output available");
        }

        if signal > max_signal {
            max_signal = signal;
        }
    }

    println!("Max thruster signal: {}", max_signal);
}

pub fn part2() {
    let initializer = read_program(INPUT_PATH);
    let mut max_signal = -1;

    for phase_settings in Heap::new(&mut vec![5, 6, 7, 8, 9]) {
        let mut intcomps = (0..5)
            .map(|i| {
                let mut intcomp = Intcomp::new(&initializer);

                intcomp.send_input(phase_settings[i]);

                intcomp
            })
            .collect::<Vec<_>>();

        let mut signal = 0;

        while !intcomps[0].is_halted() {
            for intcomp in &mut intcomps {
                intcomp.send_input(signal);
                intcomp.execute();

                signal = intcomp.receive_output().expect("no output available");
            }
        }

        if signal > max_signal {
            max_signal = signal;
        }
    }

    println!("Max thruster signal: {}", max_signal);
}
