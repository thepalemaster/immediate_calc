use std::fmt::Write;

use crate::calculator::measure;
use crate::literals::messages;
use crate::shapes;

pub struct CalculatorState {
    shapes: Vec<Box<dyn shapes::AreaShape>>,
    results: Vec<shapes::CalculationResult>,
    sum: f64,
    message: &'static str,
    area: String,
    timer: f64,
    input_units: measure::LengthUnits,
    output_units: measure::AreaUnits,
}

impl Default for CalculatorState {
    fn default() -> Self {
        Self {
            shapes: shapes::get_shapes(),
            results: Vec::new(),
            sum: 0.,
            message: "",
            area: String::from("0"),
            timer: -1.,
            input_units: measure::LengthUnits::MM,
            output_units: measure::AreaUnits::DM2,
        }
    }
}

impl CalculatorState {
    pub fn calculate(&mut self, index: usize) {
        if index >= self.shapes.len() {
            self.new_message(messages::SHAPE_FAIL);
            return;
        }
        let result =
            self.shapes[index].calculate(self.input_units.value(), self.output_units.value());
        match result {
            Ok(shape) => {
                self.sum += shape.get_area();
                self.results.push(shape);
                self.update_area();
            }
            Err(err) => self.new_message(err),
        }
    }

    pub fn form_state(&mut self, index: usize) -> Option<&mut [shapes::FormElement; 6]> {
        self.shapes.get_mut(index).map(|i| i.form_state())
    }

    pub fn form_state_from_result(
        &mut self,
        index: usize,
    ) -> Option<&mut [shapes::FormElement; 6]> {
        self.results
            .get_mut(index)
            .map(|shape| shape.get_state().form_state())
    }

    pub fn result_name(&self, index: usize) -> &str {
        match self.shapes.get(index) {
            Some(shape) => shape.name(),
            None => " ",
        }
    }

    pub fn recalculate(&mut self, result_index: usize) {
        let old_area = self.results[result_index].get_area();
        let result = self.results[result_index]
            .get_state()
            .calculate(self.input_units.value(), self.output_units.value());
        match result {
            Ok(result) => {
                self.sum -= old_area;
                self.sum += result.get_area();
                self.results[result_index] = result;
                self.update_area();
            }
            Err(err) => {
                self.new_message(err);
            }
        }
    }

    pub fn get_results(&self) -> &Vec<shapes::CalculationResult> {
        &self.results
    }

    pub fn get_shapes(&self) -> &Vec<Box<dyn shapes::AreaShape>> {
        &self.shapes
    }

    pub fn new_output_unit(&mut self, unit: measure::AreaUnits) {
        if self.output_units == unit {
            return;
        }
        let factor = self.output_units.value() / unit.value();
        self.output_units = unit;
        self.sum *= factor;
        self.results.iter_mut().for_each(|result| {
            result.scale_area(factor);
            result.update_result(self.input_units.value())
        });
        self.update_area();
    }

    pub fn current_units(&self) -> (measure::LengthUnits, measure::AreaUnits) {
        (self.input_units, self.output_units)
    }

    pub fn new_input_unit(&mut self, unit: measure::LengthUnits) {
        if self.input_units == unit {
            return;
        }
        self.input_units = unit;
        self.results
            .iter_mut()
            .for_each(|result| result.update_result(unit.value()))
    }

    pub fn clear(&mut self) {
        self.results.clear();
        self.sum = 0.;
        self.update_area();
    }

    pub fn remove(&mut self, index: usize) {
        if index >= self.results.len() {
            return;
        }
        let result = self.results.remove(index);
        self.sum -= result.get_area();
        self.update_area();
    }

    pub fn get_message(&mut self, time: f64) -> &'static str {
        if time > self.timer {
            self.timer = time;
            self.message = "";
        }
        self.message
    }

    pub fn new_message(&mut self, message: &'static str) {
        self.message = message;
        self.timer += 5.;
    }

    fn update_area(&mut self) {
        self.area.clear();
        match write!(&mut self.area, "{}", self.sum) {
            Err(_) => {
                self.new_message(messages::VIEW_FAIL);
                self.clear();
            }
            Ok(_) => {
                if let Some(pos) = self.area.find('.') {
                    if pos > 8 {
                        self.area.truncate(pos);
                    } else {
                        self.area.truncate(8);
                    }
                    shapes::localize(&mut self.area);
                }
            }
        }
    }

    pub fn get_str_area(&self) -> &str {
        self.area.as_str()
    }
}
