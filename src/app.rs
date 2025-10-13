use crate::{AssetType, Selector, load_asset};
use egui_toast::{Toast, ToastOptions, Toasts};

pub struct NamedTexture {
    pub name: String,
    pub texture: egui::TextureHandle,
}

pub struct TexCompApp {
    items: Vec<NamedTexture>,
    selected: usize,
    secondary_selected: Option<usize>,
    toasts: Toasts,
}

impl Default for TexCompApp {
    fn default() -> Self {
        Self {
            items: vec![],
            selected: 0,
            secondary_selected: None,
            toasts: Toasts::new()
                .anchor(egui::Align2::RIGHT_BOTTOM, (10.0, 10.0))
                .direction(egui::Direction::BottomUp),
        }
    }
}

impl TexCompApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    pub fn error(&mut self, message: &str) {
        let toast = Toast {
            kind: egui_toast::ToastKind::Error,
            text: egui::WidgetText::from(message),
            options: ToastOptions::default().duration_in_seconds(3.0),
            style: Default::default(),
        };
        self.toasts.add(toast);
    }

    pub fn handle_file_drop(&mut self, ctx: &egui::Context) {
        let is_dropped = ctx.input(|i| !i.raw.dropped_files.is_empty());

        if is_dropped {
            let files = ctx.input(|i| i.raw.dropped_files.clone());
            for file in files {
                match load_asset(ctx, &file) {
                    Ok(asset) => match asset {
                        AssetType::Gltf(named_texture) | AssetType::Image(named_texture) => {
                            self.items.push(named_texture);
                        }
                    },
                    Err(err) => {
                        self.error(&format!(
                            "Failed to load asset from file: {}: {}",
                            &file.name, err
                        ));
                    }
                }
            }
        }
    }

    pub fn show_file_drag(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let is_being_dragged = ctx.input(|i| !i.raw.hovered_files.is_empty());

        if is_being_dragged {
            ui.centered_and_justified(|ui| {
                ui.label(egui::RichText::new("Drop files to add").size(18.0));
            });
        }
    }

    pub fn show_selected_image(&self, ui: &mut egui::Ui) {
        if let Some(item) = self.items.get(self.selected) {
            ui.add(egui::Image::new(&item.texture).max_size(ui.available_size()));
        }
    }
}

impl eframe::App for TexCompApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // image selector
        let mut selector = Selector::new(
            &mut self.items,
            &mut self.selected,
            &mut self.secondary_selected,
        );
        selector.handle_input(ctx);

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            // title
            ui.vertical_centered(|ui| {
                ui.add_space(5.0);
                ui.heading(egui::RichText::new("TexComp").monospace());
                ui.add_space(5.0);
            });

            // textures list
            selector.show(ui, |item| &item.name);

            // Push footer to bottom
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("v0.1.0").small());
                    ui.hyperlink_to(
                        egui::RichText::new("GitHub").small(),
                        "https://github.com/Jorgeromeu/texcomp",
                    );
                });
                ui.add_space(5.0);
            });
        });

        self.handle_file_drop(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_file_drag(ctx, ui);

            if self.items.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.label(egui::RichText::new("Drag Image or GLB").size(18.0));
                });
            } else {
                self.show_selected_image(ui);
            }
        });

        self.toasts.show(ctx);
    }
}
