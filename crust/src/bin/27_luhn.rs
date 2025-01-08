pub fn luhn(cc_number: &str) -> bool {
    let mut sum = 0;
    let mut double = false;
    let mut n_digits: usize = 0;

    for c in cc_number.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            n_digits += 1;
            if double {
                let double_digit = digit * 2;
                sum += if double_digit > 9 {
                    double_digit - 9
                } else {
                    double_digit
                };
            } else {
                sum += digit;
            }
            double = !double;
        } else if c.is_whitespace() {
            continue;
        } else {
            return false;
        }
    }

    n_digits >= 2 && sum % 10 == 0
}

pub fn luhn_alt(cc_number: &str) -> bool {
    let digits: Vec<u32> = cc_number
        .chars()
        .filter(|&c| !c.is_whitespace())
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<_>>>()
        .unwrap_or_default();

    if digits.len() < 2 {
        return false;
    }

    digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| {
            if i % 2 == 1 {
                let doubled = d * 2;
                if doubled > 9 {
                    doubled - 9
                } else {
                    doubled
                }
            } else {
                d
            }
        })
        .sum::<u32>()
        % 10
        == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
    }

    #[test]
    fn test_single_digit_cc_number() {
        assert!(!luhn("0"));
        assert!(!luhn("5"));
    }

    #[test]
    fn test_two_digit_cc_number() {
        assert!(luhn("0 0"));
    }

    #[test]
    fn test_non_digit_cc_number() {
        assert!(!luhn("King"));
        assert!(!luhn("King 4263 9826 4026 9299"));
    }

    #[test]
    fn test_empty_cc_number() {
        assert!(!luhn(""));
        assert!(!luhn(" "));
        assert!(!luhn("  "));
        assert!(!luhn("   "));
        assert!(!luhn("    "));
    }
}
