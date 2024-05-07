mod helpers;
mod parser;

use super::FormElement;
use super::InnerImplShape;
use crate::literals;

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

#[derive(Clone)]
pub struct AreaCircle {
    state: [FormElement; 6],
    diameter: f64,
    factor: f64,
}

impl Default for AreaCircle {
    fn default() -> Self {
        Self {
            state: [
                FormElement::InputField(DIAMETER_CIR, String::new()),
                FormElement::FactorField(String::new()),
                FormElement::NoElement,
                FormElement::NoElement,
                FormElement::NoElement,
                FormElement::NoElement,
            ],
            diameter: 0.,
            factor: 1.,
        }
    }
}

impl InnerImplShape for AreaCircle {
    fn state(&mut self) -> &mut [FormElement; 6] {
        helpers::std_validate_state(&mut self.state);
        &mut self.state
    }

    fn get_name(&self) -> &str {
        CIRCLE
    }

    fn parse_input(&mut self, input_factor: f64) -> Result<(), &'static str> {
        let mut negative = false;
        self.diameter = helpers::get_lenght(&self.state[0], input_factor, &mut negative)?;
        self.factor = helpers::get_factor(&self.state[1], negative)?;
        Ok(())
    }

    fn get_area(&self) -> f64 {
        self.factor * self.diameter * self.diameter * std::f64::consts::PI / 4.
    }

    fn get_result(&self, input_factor: f64, area: f64) -> String {
        let d = self.diameter / input_factor;
        if cfg!(feature = "lang_rus") {
            format!("Круг S={} (d:{}, k:{})", area, d, self.factor)
        } else {
            format!("Circle S={} (d:{}, k:{})", area, d, self.factor)
        }
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

#[derive(Clone)]
pub struct AreaRectangle {
    state: [FormElement; 6],
    lenght: f64,
    height: f64,
    factor: f64,
}

impl Default for AreaRectangle {
    fn default() -> Self {
        Self {
            state: [
                FormElement::InputField(LENGHT_REC, String::new()),
                FormElement::InputField(HEIGHT_REC, String::new()),
                FormElement::FactorField(String::new()),
                FormElement::NoElement,
                FormElement::NoElement,
                FormElement::NoElement,
            ],
            lenght: 0.,
            height: 0.,
            factor: 1.,
        }
    }
}

impl InnerImplShape for AreaRectangle {
    fn parse_input(&mut self, input_factor: f64) -> Result<(), &'static str> {
        let mut negative = false;
        self.lenght = helpers::get_lenght(&self.state[0], input_factor, &mut negative)?;
        self.height = helpers::get_lenght(&self.state[1], input_factor, &mut negative)?;
        self.factor = helpers::get_factor(&self.state[2], negative)?;
        Ok(())
    }

    fn get_area(&self) -> f64 {
        self.height * self.lenght * self.factor
    }

    fn state(&mut self) -> &mut [FormElement; 6] {
        helpers::std_validate_state(&mut self.state);
        &mut self.state
    }

    fn get_name(&self) -> &str {
        RECTANGLE
    }
    fn get_result(&self, input_factor: f64, area: f64) -> String {
        let b = self.height / input_factor;
        let a = self.lenght / input_factor;
        if cfg!(feature = "lang_rus") {
            format!(
                "Прямоугольник S={} (a:{}, b:{}, k:{})",
                area, a, b, self.factor
            )
        } else {
            format!("Rectangle S={} (l:{}, h:{}, k:{})", area, a, b, self.factor)
        }
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

#[derive(Clone)]
pub struct AreaCylinder {
    state: [FormElement; 6],
    diameter: f64,
    height: f64,
    factor: f64,
    threaded: bool,
}

impl Default for AreaCylinder {
    fn default() -> Self {
        Self {
            state: [
                FormElement::InputField(DIAMETER_CYL, String::new()),
                FormElement::InputField(HEIGHT_CYL, String::new()),
                FormElement::FactorField(String::new()),
                FormElement::CheckBox(THREADED, false),
                FormElement::NoElement,
                FormElement::NoElement,
            ],
            diameter: 0.,
            height: 0.,
            factor: 1.,
            threaded: false,
        }
    }
}

impl InnerImplShape for AreaCylinder {
    fn parse_input(&mut self, input_factor: f64) -> Result<(), &'static str> {
        let mut negative = false;
        self.diameter = helpers::get_lenght(&self.state[0], input_factor, &mut negative)?;
        self.height = helpers::get_lenght(&self.state[1], input_factor, &mut negative)?;
        self.factor = helpers::get_factor(&self.state[2], negative)?;
        self.threaded = helpers::get_option(&self.state[3])?;
        Ok(())
    }

    fn get_name(&self) -> &str {
        if self.threaded {
            THREADED
        } else {
            CYLINDER
        }
    }

    fn state(&mut self) -> &mut [FormElement; 6] {
        helpers::std_validate_state(&mut self.state);
        &mut self.state
    }

    fn get_area(&self) -> f64 {
        let mut area = self.diameter * std::f64::consts::PI * self.height * self.factor;
        if self.threaded {
            area *= 1.5;
        }
        area
    }

    fn get_result(&self, input_factor: f64, area: f64) -> String {
        let d = self.diameter / input_factor;
        let h = self.height / input_factor;
        if self.threaded {
            if cfg!(feature = "lang_rus") {
                format!("Резьба S={} (d:{}, h:{}, k:{})", area, d, h, self.factor)
            } else {
                format!("Threaded S={} (d:{}, h:{}, k:{})", area, d, h, self.factor)
            }
        } else if cfg!(feature = "lang_rus") {
            format!("Цилиндр S={} (d:{}, h:{}, k:{})", area, d, h, self.factor)
        } else {
            format!("Cylinder S={} (d:{}, h:{}, k:{})", area, d, h, self.factor)
        }
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

const HEXAGON: &str = if cfg!(feature = "lang_rus") {
    "Шестиугольник"
} else {
    "Hexagon"
};

#[derive(Clone)]
pub struct AreaHexagon {
    state: [FormElement; 6],
    diameter: f64,
    factor: f64,
    circumscribed: bool,
}

impl Default for AreaHexagon {
    fn default() -> Self {
        Self {
            state: [
                FormElement::InputField(DIAMETER_HEX, String::new()),
                FormElement::FactorField(String::new()),
                FormElement::CheckBox(CIRCUMSCRIBED, false),
                FormElement::NoElement,
                FormElement::NoElement,
                FormElement::NoElement,
            ],
            diameter: 0.,
            factor: 1.,
            circumscribed: false,
        }
    }
}

impl InnerImplShape for AreaHexagon {
    fn state(&mut self) -> &mut [FormElement; 6] {
        helpers::std_validate_state(&mut self.state);
        &mut self.state
    }

    fn get_name(&self) -> &str {
        HEXAGON
    }

    fn parse_input(&mut self, input_factor: f64) -> Result<(), &'static str> {
        let mut negative = false;
        self.diameter = helpers::get_lenght(&self.state[0], input_factor, &mut negative)?;
        self.factor = helpers::get_factor(&self.state[1], negative)?;
        self.circumscribed = helpers::get_option(&self.state[2])?;
        Ok(())
    }

    fn get_area(&self) -> f64 {
        if self.circumscribed {
            3. * f64::sqrt(3.) / 2. * self.diameter * self.diameter / 4. * self.factor
        } else {
            2. * f64::sqrt(3.) * self.diameter * self.diameter / 4. * self.factor
        }
    }

    fn get_result(&self, input_factor: f64, area: f64) -> String {
        let d = self.diameter / input_factor;
        if self.circumscribed {
            if cfg!(feature = "lang_rus") {
                format!("Шестиугольник S={} (D:{}, k:{})", area, d, self.factor)
            } else {
                format!("Hexagon S={} (D:{}, k:{})", area, d, self.factor)
            }
        } else {
            if cfg!(feature = "lang_rus") {
                format!("Шестиугольник s={} (d:{}, k:{})", area, d, self.factor)
            } else {
                format!("Hexagon s={} (d:{}, k:{})", area, d, self.factor)
            }
        }
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

#[derive(Clone)]
pub struct AreaHexagonPrism {
    state: [FormElement; 6],
    diameter: f64,
    height: f64,
    factor: f64,
    circumscribed: bool,
}

impl Default for AreaHexagonPrism {
    fn default() -> Self {
        Self {
            state: [
                FormElement::InputField(DIAMETER_HEX, String::new()),
                FormElement::InputField(HEX_PRISM_HEIGHT, String::new()),
                FormElement::FactorField(String::new()),
                FormElement::CheckBox(CIRCUMSCRIBED, false),
                FormElement::NoElement,
                FormElement::NoElement,
            ],
            diameter: 0.,
            height: 0.,
            factor: 1.,
            circumscribed: false,
        }
    }
}
impl InnerImplShape for AreaHexagonPrism {
    fn state(&mut self) -> &mut [FormElement; 6] {
        helpers::std_validate_state(&mut self.state);
        &mut self.state
    }

    fn get_name(&self) -> &str {
        HEX_PRISM
    }

    fn parse_input(&mut self, input_factor: f64) -> Result<(), &'static str> {
        let mut negative = false;
        self.diameter = helpers::get_lenght(&self.state[0], input_factor, &mut negative)?;
        self.height = helpers::get_lenght(&self.state[1], input_factor, &mut negative)?;
        self.factor = helpers::get_factor(&self.state[2], negative)?;
        self.circumscribed = helpers::get_option(&self.state[3])?;
        Ok(())
    }

    fn get_area(&self) -> f64 {
        if self.circumscribed {
            6. * self.diameter / 2. * self.height * self.factor
        } else {
            6. * self.diameter * 2. / f64::sqrt(3.) / 2. * self.height * self.factor
        }
    }

    fn get_result(&self, input_factor: f64, area: f64) -> String {
        let h = self.height / input_factor;
        let d = self.diameter / input_factor;
        if self.circumscribed {
            if cfg!(feature = "lang_rus") {
                format!(
                    "Призма (N=6) S={} (D:{}, h:{}, k:{})",
                    area, d, h, self.factor
                )
            } else {
                format!(
                    "Hexagon prism S={} (D:{}, h:{}, k:{})",
                    area, d, h, self.factor
                )
            }
        } else {
            if cfg!(feature = "lang_rus") {
                format!(
                    "Призма (N=6) S={} (d:{}, h:{}, k:{})",
                    area, d, h, self.factor
                )
            } else {
                format!(
                    "Hexagon prism S={} (d:{}, h:{}, k:{})",
                    area, d, h, self.factor
                )
            }
        }
    }
}

const BUSHING: &str = if cfg!(feature = "lang_rus") {
    "Втулка"
} else {
    "Bushing"
};

const BUSHING_HEIGHT: &str = if cfg!(feature = "lang_rus") {
    "Высота"
} else {
    "Height"
};

const BUSHING_DIAMETER: &str = if cfg!(feature = "lang_rus") {
    "Диаметр втулки"
} else {
    "Outer diameter"
};

const BUSHING_INNER_DIAMETER: &str = if cfg!(feature = "lang_rus") {
    "Диаметр отверстия"
} else {
    "Inner diameter"
};

#[derive(Clone)]
pub struct AreaBushing {
    state: [FormElement; 6],
    diameter: f64,
    inner_diameter: f64,
    height: f64,
    factor: f64,
}

impl Default for AreaBushing {
    fn default() -> Self {
        Self {
            state: [
                FormElement::InputField(BUSHING_DIAMETER, String::new()),
                FormElement::InputField(BUSHING_INNER_DIAMETER, String::new()),
                FormElement::InputField(BUSHING_HEIGHT, String::new()),
                FormElement::FactorField(String::new()),
                FormElement::NoElement,
                FormElement::NoElement,
            ],
            diameter: 0.,
            inner_diameter: 0.,
            height: 0.,
            factor: 1.,
        }
    }
}

impl InnerImplShape for AreaBushing {
    fn state(&mut self) -> &mut [FormElement; 6] {
        helpers::std_validate_state(&mut self.state);
        &mut self.state
    }

    fn get_name(&self) -> &str {
        BUSHING
    }

    fn parse_input(&mut self, input_factor: f64) -> Result<(), &'static str> {
        let mut negative = false;
        self.diameter = helpers::get_lenght(&self.state[0], input_factor, &mut negative)?;
        self.inner_diameter = helpers::get_lenght(&self.state[1], input_factor, &mut negative)?;
        self.height = helpers::get_lenght(&self.state[2], input_factor, &mut negative)?;
        self.factor = helpers::get_factor(&self.state[3], negative)?;
        if self.diameter <= self.inner_diameter {
            return Err(literals::messages::WRONG_BUSHING);
        }
        Ok(())
    }
    fn get_area(&self) -> f64 {
        ((self.diameter + self.inner_diameter) * std::f64::consts::PI * self.height
            + std::f64::consts::PI
                * (self.diameter * self.diameter - self.inner_diameter * self.inner_diameter)
                / 4.0)
            * self.factor
    }

    fn get_result(&self, input_factor: f64, area: f64) -> String {
        let d1 = self.diameter / input_factor;
        let d2 = self.inner_diameter / input_factor;
        let h = self.height / input_factor;
        if cfg!(feature = "lang_rus") {
            format!(
                "Втулка S={} (D:{}, d:{}, h:{}, k:{})",
                area, d1, d2, h, self.factor
            )
        } else {
            format!(
                "Bushing S={} (D:{}, d:{}, h:{}, k:{})",
                area, d1, d2, h, self.factor
            )
        }
    }
}

const CUBOID: &str = if cfg!(feature = "lang_rus") {
    "Параллелепипед"
} else {
    "Cuboid"
};

const CUBOID_HEIGHT: &str = if cfg!(feature = "lang_rus") {
    "Высота"
} else {
    "Height"
};

const CUBOID_BREADTH: &str = if cfg!(feature = "lang_rus") {
    "Глубина"
} else {
    "Breadth"
};

const CUBOID_WIDTH: &str = if cfg!(feature = "lang_rus") {
    "Ширина"
} else {
    "Width"
};

#[derive(Clone)]
pub struct AreaCuboid {
    state: [FormElement; 6],
    breadth: f64,
    height: f64,
    width: f64,
    factor: f64,
}

impl Default for AreaCuboid {
    fn default() -> Self {
        Self {
            state: [
                FormElement::InputField(CUBOID_HEIGHT, String::new()),
                FormElement::InputField(CUBOID_BREADTH, String::new()),
                FormElement::InputField(CUBOID_WIDTH, String::new()),
                FormElement::FactorField(String::new()),
                FormElement::NoElement,
                FormElement::NoElement,
            ],
            breadth: 0.,
            height: 0.,
            width: 0.,
            factor: 1.,
        }
    }
}

impl InnerImplShape for AreaCuboid {
    fn state(&mut self) -> &mut [FormElement; 6] {
        helpers::std_validate_state(&mut self.state);
        &mut self.state
    }

    fn get_name(&self) -> &str {
        CUBOID
    }

    fn parse_input(&mut self, input_factor: f64) -> Result<(), &'static str> {
        let mut negative = false;
        self.height = helpers::get_lenght(&self.state[0], input_factor, &mut negative)?;
        self.breadth = helpers::get_lenght(&self.state[1], input_factor, &mut negative)?;
        self.width = helpers::get_lenght(&self.state[2], input_factor, &mut negative)?;
        self.factor = helpers::get_factor(&self.state[3], negative)?;
        Ok(())
    }
    fn get_area(&self) -> f64 {
        (2.0 * self.width * self.breadth +
        2.0 * self.width * self.height +
        2.0 * self.breadth * self.height)
        * self.factor
    }

    fn get_result(&self, input_factor: f64, area: f64) -> String {
        let a = self.height / input_factor;
        let b = self.breadth / input_factor;
        let c = self.width / input_factor;
        if cfg!(feature = "lang_rus") {
            format!(
                "Параллелепипед S={} (a:{}, b:{}, c:{}, k:{})",
                area, a, b, c, self.factor
            )
        } else {
            format!(
                "Cuboid S={} (a:{}, b:{}, c:{}, k:{})",
                area, a, b, c, self.factor
            )
        }
    }
}
