mod shape_list;

use crate::literals::messages;
use shape_list::*;

#[derive(Clone)]
pub enum FormElement {
    CheckBox(&'static str, bool),
    InputField(&'static str, String),
    FactorField(String),
    NoElement,
}

trait InnerImplShape {
    fn parse_input(&mut self, input_factor: f64) -> Result<(), &'static str>;
    fn get_area(&self) -> f64;
    fn get_result(&self, input_factor: f64, area: f64) -> String;
    fn state(&mut self) -> &mut [FormElement; 6];
    fn get_name(&self) -> &str;
}

trait AreaShapeClone: InnerImplShape {
    fn duplicate(&self) -> Box<dyn AreaShape>;
}

#[allow(private_bounds)]
pub trait AreaShape: AreaShapeClone {
    fn form_state(&mut self) -> &mut [FormElement; 6] {
        self.state()
    }

    fn name(&self) -> &str {
        self.get_name()
    }

    fn calculate(
        &mut self,
        input_factor: f64,
        output_factor: f64,
    ) -> Result<CalculationResult, &'static str> {
        self.parse_input(input_factor)?;
        let area = self.get_area() / output_factor;
        if !area.is_finite() {
            return Err(messages::CALCULATION_ERR);
        }
        let mut result = self.get_result(input_factor, area);
        localize(&mut result);
        Ok(CalculationResult {
            area,
            result,
            shape: self.duplicate(),
        })
    }

    fn update_result(&mut self, input_factor: f64, area: f64) -> String {
        let mut result = self.get_result(input_factor, area);
        localize(&mut result);
        result
    }
}

impl<T> AreaShapeClone for T
where
    T: 'static + AreaShape + Clone,
{
    fn duplicate(&self) -> Box<dyn AreaShape> {
        Box::new(self.clone())
    }
}

impl<T> AreaShape for T where T: InnerImplShape + Clone + 'static {}

pub fn get_shapes() -> Vec<Box<dyn AreaShape>> {
    vec![
        Box::<AreaCircle>::default(),
        Box::<AreaRectangle>::default(),
        Box::<AreaCylinder>::default(),
        Box::<AreaHexagon>::default(),
        Box::<AreaHexagonPrism>::default(),
        Box::<AreaBushing>::default(),
    ]
}

pub struct CalculationResult {
    area: f64,
    result: String,
    shape: Box<dyn AreaShape>,
}

impl CalculationResult {
    pub fn get_result(&self) -> &str {
        self.result.as_str()
    }

    pub fn get_area(&self) -> f64 {
        self.area
    }

    pub fn get_state(&mut self) -> &mut Box<dyn AreaShape> {
        &mut self.shape
    }

    pub fn scale_area(&mut self, factor: f64) {
        self.area *= factor;
    }

    pub fn update_result(&mut self, input_factor: f64) {
        let area = self.area;
        self.result = self.get_state().update_result(input_factor, area);
    }
}

const COMMA: u8 = 44;
const DOT: u8 = 46;

pub fn localize(result: &mut String) {
    if cfg!(feature = "lang_rus") {
        unsafe {
            let mut digit = false;
            result.as_mut_vec().iter_mut().for_each(|ch| {
                if *ch == DOT && digit {
                    *ch = COMMA;
                } else {
                    digit = ch.is_ascii_digit();
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::COMMA;
    use super::DOT;

    #[test]
    fn test_dot() {
        let dot: u8 = '.'.try_into().unwrap();
        assert_eq!(dot, DOT)
    }

    #[test]
    fn test_comma() {
        let comma: u8 = ','.try_into().unwrap();
        assert_eq!(comma, COMMA);
    }
}
