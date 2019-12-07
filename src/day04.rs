const START: u32 = 240_920;
const STOP: u32 = 789_857;

fn from_digits(digits: &[u32]) -> u32 {
    let mut total = 0;

    for digit in digits.iter() {
        total *= 10;
        total += digit;
    }

    total
}

fn ensure_ascending(candidate: u32) -> u32 {
    let mut digits = to_digits(candidate);

    for i in 1..digits.len() {
        if digits[i] < digits[i - 1] {
            for j in i..digits.len() {
                digits[j] = digits[i - 1];
            }

            break;
        }
    }

    from_digits(&digits)
}

fn has_pair(candidate: u32) -> bool {
    let digits = to_digits(candidate);

    for i in 0..(digits.len() - 1) {
        if digits[i] == digits[i + 1] {
            return true;
        }
    }

    false
}

fn has_strict_pair(candidate: u32) -> bool {
    let digits = to_digits(candidate);

    for i in 0..(digits.len() - 1) {
        if (i == 0 || digits[i] != digits[i - 1])
            && digits[i] == digits[i + 1]
            && (i == digits.len() - 2 || digits[i + 1] != digits[i + 2])
        {
            return true;
        }
    }

    false
}

fn to_digits(number: u32) -> Vec<u32> {
    let mut digits = Vec::new();
    let mut remaining = number;

    while remaining > 10 {
        digits.push(remaining % 10);
        remaining /= 10;
    }

    digits.push(remaining);
    digits.reverse();

    digits
}

pub fn part1() {
    let mut candidate = ensure_ascending(START);
    let mut count = 0;

    while candidate <= STOP {
        if has_pair(candidate) {
            count += 1;
        }

        candidate = ensure_ascending(candidate + 1);
    }

    println!("Possible passwords: {}", count);
}

pub fn part2() {
    let mut candidate = ensure_ascending(START);
    let mut count = 0;

    while candidate <= STOP {
        if has_strict_pair(candidate) {
            count += 1;
        }

        candidate = ensure_ascending(candidate + 1);
    }

    println!("Possible passwords: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_digits_works() {
        assert_eq!(1234, from_digits(&[1, 2, 3, 4,]));
    }

    #[test]
    fn ensure_ascending_works() {
        assert_eq!(1234, ensure_ascending(1234));
        assert_eq!(1244, ensure_ascending(1243));
        assert_eq!(1444, ensure_ascending(1423));
        assert_eq!(4444, ensure_ascending(4312));
        assert_eq!(2222, ensure_ascending(2143));
    }

    #[test]
    fn has_pair_works() {
        assert_eq!(false, has_pair(1234));
        assert_eq!(true, has_pair(1233));
        assert_eq!(true, has_pair(1223));
        assert_eq!(true, has_pair(1123));
        assert_eq!(true, has_pair(1133));
    }

    #[test]
    fn has_strict_pair_works() {
        assert_eq!(false, has_strict_pair(1234));
        assert_eq!(true, has_strict_pair(1233));
        assert_eq!(true, has_strict_pair(1223));
        assert_eq!(true, has_strict_pair(1123));
        assert_eq!(true, has_strict_pair(1133));
        assert_eq!(false, has_strict_pair(12333));
        assert_eq!(false, has_strict_pair(12223));
        assert_eq!(false, has_strict_pair(11123));
        assert_eq!(true, has_strict_pair(11333));
    }

    #[test]
    fn to_digits_works() {
        assert_eq!(vec![1, 2, 3, 4], to_digits(1234));
    }
}
