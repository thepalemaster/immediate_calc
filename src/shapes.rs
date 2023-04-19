mod shape_list;

use shape_list::*;

#[derive(Clone)]
pub enum FormElement {
    CheckBox(&'static str, bool),
    InputField(&'static str, String),
    FactorField(String),
    NoElement
}

pub trait AreaShapeClone {
    fn duplicate(&self) -> Box<dyn AreaShape>;
}

pub trait AreaShape: AreaShapeClone {
    fn form_state(&mut self) -> &mut [FormElement; 6];
    fn name(&self) -> & str;
    fn calculate(&self, input_factor: f64, output_factor: f64) -> Option<CalculationResult>;
}

impl<T> AreaShapeClone for T 
    where T: 'static + AreaShape + Clone
{
    fn duplicate(&self) -> Box<dyn AreaShape> {
        Box::new(self.clone())
    }
}

pub fn get_shapes () -> Vec<Box<dyn AreaShape>> {
    vec![
        Box::new(AreaCircle::default()),
        Box::new(AreaRectangle::default()),
        Box::new(AreaCylinder::default()),
        Box::new(AreaHexagon::default()),
        Box::new(AreaHexagonPrism::default()),
        Box::new(AreaBushing::default()),
    ]
}

pub struct CalculationResult {
    area: f64,
    pub result: String,
    shape: Box<dyn AreaShape> 
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

    pub fn name(&self) -> &str {
        self.shape.name()
    }

    pub fn recalculate(&mut self, input_factor: f64, output_factor: f64) {
        let result = self.shape.calculate(input_factor, output_factor);
        if result.is_some() {
            let some_result = result.unwrap();
            self.area = some_result.get_area();
            self.result = some_result.result;
        }
    }
}
