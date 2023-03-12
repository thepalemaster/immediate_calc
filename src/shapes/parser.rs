pub fn split_numbers (input: &str) ->Option<f64> {
    let mut prepared_input = input.trim_start();
    let mut positive = if prepared_input.starts_with('-'){
        prepared_input = prepared_input.strip_prefix('-')?;
        false
    } else {
        true
    };
    let numbers = prepared_input.split_inclusive(is_sign);
    let mut sum = 0.;
    for num in numbers {
        let mut next_positive = true;
        let trimmed_num;
        if num.ends_with("-") {
            next_positive = false;
        }
        trimmed_num = num.trim_end_matches(is_sign).trim();
        let mut float_number;
        if trimmed_num.starts_with(decimal_separator) {
            float_number = get_fractional(trimmed_num.trim_start_matches(decimal_separator))?;
        } else {
            let mut float_parts = trimmed_num.splitn(2, decimal_separator);
            float_number = float_parts.next()?.parse::<i32>().ok()? as f64;
            let str_fractional = float_parts.next();
            if str_fractional.is_some() {
                float_number += str_fractional.and_then(|input| get_fractional(input))?;
            }
        }
        if !positive {
            float_number *= -1.0;
        }
        positive = next_positive;
        sum += float_number;
    }
    Some(sum)
}

fn get_fractional(input: &str) -> Option<f64> {
    let len_fractional = input.len();
    if len_fractional > 0 {
        let denominator = 10.0_f64.powi(len_fractional.try_into().ok()?);
        let fractional = input.parse::<i32>().ok()? as f64 / denominator;
        return Some(fractional);
    }
    return Some(0.0)
}

fn decimal_separator(letter: char) -> bool {
    letter == '.' || letter == ','
}

fn is_sign(letter: char) -> bool {
    letter == '+' || letter == '-'
}


#[cfg(test)]
mod tests {
    use super::split_numbers;

    #[test]
    fn test_parcer_1() {
        assert_eq!(split_numbers("43"), Some(43.0));
    }
    #[test]
    fn test_parcer_2() {
        assert_eq!(split_numbers("43.5"), Some(43.5));
    }
    #[test]
    fn test_parcer_3() {
        assert_eq!(split_numbers("4+3"), Some(7.0));
    }
    #[test]
    fn test_parcer_4() {
        assert_eq!(split_numbers("4 - 3"), Some(1.0));
    }
    #[test]
    fn test_parcer_5() {
        assert_eq!(split_numbers(" -4.0 + 3,0"), Some(-1.0));
    }
    #[test]
    fn test_parcer_6() {
        assert_eq!(split_numbers("12,3 - 4.0 + 3,0"), Some(11.3));
    }
    #[test]
    fn test_parcer_7() {
        assert_eq!(split_numbers("12,3  4.0"), None);
    }
    #[test]
    fn test_parcer_8() {
        assert_eq!(split_numbers("-- 2,3 - 7"), None);
    }
    #[test]
    fn test_parcer_9() {
        assert_eq!(split_numbers(",3  +.7- ,5"), Some(0.5));
    }
    #[test]
    fn test_parcer_10() {
        assert_eq!(split_numbers("0,3 + 1..7"), None);
    }
}
