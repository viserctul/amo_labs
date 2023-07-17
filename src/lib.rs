#![warn(clippy::all, rust_2018_idioms)]

mod lab_1;
pub use lab_1::Lab1;

mod combined;
pub use combined::Combined;

#[derive(Debug, Default)]
pub struct ErrorWindow {
    text: String,
    open: bool,
}

impl ErrorWindow {
    fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("Error")
            .open(&mut self.open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::default())
            .show(ctx, |ui| {
                ui.label(&self.text);
            });
    }
    fn close(&mut self) {
        self.open = false;
    }
    fn consume<T, E: ToString>(&mut self, res: Result<T, E>) -> Result<T, E> {
        match &res {
            Ok(_) => self.close(),
            Err(e) => {
                self.text = e.to_string();
                self.open = true;
            }
        }
        res
    }
}

pub fn top_menu<V, P>(
    ctx: &egui::Context,
    _frame: &mut eframe::Frame,
    error: &mut ErrorWindow,
    values: &mut V,
    _path: P,
) where
    V: serde::Serialize + serde::de::DeserializeOwned + Default,
    P: AsRef<std::path::Path>,
{
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(ui, "File", |ui| {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    if ui.button("Read from file").clicked() {
                        if let Ok(res) = error.consume(
                            std::fs::read_to_string(&_path)
                                .map_err(|e| e.to_string())
                                .and_then(|buf| toml::from_str(&buf).map_err(|e| e.to_string())),
                        ) {
                            *values = res;
                        }
                    };
                    if ui.button("Write to file").clicked() {
                        let res = toml::to_string(values).unwrap();
                        let _ = error.consume(std::fs::write(&_path, res.as_bytes()));
                    }
                    if ui.button("Read from clipboard").clicked() {
                        if let Ok(res) = error.consume(
                            arboard::Clipboard::new()
                                .and_then(|mut cb| cb.get_text())
                                .map_err(|e| e.to_string())
                                .and_then(|buf| toml::from_str(&buf).map_err(|e| e.to_string())),
                        ) {
                            *values = res;
                        }
                    }
                }
                if ui.button("Write to clipboard").clicked() {
                    let res = toml::to_string(values).unwrap();
                    ui.output_mut(|o| o.copied_text = res);
                    error.close();
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                }
            });

            egui::menu::menu_button(ui, "Reset", |ui| {
                if ui.button("Values").clicked() {
                    *values = Default::default();
                }
                if ui.button("Egui").clicked() {
                    ui.ctx().memory_mut(|mem| *mem = Default::default());
                    ui.close_menu();
                }
            });

            egui::widgets::global_dark_light_mode_switch(ui);
        });
    });
}

#[derive(Debug, Default, PartialEq)]
pub struct StringVecEdit {
    edit: String,
    curr: String,
}

pub fn parse_vec<T>(ui: &mut egui::Ui, x: &mut Vec<T>, x_str: &mut StringVecEdit) -> bool
where
    T: std::str::FromStr + std::string::ToString,
{
    if ui.text_edit_singleline(&mut x_str.edit).lost_focus() {
        *x = x_str
            .edit
            .split([',', ' '])
            .flat_map(|s| s.parse())
            .collect();
        x_str.curr.clear()
    }
    let changed = !x.is_empty() && x_str.curr.is_empty();
    if changed {
        let s = x.iter().map(T::to_string).collect::<Vec<_>>().join(", ");
        x_str.edit.clone_from(&s);
        x_str.curr = s;
    }
    changed
}
