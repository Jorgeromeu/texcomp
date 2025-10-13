use crate::Selector;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TexCompApp {
    items: Vec<String>,
    selected: usize,
    secondary_selected: Option<usize>,
}

impl Default for TexCompApp {
    fn default() -> Self {
        Self {
            items: vec!["First".to_owned(), "Second".to_owned(), "Third".to_owned()],
            selected: 0,
            secondary_selected: None,
        }
    }
}

impl TexCompApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TexCompApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut sel = Selector::new(
            &mut self.items,
            &mut self.selected,
            &mut self.secondary_selected,
        );
        sel.handle_input(ctx);

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("TexComp");
            ui.separator();
            sel.show(ui);
        });
            
        let is_being_dragged = ctx.input(|i| !i.raw.hovered_files.is_empty());
        let is_dropped = ctx.input(|i| !i.raw.dropped_files.is_empty());

        egui::CentralPanel::default().show(ctx, |ui| {

            // show on file drag
            if is_being_dragged {
                ui.centered_and_justified(|ui| {
                    ui.label(egui::RichText::new("Drop files to add").size(18.0));
                });
            }

            // show on file drop
            if is_dropped {
                let files = ctx.input(|i| i.raw.dropped_files.clone());
                for file in files {
                    self.items.push(file.name);
                }

            }


        });

    }
}
