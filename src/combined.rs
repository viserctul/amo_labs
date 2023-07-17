use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Combined {
    open_panel: Panel,
    #[serde(skip)]
    lab1: crate::Lab1,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
enum Panel {
    #[default]
    Lab1,
}

impl Combined {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        cc.storage
            .map(|st| {
                let mut comb: Self = eframe::get_value(st, eframe::APP_KEY).unwrap_or_default();
                comb.lab1 = crate::Lab1::new(cc);
                comb
            })
            .unwrap_or_default()
    }
}

impl eframe::App for Combined {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
        self.lab1.save(storage);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { open_panel, .. } = self;

        egui::SidePanel::left("side_panel")
            .resizable(false)
            .exact_width(40.0)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.selectable_value(open_panel, Panel::Lab1, "Lab1");
                });
            });

        match open_panel {
            Panel::Lab1 => self.lab1.update(ctx, _frame),
        }
    }
}
