use crate::literals::messages;

use super::parser::{decimal_separator, is_sign, split_numbers};
use super::FormElement;

pub fn get_lenght(
    field: &FormElement,
    factor: f64,
    negative: &mut bool,
) -> Result<f64, &'static str> {
    let x = match field {
        FormElement::InputField(_, x) => split_numbers(x.as_str()),
        _ => return Err(messages::WRONG_FIELD),
    };
    x.map(|number| {
        if number < 0. {
            *negative = true;
            number.abs() * factor
        } else {
            number * factor
        }
    })
}

pub fn get_factor(field: &FormElement, negative: bool) -> Result<f64, &'static str> {
    match field {
        FormElement::FactorField(x) => {
            let tmp = if x.is_empty() {
                Ok(1.0)
            } else {
                split_numbers(x.as_str())
            };
            tmp.map(|x| {
                if negative && x.is_sign_positive() {
                    x * -1.0
                } else {
                    x
                }
            })
        }
        _ => Err(messages::WRONG_FIELD),
    }
}

pub fn get_option(field: &FormElement) -> Result<bool, &'static str> {
    match field {
        FormElement::CheckBox(_, option) => Ok(*option),
        _ => Err(messages::WRONG_FIELD),
    }
}

pub fn std_validate_state(state: &mut [FormElement; 6]) {
    for field in state {
        match field {
            FormElement::InputField(_, input) | FormElement::FactorField(input) => {
                validate(input);
            }
            _ => {}
        }
    }
}

fn validate(input: &mut String) {
    let mut has_decimator = false;
    let mut has_digits = false;
    let mut has_sign = false;
    input.retain(|letter| {
        if letter.is_ascii_digit() {
            has_sign = false;
            has_digits = true;
            true
        } else if decimal_separator(letter) {
            if !has_decimator {
                has_decimator = true;
                true
            } else {
                false
            }
        } else if letter == ' ' {
            has_decimator = false;
            has_digits = false;
            true
        } else if is_sign(letter) {
            if has_sign {
                false
            } else {
                has_sign = true;
                has_decimator = false;
                true
            }
        } else {
            false
        }
    })
}

#[cfg(test)]
mod tests {
    use super::validate;

    #[test]
    fn test_validator_1() {
        let mut input = String::from("4 ++5");
        validate(&mut input);
        assert_eq!(String::from("4 +5"), input);
    }
    #[test]
    fn test_validator_2() {
        let mut input = String::from("4 -,+5");
        validate(&mut input);
        assert_eq!(String::from("4 -,5"), input);
    }
    #[test]
    fn test_validator_3() {
        let mut input = String::from("-4j -,+5");
        validate(&mut input);
        assert_eq!(String::from("-4 -,5"), input);
    }
    #[test]
    fn test_validator_4() {
        let mut input = String::from("-4,5");
        validate(&mut input);
        assert_eq!(String::from("-4,5"), input);
    }

    #[test]
    fn test_validator_5() {
        let mut input = String::from("-4,,");
        validate(&mut input);
        assert_eq!(String::from("-4,"), input);
    }
}
