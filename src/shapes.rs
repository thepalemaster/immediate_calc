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
    fn calculate(&self, input_factor: f64, output_factor: f64) -> Option<CalculationResult>;
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
    pub fn recalculate(&mut self, input_factor: f64, output_factor: f64) {
        let result = self.shape.calculate(input_factor, output_factor);
        if result.is_some() {
            let some_result = result.unwrap();
            self.area = some_result.get_area();
            self.result = some_result.result;
        }
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
        helpers::std_validate_state(& mut self.state);
        &mut self.state
    }
    fn name (&self) -> & str{
        "Круг"
    }
    fn calculate(&self, input_factor: f64, output_factor: f64 ) -> Option<CalculationResult> {
        let d = helpers::get_number(&self.state[0], input_factor)?;
        let f = helpers::get_factor(&self.state[1])?;
        let area = f * std::f64::consts::PI * d * d / 4.0 / output_factor;
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
    fn calculate(&self, input_factor: f64, output_factor: f64) -> Option<CalculationResult> {
        let a = helpers::get_number(&self.state[0], input_factor)?;
        let b = helpers::get_number(&self.state[1], input_factor)?;
        let f = helpers::get_factor(&self.state[2])?;
        let area = a * b * f / output_factor;
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
        helpers::std_validate_state(& mut self.state);
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
                FormElement::InputField("Диаметр", String::new()),
                FormElement::InputField("Высота", String::new()),
                FormElement::FactorField(String::new()),
                FormElement::CheckBox("Резьба", false),
                FormElement::NoElement,
                FormElement::NoElement,
            ]
        }
    }
}

impl AreaShape for AreaCylinder {
    fn calculate(&self, input_factor: f64, output_factor: f64) -> Option<CalculationResult> {
        let d = helpers::get_number(&self.state[0], input_factor)?;
        let h = helpers::get_number(&self.state[1], input_factor)?;
        let f = helpers::get_factor(&self.state[2])?;
        let mut area = d * std::f64::consts::PI * h * f / output_factor;
        if !area.is_finite() {
            return None;
        }
        let string_result;
        if let FormElement::CheckBox(_, threaded) = self.state[3] {
            if threaded {
                area *= 1.5;
                string_result = format!("Резьба S={} (d:{}, h:{}, k:{})", area, d, h, f);
            } else {
                string_result = format!("Цилиндр S={} (d:{}, h:{}, k:{})", area, d, h, f);
            }
        } else {
            return None;
        }
        Some(
            CalculationResult{
                area,
                result: string_result,
                shape: self.duplicate()
            }
        )
    }
    fn form_state(&mut self) -> &mut [FormElement; 6] {
        helpers::std_validate_state(& mut self.state);
        &mut self.state
    }
    fn name(&self) -> & str {
        if let FormElement::CheckBox(_, threaded) = self.state[3] {
            if threaded {
                return "Резьба";
            }
        }
        "Цилидр"
    }
}

#[derive(Clone)]
pub struct AreaHexagon {
    state: [FormElement; 6]
}

impl Default for AreaHexagon {
    fn default()-> Self {
        Self {
            state: [
                FormElement::InputField("Диаметр", String::new()),
                FormElement::CheckBox("Описанная окружность", false),
                FormElement::NoElement,
                FormElement::NoElement,
                FormElement::NoElement,
                FormElement::NoElement,
            ]
        }
    }
}

impl AreaShape for AreaHexagon {
    fn form_state(&mut self) -> &mut [FormElement; 6] {
        helpers::std_validate_state(& mut self.state);
        &mut self.state
    }
    fn name (&self) -> & str{
        "Шестиугольник"
    }
    fn calculate(&self, input_factor: f64, output_factor: f64) -> Option<CalculationResult> {
        let d = helpers::get_number(&self.state[0], input_factor)?;
        if let FormElement::CheckBox(_,  circumscribed) = self.state[1] {
            let area;
            let result;
            if circumscribed {
                area = 3. * f64::sqrt(3.) / 2. * d * d / 4. / output_factor;
                result = format!("Шестиугольник S={} (D:{})", area, d);
            } else {
                area = 2. * f64::sqrt(3.) * d * d / 4. / output_factor;
                result = format!("Шестиугольник S={} (d:{})", area, d);
            }
            if !area.is_finite() {
                return None;
            }
            return Some(
                CalculationResult {
                    area,
                    result,
                    shape: self.duplicate()
                }
            )
        };
        None
    }
}

