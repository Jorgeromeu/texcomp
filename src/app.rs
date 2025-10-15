use crate::asset::{Asset, AssetEnum};
use crate::image::viewer::ImageViewerWidget;
use crate::selector::Selector;
use crate::viewer::ViewerWidget;
use egui_toast::{Toast, ToastOptions, Toasts};
use three_d_asset::io::load_and_deserialize_async;

pub struct NamedAsset {
    pub name: String,
    pub asset: AssetEnum,
}

pub struct TexCompApp {
    items: Vec<NamedAsset>,
    toasts: Toasts,
    image_viewer: ImageViewerWidget,
    selector: Selector,
}

impl Default for TexCompApp {
    fn default() -> Self {
        Self {
            items: vec![],
            toasts: Toasts::new()
                .anchor(egui::Align2::RIGHT_BOTTOM, (10.0, 10.0))
                .direction(egui::Direction::BottomUp),
            image_viewer: ImageViewerWidget::default(),
            selector: Selector::new(),
        }
    }
}

impl TexCompApp {
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

        if !is_dropped {
            return;
        }

        let files = ctx.input(|i| i.raw.dropped_files.clone());
        for file in files {
            let loaded_asset = AssetEnum::from_dropped_file(ctx, &file);

            let Ok(asset) = loaded_asset else {
                self.error(&format!("Failed to load asset from file: {}", file.name));
                continue;
            };

            let name = file.name;
            let named_asset = NamedAsset { name, asset };
            self.items.push(named_asset);
            self.selector.selected_index = self.items.len() - 1;
        }
    }

    pub fn show_drop_overlay(&mut self, ctx: &egui::Context, ui: &egui::Ui) {
        let hovering = ctx.input(|i| !i.raw.hovered_files.is_empty());
        if !hovering {
            return;
        }

        let target_rect = ui.max_rect();
        egui::Area::new(egui::Id::new("drop overlay"))
            .order(egui::Order::Foreground)
            .interactable(false)
            .show(ctx, |ui| {
                let size = egui::vec2(170.0, 40.0);
                let rect = egui::Rect::from_center_size(target_rect.center(), size);

                // Draw background with default panel color
                let bg_color = ui.visuals().window_fill;
                ui.painter().rect_filled(rect, 8.0, bg_color);

                // Draw text
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "📁 Drop file here",
                    egui::FontId::proportional(18.0),
                    ui.visuals().strong_text_color(),
                );
            });
    }

    pub fn show_viewer(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        // make info window
        let spacing = 8.0;
        let panel_rect = ui.max_rect();
        let window = egui::Window::new(egui::RichText::new("Info").size(12.0))
            .collapsible(true)
            .resizable(false)
            .pivot(egui::Align2::RIGHT_TOP)
            .fixed_pos(egui::pos2(
                panel_rect.right() - spacing,
                panel_rect.top() + spacing,
            ))
            .default_width(200.0);

        // get selected asset
        let asset_opt = self.items.get(self.selector.selected_index);

        // handle case of no asset
        let Some(asset) = asset_opt else {
            ui.centered_and_justified(|ui| {
                ui.label(egui::RichText::new("📁 Drop Files to view").size(18.0));
            });
            return;
        };

        match &asset.asset {
            AssetEnum::Image(image_asset) => {
                self.image_viewer.show_viewer(ui, image_asset);
                window.show(ctx, |ui| {
                    self.image_viewer.show_info(ui);
                });
            }
        }
    }
}

impl eframe::App for TexCompApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            // title
            ui.vertical_centered(|ui| {
                ui.add_space(5.0);
                ui.heading(egui::RichText::new("TexComp").monospace());
                ui.add_space(5.0);
            });

            // selector ui
            self.selector.show(ui, &mut self.items, |item| &item.name);

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

        egui::CentralPanel::default()
            .frame(egui::Frame {
                inner_margin: egui::Margin::ZERO,
                outer_margin: egui::Margin::ZERO,
                ..Default::default()
            })
            .show(ctx, |ui| {
                self.show_viewer(ui, ctx);
                self.show_drop_overlay(ctx, ui);
            });

        self.toasts.show(ctx);
    }
}
