mod helpers;
mod parser;

use super::AreaShapeClone;
use super::AreaShape;
use super::CalculationResult;
use super::FormElement;

const CIRCLE: &str = if cfg!(feature = "lang_rus") {
    "Круг"
} else {
    "Circle"
};

const DIAMETER_CIR: &str = if cfg!(feature = "lang_rus") {
    "Диаметр"
} else {
    "Diameter"
};

fn circle_string (area: f64, d: f64, f: f64) -> String{
    if cfg!(feature = "lang_rus") {
        format!("Круг S={} (d:{}, k:{})", area, d, f)
    } else {
        format!("Circle S={} (d:{}, k:{})", area, d, f)
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
                FormElement::InputField(DIAMETER_CIR, String::new()),
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
        CIRCLE
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
                result: circle_string(area, d, f),
                shape: self.duplicate()
            }
        )
    }
}


const RECTANGLE: &str = if cfg!(feature = "lang_rus") {
    "Прямоугольник"
} else {
    "Rectangle"
};

const LENGHT_REC: &str = if cfg!(feature = "lang_rus") {
    "Длина"
} else {
    "Lenght"
};

const HEIGHT_REC: &str = if cfg!(feature = "lang_rus") {
    "Высота"
} else {
    "Height"
};

fn rectangle_string (area: f64, h: f64, l: f64, f: f64) -> String {
    if cfg!(feature = "lang_rus") {
        format!("Прямоугольник S={} (a:{}, b:{}, k:{})", area, l, h, f)
    } else {
        format!("Rectangle S={} (l:{}, h:{}, k:{})", area, l, h, f)
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
                FormElement::InputField(LENGHT_REC, String::new()),
                FormElement::InputField(HEIGHT_REC, String::new()),
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
                result: rectangle_string(area, b, a, f),
                shape: self.duplicate()
            }
        )
    }
   
    fn form_state(&mut self) -> &mut [FormElement; 6] {
        helpers::std_validate_state(& mut self.state);
        &mut self.state
    }
    
    fn name(&self) -> & str {
        RECTANGLE
    }
}


const HEIGHT_CYL: &str = if cfg!(feature = "lang_rus") {
    "Высота"
} else {
    "Height"
};

const DIAMETER_CYL: &str = if cfg!(feature = "lang_rus") {
    "Диаметр"
} else {
    "Diameter"
};

const CYLINDER: &str = if cfg!(feature = "lang_rus") {
    "Цилиндр"
} else {
    "Cylinder"
};


const THREADED: &str = if cfg!(feature = "lang_rus") {
    "Резьба"
} else {
    "Threaded"
};


fn cylinder_string (area: f64, d: f64, h: f64, f: f64, threaded: bool) -> String {
    if threaded {
        if cfg! (feature = "lang_rus") {
            format!("Резьба S={} (d:{}, h:{}, k:{})", area, d, h, f)
        } else {
            format!("Threaded S={} (d:{}, h:{}, k:{})", area, d, h, f)
        }
    } else {
        if cfg! (feature = "lang_rus") {
            format!("Цилиндр S={} (d:{}, h:{}, k:{})", area, d, h, f)
        } else {
            format!("Cylinder S={} (d:{}, h:{}, k:{})", area, d, h, f)
        }
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
                FormElement::InputField(DIAMETER_CYL, String::new()),
                FormElement::InputField(HEIGHT_CYL, String::new()),
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
        if let FormElement::CheckBox(_, threaded) = self.state[3] {
            if threaded {
                area *= 1.5;
            }
            Some(
                CalculationResult{
                    area,
                    result: cylinder_string(area, d, h, f, threaded),
                    shape: self.duplicate()
                }
            )
        } else {
            None
        }
    }

    fn form_state(&mut self) -> &mut [FormElement; 6] {
        helpers::std_validate_state(& mut self.state);
        &mut self.state
    }

    fn name(&self) -> & str {
        if let FormElement::CheckBox(_, threaded) = self.state[3] {
            if threaded {
                return THREADED;
            }
        }
        CYLINDER
    }
}

const DIAMETER_HEX: &str = if cfg!(feature = "lang_rus") {
    "Диаметр"
} else {
    "Diameter"
};

const CIRCUMSCRIBED: &str = if cfg!(feature = "lang_rus") {
    "Описанная окружность"
} else {
    "Circumscribed circle"
};

//const INSCRIBED: &str = if cfg!(feature = "lang_rus") {
//    "Вписанная окружность"
//} else {
//    "Inscribed circle"
//};

