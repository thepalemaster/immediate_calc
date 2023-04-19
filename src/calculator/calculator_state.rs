use crate::shapes;
use crate::calculator::measure;

pub struct CalculatorState {
    shapes: Vec<Box<dyn shapes::AreaShape>>,
    results: Vec<shapes::CalculationResult>,
    sum: f64,
    input_units: measure::LengthUnits,
    output_units: measure::AreaUnits
}

impl Default for CalculatorState {
    fn default() -> Self {
        Self {
            shapes: shapes::get_shapes(),
            results: Vec::new(),
            sum: 0.,
            input_units: measure::LengthUnits::MM,
            output_units: measure::AreaUnits::DM2,
        }
    }
}

impl CalculatorState {
    pub fn calculate(&mut self, index: usize) {
        if index >= self.shapes.len() {
            return;
        }
        let result = self.shapes[index].calculate(self.input_units.value(), self.output_units.value());
        if result.is_some() {
            self.sum += result.as_ref().unwrap().get_area();
            self.results.push(result.unwrap())
        }
    }

    pub fn form_state(&mut self, index:usize) -> Option<&mut [shapes::FormElement; 6]> {
        if index >= self.shapes.len() {
            return None;
        }
        Some(self.shapes[index].form_state())
    }

    pub fn form_state_from_result(&mut self, index:usize) -> Option<&mut [shapes::FormElement; 6]> {
        if index >= self.shapes.len() {
            return None;
        }
        Some(self.results[index].get_state().form_state())
    }

    pub fn result_name(&self, index: usize) -> &str {
        if index >= self.shapes.len() {
            return " ";
        }
        self.results[index].name()
    }

    pub fn recalculate (&mut self, result_index:usize) {
        let old_area = self.results[result_index].get_area();
        let result = self.results[result_index].
            get_state().calculate(self.input_units.value(), self.output_units.value());
        match result {
            Some(result) => {
                self.sum -= old_area;
                self.sum += result.get_area();
                self.results[result_index] = result;
            }
            None => {}
        }
    }

    pub fn get_results(&self) -> &Vec<shapes::CalculationResult> {
        &self.results
    }

    pub fn get_shapes(&self) -> &Vec<Box<dyn shapes::AreaShape>> {
        &self.shapes
    }

    pub fn get_area(&self) -> f64 {
        self.sum
    }

    pub fn new_output_unit(&mut self, unit: measure::AreaUnits) {
        if self.output_units != unit {
            return
        }
        self.output_units = unit;
        self.update_units();
    }

    pub fn current_units (&self) -> (measure::LengthUnits, measure::AreaUnits) {
        (self.input_units, self.output_units)
    }

    pub fn new_input_unit(&mut self, unit: measure::LengthUnits) {
        if self.input_units != unit {
            return
        }
        self.input_units = unit;
        self.update_units();
    }

    fn update_units(&mut self) {
        self.sum = 0.;
        for item in &mut self.results {
            item.recalculate(self.input_units.value(), self.output_units.value());
            self.sum += item.get_area();
        }
    }

    pub fn clear(&mut self) {
        self.results.clear();
        self.sum = 0.;
    }
    
    pub fn remove(&mut self, index: usize) {
        if index >= self.results.len() {
            return;
        }
        let result = self.results.remove(index);
        self.sum -= result.get_area();
    }

}
