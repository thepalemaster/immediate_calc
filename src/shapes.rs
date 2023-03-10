mod helpers;
mod parser;

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
    fn calculate(&self) -> Option<CalculationResult>;
}

impl<T> AreaShapeClone for T 
    where T: 'static + AreaShape + Clone
{
    fn duplicate(&self) -> Box<dyn AreaShape> {
        Box::new(self.clone())
    }
}

pub struct CalculationResult {
    area: f64,
    result: String,
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
}


#[derive(Clone)]
pub struct AreaCircle {
    state: [FormElement; 6]
}

impl Default for AreaCircle {
    fn default()-> Self {
        Self {
            state: [
                FormElement::InputField("Диаметр", String::new()),
                FormElement::FactorField(String::new()),
                FormElement::NoElement,
                FormElement::NoElement,
                FormElement::NoElement,
                FormElement::NoElement,
            ]
        }
    }
}

impl AreaShape for AreaCircle {
    fn form_state(&mut self) -> &mut [FormElement; 6] {
        &mut self.state
    }
    fn name (&self) -> & str{
        "Круг"
    }
    fn calculate(&self) -> Option<CalculationResult> {
        let d = helpers::get_number(&self.state[0])?;
        let f = helpers::get_factor(&self.state[1])?;
        let area = f * std::f64::consts::PI * d * d / 4.0;
        if !area.is_finite() {
            return None;
        }
        Some(
            CalculationResult {
                area,
                result: format!("Круг S={} (d:{}, k:{})", area, d, f),
                shape: self.duplicate()
            }
        )
    }
}

#[derive(Clone)]
pub struct AreaRectangle {
    state: [FormElement; 6]
}

impl Default for AreaRectangle {
    fn default()-> Self {
        Self {
            state: [
                FormElement::InputField("Сторона А", String::new()),
                FormElement::InputField("Сторона B", String::new()),
                FormElement::FactorField(String::new()),
                FormElement::NoElement,
                FormElement::NoElement,
                FormElement::NoElement,
            ]
        }
    }
}

impl AreaShape for AreaRectangle {
    fn calculate(&self) -> Option<CalculationResult> {
        let a = helpers::get_number(&self.state[0])?;
        let b = helpers::get_number(&self.state[1])?;
        let f = helpers::get_factor(&self.state[2])?;
        let area = a * b * f;
        if !area.is_finite() {
            return None;
        }
        Some(
            CalculationResult {
                area,
                result: format!("Прямоугольник S={} (a:{}, b:{}, k:{})", area, a, b, f),
                shape: self.duplicate()
            }
        )
    }
   
    fn form_state(&mut self) -> &mut [FormElement; 6] {
        &mut self.state
    }
    
    fn name(&self) -> & str {
        "Прямоугольник"
    }
}

#[derive(Clone)]
pub struct AreaCylinder {
    state: [FormElement; 6]
}

impl Default for AreaCylinder {
    fn default()-> Self {
        Self {
            state: [
                FormElement::InputField("Сторона А", String::new()),
                FormElement::InputField("Сторона B", String::new()),
                FormElement::FactorField(String::new()),
                FormElement::NoElement,
                FormElement::NoElement,
                FormElement::NoElement,
            ]
        }
    }
}

impl AreaShape for AreaCylinder {
    fn calculate(&self) -> Option<CalculationResult> {
        let d = helpers::get_number(&self.state[0])?;
        let h = helpers::get_number(&self.state[1])?;
        let f = helpers::get_factor(&self.state[2])?;
        let area = d * std::f64::consts::PI * h * f;
        if !area.is_finite() {
            return None;
        }
        Some(
            CalculationResult{
                area,
                result: format!("Цилиндр S={} (d:{}, h:{}, k:{})", area, d, h, f),
                shape: self.duplicate()
            }
        )
    }
    fn form_state(&mut self) -> &mut [FormElement; 6] {
        &mut self.state
    }
    fn name(&self) -> & str {
        "Цилидр"
    }
}
