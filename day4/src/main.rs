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


fn main() {
    let num_passwords = (372037..=905157)
        .filter(|&n| valid_password(n))
        .count();

    println!("{}", num_passwords);
}

#[cfg(test)]
mod tests {
    use super::valid_password;

    #[test]
    fn example1() {
        assert!(valid_password(111111));
    }

    #[test]
    fn example2() {
        assert!(!valid_password(223450));

    }

    #[test]
    fn example3() {
        assert!(!valid_password(123789));
    }
}
