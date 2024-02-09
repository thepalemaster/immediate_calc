use crate::{literals, shapes};
use arboard::Clipboard;
use eframe::egui;
use eframe::egui::Key;
use egui_modal::Modal;

use calculator_state::CalculatorState;
use literals::messages;

mod calculator_state;
mod measure;

enum ViewFlags {
    NoFlags,
    Remove(usize),
    Modal(usize),
    //Message(&str)
}

pub struct Calculator {
    state: CalculatorState,
    current: usize,
    flags: ViewFlags,
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            state: CalculatorState::default(),
            current: 0,
            flags: ViewFlags::NoFlags,
        }
    }
}

impl Calculator {
    fn calculate(&mut self) {
        self.state.calculate(self.current);
    }

    fn calculation_list(&mut self, ui: &mut egui::Ui) {
        self.state
            .get_results()
            .iter()
            .enumerate()
            .for_each(|(index, item)| {
                ui.horizontal(|ui| {
                    ui.label(item.get_result());
                    if ui.add(egui::widgets::Button::new("⚙").small()).clicked() {
                        self.flags = ViewFlags::Modal(index);
                    }
                    if ui.add(egui::widgets::Button::new("❌").small()).clicked() {
                        self.flags = ViewFlags::Remove(index);
                    }
                });
            })
    }

    fn shape_chooser(&mut self, ui: &mut egui::Ui) {
        let row_size = 3;
        for (row, item) in self.state.get_shapes().chunks(row_size).enumerate() {
            ui.horizontal(|ui| {
                for (index, button) in item.iter().enumerate() {
                    if ui
                        .add(
                            egui::widgets::Button::new(button.name())
                                .min_size(egui::vec2(literals::STEP * 3., literals::STEP * 0.5)),
                        )
                        .clicked()
                    {
                        self.current = index + row_size * row;
                    }
                }
            });
        }
    }
    fn measure_units(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let (mut current_input, mut current_output) = self.state.current_units();
            egui::ComboBox::from_label(literals::INPUT_UNITS)
                .selected_text(current_input.name())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut current_input, measure::LengthUnits::MM, literals::MM);
                    ui.selectable_value(&mut current_input, measure::LengthUnits::SM, literals::SM);
                    ui.selectable_value(&mut current_input, measure::LengthUnits::DM, literals::DM);
                    ui.selectable_value(&mut current_input, measure::LengthUnits::M, literals::M);
                });
            self.state.new_input_unit(current_input);
            egui::ComboBox::from_label(literals::OUTPUT_UNITS)
                .selected_text(current_output.name())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut current_output,
                        measure::AreaUnits::MM2,
                        literals::MM2,
                    );
                    ui.selectable_value(
                        &mut current_output,
                        measure::AreaUnits::SM2,
                        literals::SM2,
                    );
                    ui.selectable_value(
                        &mut current_output,
                        measure::AreaUnits::DM2,
                        literals::DM2,
                    );
                    ui.selectable_value(&mut current_output, measure::AreaUnits::M2, literals::M2);
                });
            self.state.new_output_unit(current_output);
        });
    }
}

fn shape_input(shape: &mut [shapes::FormElement; 6], ui: &mut egui::Ui) {
    for field in shape {
        match field {
            shapes::FormElement::InputField(label, txt) => {
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(txt)
                        .labelled_by(ui.label(*label).id);
                });
            }
            shapes::FormElement::CheckBox(label, state) => {
                ui.horizontal(|ui| {
                    ui.checkbox(state, *label);
                });
            }
            shapes::FormElement::FactorField(txt) => {
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(txt)
                        .labelled_by(ui.label(literals::FACTOR).id);
                });
            }
            shapes::FormElement::NoElement => {
                ui.add_visible(false, egui::TextEdit::singleline(&mut String::new()));
            }
        }
    }
}

impl eframe::App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let spacing = ui.spacing().item_spacing.x;
            self.shape_chooser(ui);
            ui.label(self.state.get_shapes()[self.current].name());
            let shape = self.state.form_state(self.current);
            match shape {
                Some(form) => {
                    shape_input(form, ui);
                }
                None => {
                    for _i in 0..5 {
                        ui.label(" ");
                    }
                }
            }
            ui.horizontal(|ui| {
                if ui
                    .add(
                        egui::widgets::Button::new(literals::CALCULATE).min_size(egui::vec2(
                            literals::STEP * 9. + 2. * spacing,
                            literals::STEP * 0.5,
                        )),
                    )
                    .clicked()
                {
                    self.calculate();
                }
            });
            ui.horizontal(|ui| {
                if ui
                    .add(
                        egui::widgets::Button::new(literals::CLEAR).min_size(egui::vec2(
                            literals::STEP * 4.5 + spacing,
                            literals::STEP * 0.5,
                        )),
                    )
                    .clicked()
                {
                    self.state.clear();
                }
                if ui
                    .add(
                        egui::widgets::Button::new(literals::COPY)
                            .min_size(egui::vec2(literals::STEP * 4.5, literals::STEP * 0.5)),
                    )
                    .clicked()
                {
                    let clipboard = Clipboard::new();
                    match clipboard {
                        Ok(mut buffer) => match buffer.set_text(self.state.get_str_area()) {
                            Err(_) => {
                                self.state.new_message(messages::BUFFER_FAIL);
                            }
                            Ok(_) => {
                                self.state.new_message(messages::BUFFER_COPIED);
                            }
                        },
                        _ => {
                            self.state.new_message(messages::BUFFER_ERROR);
                        }
                    }
                }
            });
            self.measure_units(ui);
            ui.label(
                egui::RichText::new(format!("{} {}", literals::TOTAL, self.state.get_str_area()))
                    .size(literals::STEP / 2.)
                    .strong(),
            );

            egui::TopBottomPanel::bottom("status bar")
                .resizable(false)
                .min_height(0.)
                .show_inside(ui, |ui| {
                    ui.label(self.state.get_message(ui.ctx().input(|i| i.time)))
                });

            egui::CentralPanel::default().show_inside(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        self.calculation_list(ui);
                    })
            });
            //println!("{}", ui.ctx().input(|i| i.time));
            match self.flags {
                ViewFlags::Modal(index) => {
                    let modal = Modal::new(ctx, "edit_modal");
                    modal.show(|ui| {
                        modal.title(ui, literals::EDIT);
                        modal.frame(ui, |ui| {
                            ui.label(self.state.result_name(index));
                            let shape = self.state.form_state_from_result(index);
                            match shape {
                                Some(form) => {
                                    shape_input(form, ui);
                                }
                                None => {
                                    self.flags = ViewFlags::NoFlags;
                                }
                            }
                        });
                        modal.buttons(ui, |ui| {
                            if modal.button(ui, literals::CANCEL).clicked() {
                                self.flags = ViewFlags::NoFlags;
                            }
                            if modal.button(ui, literals::SAVE).clicked() {
                                self.state.recalculate(index);
                                self.flags = ViewFlags::NoFlags;
                            }
                        });
                    });
                    modal.open();
                }
                ViewFlags::Remove(index) => {
                    self.state.remove(index);
                    self.flags = ViewFlags::NoFlags;
                }
                _ => {}
            }
        });
        if ctx.input(|i| i.key_released(Key::Enter)) {
            self.calculate();
        }
    }
}
