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
