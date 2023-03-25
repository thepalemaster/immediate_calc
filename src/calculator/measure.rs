
#[derive(Copy, Clone, PartialEq)]
pub enum LengthUnits {
    MM,
    SM,
    DM,
    M,
}

#[derive(Copy, Clone, PartialEq)]
pub enum AreaUnits {
    MM2,
    SM2,
    DM2,
    M2,
}


impl LengthUnits {
    pub fn value(&self) -> f64 {
        match self {
            LengthUnits::MM => 1.,
            LengthUnits::SM => 10.,
            LengthUnits::DM => 100.,
            LengthUnits::M => 1000.
        }
    }

    pub fn name(&self) -> &str{
        match self {
            LengthUnits::MM => "мм",
            LengthUnits::SM => "см",
            LengthUnits::DM => "дм",
            LengthUnits::M => "м"
        }
    }
}

impl AreaUnits {
    pub fn value(&self) -> f64 {
        match self {
            AreaUnits::MM2 => 1.,
            AreaUnits::SM2 => 100.,
            AreaUnits::DM2 => 10000.,
            AreaUnits::M2 => 1000000.
        }
    }
    pub fn name(&self) -> &str {
        match self {
            AreaUnits::MM2 => "мм²",
            AreaUnits::SM2 => "см²",
            AreaUnits::DM2 => "дм²",
            AreaUnits::M2 => "м²"
        }
    }
}

