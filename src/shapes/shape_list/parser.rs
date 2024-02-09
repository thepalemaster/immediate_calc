use crate::literals::messages;
use core::f64;

pub fn split_numbers(input: &str) -> Result<f64, &'static str> {
    let mut prepared_input = input.trim_start();
    let mut positive = if prepared_input.starts_with('-') {
        prepared_input = prepared_input.strip_prefix('-').ok_or("parse_error")?;
        false
    } else {
        true
    };
    let numbers = prepared_input.split_inclusive(is_sign);
    let mut sum = 0.;
    for num in numbers {
        let next_positive = !num.ends_with('-');
        let trimmed_num = num.trim_end_matches(is_sign).trim();
        let mut float_number;
        if trimmed_num.starts_with(decimal_separator) {
            float_number = get_fractional(trimmed_num.trim_start_matches(decimal_separator))?;
        } else {
            let mut float_parts = trimmed_num.splitn(2, decimal_separator);
            float_number = float_parts
                .next()
                .ok_or(messages::INPUT_WRONG)?
                .parse::<i32>()
                .or(Err(messages::PARSE_WRONG))? as f64;
            if let Some(val) = float_parts.next() {
                float_number += get_fractional(val)?;
            }
        }
        if !positive {
            float_number *= -1.0;
        }
        positive = next_positive;
        sum += float_number;
    }
    Ok(sum)
}

fn get_fractional(input: &str) -> Result<f64, &'static str> {
    let len_fractional = input.len();
    if len_fractional > 0 {
        let denominator = 10.0_f64.powi(len_fractional as i32);
        return match input.parse::<i64>() {
            Ok(num) => Ok(num as f64 / denominator),
            Err(_) => Err(messages::PARSE_WRONG),
        };
    }
    Ok(0.)
}

pub fn decimal_separator(letter: char) -> bool {
    letter == '.' || letter == ','
}

pub fn is_sign(letter: char) -> bool {
    letter == '+' || letter == '-'
}

#[cfg(test)]
mod tests {
    use crate::literals::messages;

    use super::split_numbers;

    #[test]
    fn test_parcer_1() {
        assert_eq!(split_numbers("43"), Ok(43.0));
    }
    #[test]
    fn test_parcer_2() {
        assert_eq!(split_numbers("43.5"), Ok(43.5));
    }
    #[test]
    fn test_parcer_3() {
        assert_eq!(split_numbers("4+3"), Ok(7.0));
    }
    #[test]
    fn test_parcer_4() {
        assert_eq!(split_numbers("4 - 3"), Ok(1.0));
    }
    #[test]
    fn test_parcer_5() {
        assert_eq!(split_numbers(" -4.0 + 3,0"), Ok(-1.0));
    }
    #[test]
    fn test_parcer_6() {
        assert_eq!(split_numbers("12,3 - 4.0 + 3,0"), Ok(11.3));
    }
    #[test]
    fn test_parcer_7() {
        assert_eq!(split_numbers("12,3  4.0"), Err(messages::PARSE_WRONG));
    }
    #[test]
    fn test_parcer_8() {
        assert_eq!(split_numbers("-- 2,3 - 7"), Err(messages::PARSE_WRONG));
    }
    #[test]
    fn test_parcer_9() {
        assert_eq!(split_numbers(",3  +.7- ,5"), Ok(0.5));
    }
    #[test]
    fn test_parcer_10() {
        assert_eq!(split_numbers("0,3 + 1..7"), Err(messages::PARSE_WRONG));
    }

    #[test]
    fn test_parcer_11() {
        assert_eq!(split_numbers("0,,7"), Err(messages::PARSE_WRONG));
    }
}
