use super::FormElement;
use super::parser::split_numbers;

pub fn get_number(field: &FormElement) -> Option<f64>{
    let x = match field {
        FormElement::InputField(_, x) => {//ref?
            split_numbers(x.as_str())
        },
        _ => return None
    };
    x
}

pub fn get_factor(field: &FormElement) -> Option<f64>{
    let x = match field {
        FormElement::FactorField(x) => {//ref?
            if x.is_empty() {
                Some(1.0)
            } else {
                split_numbers(x.as_str())
            }
        },
        _ => return None
    };
    x
}

pub fn std_validate_state (state: &mut [FormElement; 6]) {
    for field in state {
        match field {
            FormElement::InputField(_, input ) 
            | FormElement::FactorField(input) => {
                    validate(input);
            }
            _ => {}
        }
    }
}

fn validate (input: &mut String) {
    let mut has_decimator = false;
    let mut has_digits = false;
    let mut has_sign = false;
    input.retain(|letter| {
        if letter.is_digit(10) {
            has_sign = false;
            has_digits = true;
            return true;
        } else if letter == ',' || letter == '.' {
            if !has_decimator {
                return true;
            } else {
                has_decimator = true;
                return false;
            }
        }else if letter == ' '{
            has_decimator = false;
            has_digits = false;
            return true;
        }else if letter == '+' || letter == '-'{
            if has_sign {
                return false;
            }
            has_sign = true;
            has_decimator = false;
            return true;
        } else {
            return false;
        }
    })                               
}

#[cfg(test)]
mod tests {
    use super::validate;

    #[test]
    fn test_validator_1 () {
        let mut input = String::from("4 ++5");
        validate(&mut input);
        assert_eq!(String::from("4 +5"), input);
    }
    #[test]
    fn test_validator_2 () {
        let mut input = String::from("4 -,+5");
        validate(&mut input);
        assert_eq!(String::from("4 -,5"), input);
    }
    #[test]
    fn test_validator_3 () {
        let mut input = String::from("-4j -,+5");
        validate(&mut input);
        assert_eq!(String::from("-4 -,5"), input);
    }
    #[test]
    fn test_validator_4 () {
        let mut input = String::from("-4,5");
        validate(&mut input);
        assert_eq!(String::from("-4,5"), input);
    }
}

