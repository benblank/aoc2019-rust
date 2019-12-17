use std::fs;

const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];
const INPUT_PATH: &str = "day16.input.txt";

fn get_multiplier(scale: usize, offset: usize) -> i32 {
    BASE_PATTERN[((offset + 1) / (scale + 1) % 4)]
}

fn run_phase(input: &[u32]) -> Vec<u32> {
    let mut output = Vec::new();

    for output_index in 0..input.len() {
        let mut total = 0;

        for (input_index, digit) in input.iter().enumerate() {
            total += *digit as i32 * get_multiplier(output_index, input_index);
        }

        output.push(i32::abs(total) as u32 % 10);
    }

    output
}

pub fn part1() {
    let mut digits = fs::read(INPUT_PATH)
        .expect("could not read input file")
        .iter()
        .map(|digit| (*digit as char).to_digit(10).expect("bad digit"))
        .collect::<Vec<_>>();

    for _ in 0..100 {
        digits = run_phase(&digits);
    }

    digits.truncate(8);

    println!(
        "First 8 digits: {}",
        digits
            .into_iter()
            .map(|digit| digit.to_string())
            .collect::<String>()
    );
}

pub fn part2() {
    let mut digits = fs::read(INPUT_PATH)
        .expect("could not read input file")
        .iter()
        .map(|digit| (*digit as char).to_digit(10).expect("bad digit"))
        .collect::<Vec<_>>();

    let offset = digits[..7]
        .to_owned()
        .iter()
        .fold(0, |offset, digit| offset * 10 + digit) as usize;

    let mut chain = Vec::new();

    for _ in 0..10_000 {
        chain.push(digits.to_owned());
    }

    // Yes, this is totally inefficient.  But it's still fast enough.
    digits = chain.concat()[offset..].to_owned();

    for _ in 0..100 {
        for i in (0..digits.len() - 1).rev() {
            digits[i] = (digits[i] + digits[i + 1]) % 10;
        }
    }

    let message = digits[..8]
        .iter()
        .fold(0, |message, digit| message * 10 + digit);

    println!("Message: {}", message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_multiplier_works_0() {
        assert_eq!(1, get_multiplier(0, 0));
        assert_eq!(0, get_multiplier(0, 1));
        assert_eq!(-1, get_multiplier(0, 2));
        assert_eq!(0, get_multiplier(0, 3));
        assert_eq!(1, get_multiplier(0, 4));
        assert_eq!(0, get_multiplier(0, 5));
        assert_eq!(-1, get_multiplier(0, 6));
        assert_eq!(0, get_multiplier(0, 7));
    }

    #[test]
    fn get_multiplier_works_1() {
        assert_eq!(0, get_multiplier(1, 0));
        assert_eq!(1, get_multiplier(1, 1));
        assert_eq!(1, get_multiplier(1, 2));
        assert_eq!(0, get_multiplier(1, 3));
        assert_eq!(0, get_multiplier(1, 4));
        assert_eq!(-1, get_multiplier(1, 5));
        assert_eq!(-1, get_multiplier(1, 6));
        assert_eq!(0, get_multiplier(1, 7));
    }

    #[test]
    fn get_multiplier_works_2() {
        assert_eq!(0, get_multiplier(2, 0));
        assert_eq!(0, get_multiplier(2, 1));
        assert_eq!(1, get_multiplier(2, 2));
        assert_eq!(1, get_multiplier(2, 3));
        assert_eq!(1, get_multiplier(2, 4));
        assert_eq!(0, get_multiplier(2, 5));
        assert_eq!(0, get_multiplier(2, 6));
        assert_eq!(0, get_multiplier(2, 7));
    }

    #[test]
    fn get_multiplier_works_3() {
        assert_eq!(0, get_multiplier(3, 0));
        assert_eq!(0, get_multiplier(3, 1));
        assert_eq!(0, get_multiplier(3, 2));
        assert_eq!(1, get_multiplier(3, 3));
        assert_eq!(1, get_multiplier(3, 4));
        assert_eq!(1, get_multiplier(3, 5));
        assert_eq!(1, get_multiplier(3, 6));
        assert_eq!(0, get_multiplier(3, 7));
    }

    #[test]
    fn run_phase_works_1() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let output = vec![4, 8, 2, 2, 6, 1, 5, 8];

        assert_eq!(output, run_phase(&input));
    }

    #[test]
    fn run_phase_works_2() {
        let input = vec![4, 8, 2, 2, 6, 1, 5, 8];
        let output = vec![3, 4, 0, 4, 0, 4, 3, 8];

        assert_eq!(output, run_phase(&input));
    }

    #[test]
    fn run_phase_works_3() {
        let input = vec![3, 4, 0, 4, 0, 4, 3, 8];
        let output = vec![0, 3, 4, 1, 5, 5, 1, 8];

        assert_eq!(output, run_phase(&input));
    }

    #[test]
    fn run_phase_works_4() {
        let input = vec![0, 3, 4, 1, 5, 5, 1, 8];
        let output = vec![0, 1, 0, 2, 9, 4, 9, 8];

        assert_eq!(output, run_phase(&input));
    }
}
