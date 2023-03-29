use eframe::{egui, epaint::{Stroke, Color32}};
use egui_modal::Modal;
use arboard::Clipboard;
use crate::{shapes, literals};

mod measure;

enum ViewFlags {
    NoFlags,
    Remove(usize),
    Modal(usize)
}

pub struct Calculator {
    shapes: Vec<Box<dyn shapes::AreaShape>>,
    current: usize, 
    flags: ViewFlags,
    results: Vec<shapes::CalculationResult>,
    sum: f64,
    input_units: measure::LengthUnits,
    output_units: measure::AreaUnits
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            shapes: shapes::get_shapes(),
            current: 0,
            flags: ViewFlags::NoFlags,
            results: Vec::new(),
            sum: 0.,
            input_units: measure::LengthUnits::MM,
            output_units: measure::AreaUnits::DM2,
        }
    }
}

impl Calculator {
    fn calculate(&mut self) {
        let result = self.shapes[self.current].calculate(self.input_units.value(), self.output_units.value());
        if result.is_some() {
            self.sum += result.as_ref().unwrap().get_area();
            self.results.push(result.unwrap())
        }
    }

    fn update_units(&mut self) {
        for item in &mut self.results {
            item.recalculate(self.input_units.value(), self.output_units.value());
        }
    }

    fn clear(&mut self) {
        self.results.clear();
        self.sum = 0.;
    }
    
    fn remove(&mut self, index: usize) {
        if index >= self.results.len() {
            return;
        }
        let result = self.results.remove(index);
        self.sum -= result.get_area();
    }

    fn calculation_list(&mut self, ui: &mut egui::Ui) {
        match self.flags {
            ViewFlags::Remove(index) => {
                    self.remove(index);
                    self.flags = ViewFlags::NoFlags;
                }
            _ => {}
        }
        self.results.iter().enumerate().for_each(|(index, item)| {
            ui.horizontal(|ui| {
                ui.label(item.get_result());
                if ui.add(egui::widgets::Button::new("📋")
                          .stroke(Stroke::NONE)
                          .small()
                          .fill(Color32::TRANSPARENT))
                    .clicked() {
                    self.flags = ViewFlags::Modal(index);
                }
                if ui.add(egui::widgets::Button::new("×")
                          .stroke(Stroke::NONE)
                          .small()
                          .fill(Color32::TRANSPARENT))
                    .clicked() {
                    self.flags = ViewFlags::Remove(index);
                }
            });
        })
    }

    fn shape_chooser(&mut self, ui: &mut egui::Ui) {
        let row_size = 3;
        for (row, item) in self.shapes.chunks(row_size).enumerate() {
            ui.horizontal( |ui| {
                for (index, button) in item.iter().enumerate() { 
                    if ui.add(egui::widgets::Button::new(button.name())
                                .min_size(egui::vec2(literals::STEP * 3., 0.))).clicked() {
                        self.current = index + row_size * row;
                    }
                }    
            });
        }
    }
    fn measure_units(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let current_input = self.input_units;
            let current_output = self.output_units;
            egui::ComboBox::from_label(literals::INPUT_UNITS)
                .selected_text(self.input_units.name())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.input_units, measure::LengthUnits::MM, literals::MM);
                    ui.selectable_value(&mut self.input_units, measure::LengthUnits::SM, literals::SM);
                    ui.selectable_value(&mut self.input_units, measure::LengthUnits::DM, literals::DM);
                    ui.selectable_value(&mut self.input_units, measure::LengthUnits::M, literals::M);
                });
            if current_input != self.input_units {
                self.update_units();
            }
            egui::ComboBox::from_label(literals::OUTPUT_UNITS)
                .selected_text(self.output_units.name())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.output_units, measure::AreaUnits::MM2, literals::MM2);
                    ui.selectable_value(&mut self.output_units, measure::AreaUnits::SM2, literals::SM2);
                    ui.selectable_value(&mut self.output_units, measure::AreaUnits::DM2, literals::DM2);
                    ui.selectable_value(&mut self.output_units, measure::AreaUnits::M2, literals::M2);
                });
            if current_output != self.output_units {
                self.update_units();
            }
        });
    }
}

fn shape_input(shape: &mut [shapes::FormElement; 6], ui: &mut egui::Ui) {
    for field in shape {
        match field {
            shapes::FormElement::InputField(label, txt) => {
                ui.horizontal(|ui|{
                    ui.text_edit_singleline(txt).labelled_by(ui.label(*label).id);
                });
            }
           shapes::FormElement::CheckBox(label, state) => {
                ui.horizontal(|ui| {
                    ui.checkbox(state, *label);
                });
            }
            shapes::FormElement::FactorField(txt) => {
                ui.horizontal(|ui|{
                    ui.text_edit_singleline(txt).labelled_by(ui.label(literals::FACTOR).id);
                });
            }
            shapes::FormElement::NoElement => {
                ui.label(" ");
            }
        }
    }
}


impl eframe::App for Calculator{
    fn update (&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let spacing = ui.spacing().item_spacing.x;
            self.shape_chooser(ui);
            ui.label(self.shapes[self.current].name());
            let shape = self.shapes[self.current].form_state();
            shape_input(shape, ui);
            ui.horizontal(|ui|{
                if ui.add(egui::widgets::Button::new(literals::CALCULATE)
                    .min_size(egui::vec2(literals::STEP * 9. + 2. * spacing, 0.))).clicked() {
                    self.calculate();
                }
            });
            ui.horizontal(|ui|{
                if ui.add(egui::widgets::Button::new(literals::CLEAR)
                    .min_size(egui::vec2(literals::STEP * 4.5 + spacing, 0.))).clicked() {
                    self.clear();
                }
                if ui.add(egui::widgets::Button::new(literals::COPY)
                    .min_size(egui::vec2(literals::STEP * 4.5, 0.))).clicked() {
                    let clipboard = Clipboard::new();
                    match clipboard {
                        Ok(mut buffer) => {
                            match buffer.set_text(format!("{}", self.sum)) {
                                Err(e) => {
                                    println!("{}", e);
                                }
                                Ok(_) => {}
                            }
                        }
                        _ => {
                            println!("System buffer unavailable");
                        }
                    }
                }
            });
            self.measure_units(ui);
            ui.label(format!("{} {}", literals::TOTAL, self.sum));
            self.calculation_list(ui);
            match self.flags {
                ViewFlags::Modal(index) => {
                    let modal = Modal::new(ctx, "edit_modal");
                    modal.show( |ui| {
                        modal.title(ui, literals::EDIT);
                        modal.frame(ui, |ui| {
                            let shape = self.results[index].get_state().form_state();
                            shape_input(shape, ui);
                        });
                        modal.buttons(ui, |ui| {
                            if modal.button(ui, literals::CANCEL).clicked() {
                                self.flags = ViewFlags::NoFlags;
                            }
                            if modal.button(ui, literals::SAVE).clicked() {
                                let shape = self.results[index].get_state();
                                let result = shape.calculate(self.input_units.value(), self.output_units.value());
                                if result.is_some() {
                                    self.results[index] = result.unwrap();
                                }
                                self.flags = ViewFlags::NoFlags;
                            }
                        });
                    });
                    modal.open();
                }
                _ => {}
            }
        });
    }
}
