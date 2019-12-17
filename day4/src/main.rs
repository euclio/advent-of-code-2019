fn digits(mut n: u32) -> Vec<u32> {
    let mut digits = Vec::with_capacity(6);
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.reverse();
    digits
}

fn valid_password(n: u32) -> bool {
    let digits = digits(n);

    let mut has_double = false;

    for window in digits.windows(2) {
        let first = window[0];
        let second = window[1];

        if second < first {
            return false;
        }

        if first == second {
            has_double = true;
        }

    }

    has_double
}

fn contains_run_of_exactly_two(n: u32) -> bool {
    let digits = digits(n);

    let mut last_digit = digits[0];
    let mut chain_len = 1;

    for &digit in &digits[1..] {
        if last_digit == digit {
            chain_len += 1;
        } else if chain_len == 2 {
            return true;
        } else {
            chain_len = 1;
        }

        last_digit = digit;
    }

    return chain_len == 2;
}

fn main() {
    let num_passwords = (372037..=905157)
        .filter(|&n| valid_password(n))
        .count();

    println!("part 1: {}", num_passwords);

    let num_passwords = (372037..=905157)
        .filter(|&n| valid_password(n) && contains_run_of_exactly_two(n))
        .count();

    println!("part 2: {}", num_passwords);
}

#[cfg(test)]
mod tests {
    use super::{contains_run_of_exactly_two, valid_password};

    #[test]
    fn part1_example1() {
        assert!(valid_password(111111));
    }

    #[test]
    fn part1_example2() {
        assert!(!valid_password(223450));

    }

    #[test]
    fn part1_example3() {
        assert!(!valid_password(123789));
    }

    #[test]
    fn part2_example1() {
        assert!(contains_run_of_exactly_two(112233));
    }

    #[test]
    fn part2_example2() {
        assert!(!contains_run_of_exactly_two(123444));
    }

    #[test]
    fn part2_example3() {
        assert!(contains_run_of_exactly_two(111122));
    }
}
