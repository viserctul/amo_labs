use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Lab1 {
    #[serde(skip)]
    error: crate::ErrorWindow,
    values: Values,
    open_panel: Panel,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
struct Values {
    linear: Linear,
    conditional: Conditional,
    cyclic: Cyclic,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct Linear {
    a: f64,
    b: f64,
    c: f64,
    s: f64,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct Conditional {
    d: f64,
    h: f64,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct Cyclic {
    a: Vec<f64>,
    b: Vec<f64>,
    #[serde(skip)]
    a_str: crate::StringVecEdit,
    #[serde(skip)]
    b_str: crate::StringVecEdit,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
enum Panel {
    #[default]
    Linear,
    Conditional,
    Cyclic,
}

impl Lab1 {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.storage
            .and_then(|st| eframe::get_value(st, "lab_1"))
            .unwrap_or_default()
    }
}

impl eframe::App for Lab1 {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "lab_1", self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::top_menu(ctx, _frame, &mut self.error, &mut self.values, "lab_1.toml");

        egui::CentralPanel::default().show(ctx, |ui| {
            let Self {
                values, open_panel, ..
            } = self;

            ui.horizontal_wrapped(|ui| {
                ui.selectable_value(open_panel, Panel::Linear, "Linear");
                ui.selectable_value(open_panel, Panel::Conditional, "Conditional");
                ui.selectable_value(open_panel, Panel::Cyclic, "Cyclic");
                ui.separator();
                match open_panel {
                    Panel::Linear => egui::reset_button(ui, &mut values.linear),
                    Panel::Conditional => egui::reset_button(ui, &mut values.conditional),
                    Panel::Cyclic => egui::reset_button(ui, &mut values.cyclic),
                }
            });
            ui.separator();

            match open_panel {
                Panel::Linear => {
                    ui.group(|ui| {
                        let Linear { a, b, c, s } = &mut values.linear;
                        egui::Grid::new("grid").striped(true).show(ui, |ui| {
                            ui.label("a:");
                            ui.add(egui::DragValue::new(a).speed(0.1));
                            ui.end_row();
                            ui.label("b:");
                            ui.add(egui::DragValue::new(b).speed(0.1));
                            ui.end_row();
                            ui.label("c:");
                            ui.add(egui::DragValue::new(c).speed(0.1));
                            ui.end_row();
                            ui.label("s:");
                            ui.add(egui::DragValue::new(s).speed(0.1));
                        });
                    });

                    let Linear { a, b, c, s } = values.linear;
                    let y1 = ((a + b) / (a - b) + (c + s) / (c - s)).powi(2);
                    ui.label(format!("Y1: {}", y1));
                }

                Panel::Conditional => {
                    ui.group(|ui| {
                        let Conditional { d, h } = &mut values.conditional;
                        egui::Grid::new("grid").striped(true).show(ui, |ui| {
                            ui.label("d:");
                            ui.add(egui::DragValue::new(d).speed(0.1));
                            ui.end_row();
                            ui.label("h:");
                            ui.add(egui::DragValue::new(h).speed(0.1));
                        });
                    });

                    let Conditional { d, h } = values.conditional;
                    let y = if d > 0. {
                        PI * d * h + (PI + 23. * d).sqrt()
                    } else {
                        (PI * d.abs()).sqrt() / 129. * h
                    };
                    ui.label(format!("d > 0: {}", d > 0.));
                    ui.label(format!("y: {}", y));
                }

                Panel::Cyclic => {
                    ui.group(|ui| {
                        let Cyclic { a, b, a_str, b_str } = &mut values.cyclic;
                        egui::Grid::new("grid")
                            .num_columns(2)
                            .striped(true)
                            .show(ui, |ui| {
                                ui.label("a:");
                                crate::parse_vec(ui, a, a_str);
                                ui.end_row();
                                ui.label("b:");
                                crate::parse_vec(ui, b, b_str);
                            });
                    });

                    let Cyclic { a, b, .. } = &values.cyclic;
                    let f: f64 = a
                        .iter()
                        .map(|a| a + b.iter().map(|b| (a + b) / (a * b)).sum::<f64>())
                        .sum();
                    ui.label(format!("f: {}", f));
                }
            }
        });

        self.error.show(ctx);
    }
}