const HEXAGON: &str = if cfg!(feature = "lang_rus") {
    "Шестиугольник"
} else {
    "Hexagon"
};

fn hexagon_string (area: f64, d: f64, f: f64, circumscribed: bool) -> String {
    if circumscribed {
        if cfg! (feature = "lang_rus") {
            format!("Шестиугольник S={} (D:{}, k:{})", area, d, f)
        } else {
            format!("Hexagon S={} (D:{}, k:{})", area, d, f)
        }
    } else {
        if cfg! (feature = "lang_rus") {
            format!("Шестиугольник s={} (d:{}, k:{})", area, d, f)
        } else {
            format!("Hexagon s={} (d:{}, k:{})", area, d, f)
        }
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
                FormElement::InputField(DIAMETER_HEX, String::new()),
                FormElement::FactorField(String::new()),
                FormElement::CheckBox(CIRCUMSCRIBED, false),
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
        HEXAGON
    }

    fn calculate(&self, input_factor: f64, output_factor: f64) -> Option<CalculationResult> {
        let d = helpers::get_number(&self.state[0], input_factor)?;
        let f = helpers::get_factor(&self.state[1])?;
        if let FormElement::CheckBox(_,  circumscribed) = self.state[2] {
            let area;
            if circumscribed {
                area = 3. * f64::sqrt(3.) / 2. * d * d / 4. * f / output_factor;
            } else {
                area = 2. * f64::sqrt(3.) * d * d / 4. * f / output_factor;
            }
            if !area.is_finite() {
                return None;
            }
            return Some(
                CalculationResult {
                    area,
                    result: hexagon_string(area, d, f, circumscribed),
                    shape: self.duplicate()
                }
            )
        };
        None
    }
}

const HEX_PRISM: &str = if cfg!(feature = "lang_rus") {
    "Шестиугольная призма"
} else {
    "Hexagon prism"
};

const HEX_PRISM_HEIGHT: &str = if cfg!(feature = "lang_rus") {
    "Высота"
} else {
    "Height"
};

fn hexagon_prism_string (area: f64, d: f64, h: f64, f: f64, circumscribed: bool) -> String {
    if circumscribed {
        if cfg! (feature = "lang_rus") {
            format!("Призма (N=6) S={} (D:{}, h:{}, k:{})", area, d, h, f)
        } else {
            format!("Hexagon prism S={} (D:{}, h:{}, k:{})", area, d, h, f)
        }
    } else {
        if cfg! (feature = "lang_rus") {
            format!("Призма (N=6) S={} (d:{}, h:{}, k:{})", area, d, h, f)
        } else {
            format!("Hexagon prism S={} (d:{}, h:{}, k:{})", area, d, h, f)
        }
    }
}
#[derive(Clone)]
pub struct AreaHexagonPrism {
    state: [FormElement; 6]
}

impl Default for AreaHexagonPrism {
    fn default()-> Self {
        Self {
            state: [
                FormElement::InputField(DIAMETER_HEX, String::new()),
                FormElement::InputField(HEX_PRISM_HEIGHT, String::new()),
                FormElement::FactorField(String::new()),
                FormElement::CheckBox(CIRCUMSCRIBED, false),
                FormElement::NoElement,
                FormElement::NoElement,
            ]
        }
    }
}
impl AreaShape for AreaHexagonPrism {
    fn form_state(&mut self) -> &mut [FormElement; 6] {
        helpers::std_validate_state(& mut self.state);
        &mut self.state
    }

    fn name (&self) -> & str{
        HEX_PRISM
    }

    fn calculate(&self, input_factor: f64, output_factor: f64) -> Option<CalculationResult> {
        let d = helpers::get_number(&self.state[0], input_factor)?;
        let h = helpers::get_number(&self.state[1], input_factor)?;
        let f = helpers::get_factor(&self.state[2])?;
        if let FormElement::CheckBox(_,  circumscribed) = self.state[3] {
            let area;
            if circumscribed {
                area = 6. * d / 2. * h * f / output_factor;
            } else {
                area = 6. * d * 2. / f64::sqrt(3.) / 2. * h * f / output_factor;
            }
            if !area.is_finite() {
                return None;
            }
            return Some(
                CalculationResult {
                    area,
                    result: hexagon_prism_string(area, d, h, f, circumscribed),
                    shape: self.duplicate()
                }
            )
        };
        None
    }
}
