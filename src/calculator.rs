use eframe::egui;
use egui_modal::Modal;
use arboard::Clipboard;
use crate::shapes;

const STEP: f32 = 50.;

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
    sum: f64
}

impl Calculator {
    fn calculate(&mut self) {
        let result = self.shapes[self.current].calculate();
        if result.is_some() {
            self.sum += result.as_ref().unwrap().get_area();
            self.results.push(result.unwrap())
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
                if ui.add(egui::widgets::Button::new("×").small()).clicked() {
                    self.flags = ViewFlags::Remove(index);
                }
                if ui.add(egui::widgets::Button::new("…").small()).clicked() {
                    self.flags = ViewFlags::Modal(index);
                }
            });
        })
    }


    fn shape_chooser(&mut self, ui: &mut egui::Ui) {
        for (row, item) in self.shapes.chunks(3).enumerate() {
            ui.horizontal( |ui| {
                for (index, button) in item.iter().enumerate() { 
                    if ui.add(egui::widgets::Button::new(button.name())
                                .min_size(egui::vec2(STEP * 3., STEP * 1.))).clicked() {
                        self.current = index + row;
                    }
                }    
            });
        }
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
                    ui.text_edit_singleline(txt).labelled_by(ui.label("Коэфицент").id);
                });
            }
            shapes::FormElement::NoElement => {
                ui.label(" ");
            }
        }
    }
}



impl Default for Calculator {
    fn default()->Self {
        Self {
            shapes: vec![
                Box::new(shapes::AreaCircle::default()),
                Box::new(shapes::AreaRectangle::default()),
                Box::new(shapes::AreaCylinder::default())
            ],
            current: 0,
            flags: ViewFlags::NoFlags,
            results: Vec::new(),
            sum: 0.
        }
    }
}

impl eframe::App for Calculator{
    fn update (&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.shape_chooser(ui);
            ui.label(self.shapes[self.current].name());
            let shape = self.shapes[self.current].form_state();
            shape_input(shape, ui);
            if ui.button("Расчитать").clicked() {
                self.calculate();
            }
            if ui.button("Сброс").clicked() {
                self.clear();
            }
            if ui.button("Скопировать").clicked() {
                let mut clipboard = Clipboard::new().unwrap();
                println!("{:?}", clipboard.set_text(format!("{}", self.sum)));
            }
            ui.label(format!("Итого {}", self.sum));
            self.calculation_list(ui);
            match self.flags {
                ViewFlags::Modal(index) => {
                    let modal = Modal::new(ctx, "edit_modal");
                    modal.show( |ui| {
                        modal.title(ui, "Редактировать");
                        modal.frame(ui, |ui| {
                            let shape = self.results[index].get_state().form_state();
                            shape_input(shape, ui);
                        });
                        modal.buttons(ui, |ui| {
                            if modal.button(ui, "Отмена").clicked() {
                                self.flags = ViewFlags::NoFlags;
                            }
                            if modal.button(ui, "Сохранить").clicked() {
                                let shape = self.results[index].get_state();
                                let result = shape.calculate();
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
