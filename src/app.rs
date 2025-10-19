use crate::asset::{Asset, AssetEnum};
use crate::image::viewer::ImageViewerWidget;
use crate::selector::Selector;
use egui_toast::{Toast, ToastOptions, Toasts};

pub struct App {
    items: Vec<AssetEnum>,
    image_viewer: ImageViewerWidget,
    selector: Selector,
    sidebar_open: bool,
    help_open: bool,
    toasts: Toasts,
}

impl Default for App {
    fn default() -> Self {
        Self {
            items: vec![],
            toasts: Toasts::new()
                .anchor(egui::Align2::RIGHT_BOTTOM, (10.0, 10.0))
                .direction(egui::Direction::BottomUp),
            image_viewer: ImageViewerWidget::default(),
            selector: Selector::new(),
            sidebar_open: false,
            help_open: false,
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.style_mut(|style| {
            // No dropshadows
            style.visuals.window_shadow = egui::epaint::Shadow::NONE;

            // Make window title font smaller
            style.text_styles.insert(
                egui::TextStyle::Name("window_title".into()),
                egui::FontId::new(12.0, egui::FontFamily::Proportional),
            );
        });

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
        // check if fies are dropped
        let is_dropped = ctx.input(|i| !i.raw.dropped_files.is_empty());
        if !is_dropped {
            return;
        }

        let files = ctx.input(|i| i.raw.dropped_files.clone());
        for file in files {
            let loaded_asset = AssetEnum::from_dropped_file(ctx, &file);

            match loaded_asset {
                Err(e) => {
                    self.error(&format!(
                        "Failed to load asset from file: {}. Error: {}",
                        file.name, e
                    ));
                }
                Ok(asset) => {
                    self.items.push(asset);
                    self.selector.selected_index = self.items.len() - 1;
                }
            }
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
        let asset_opt = self.items.get_mut(self.selector.selected_index);

        // handle case of no asset
        let Some(asset) = asset_opt else {
            ui.centered_and_justified(|ui| {
                ui.label(egui::RichText::new("📁 Drop Files to view").size(18.0));
            });
            return;
        };

        // Show Viewer
        match asset {
            AssetEnum::Image(image_asset) => {
                self.image_viewer.show_viewer(ui, image_asset);
            }
            AssetEnum::Model(model) => {
                ui.centered_and_justified(|ui| {
                    ui.label(format!(
                        "verts: {}, indices: {}",
                        model.verts.len(),
                        model.indices.len()
                    ));
                });
            }
        }

        // show info window
        window.show(ctx, |ui| match asset {
            AssetEnum::Image(image_asset) => {
                self.image_viewer.show_info(ui);
            }
            AssetEnum::Model(model) => {}
        });
    }

    pub fn show_footer(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.toggle_value(&mut self.help_open, "Help");

            ui.label(egui::RichText::new("v0.1.0").small());
            ui.hyperlink_to(
                egui::RichText::new("GitHub").small(),
                "https://github.com/Jorgeromeu/texcomp",
            );
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_file_drop(ctx);

        // handle input
        if ctx.input(|i| i.key_pressed(egui::Key::S)) {
            self.sidebar_open = !self.sidebar_open;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Questionmark)) {
            self.help_open = !self.help_open;
        }

        if self.help_open {
            egui::Window::new("Help")
                .collapsible(false)
                .resizable(true)
                .open(&mut self.help_open)
                .show(ctx, |ui| {
                    ui.label("📁 Drop files into the window to view");
                    ui.separator();
                    ui.heading("Keyboard Shortcuts");
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("S").monospace().strong());
                        ui.label("Toggle between sidebar and bottom bar");
                    });
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("?").monospace().strong());
                        ui.label("Toggle help");
                    });
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("←/→").monospace().strong());
                        ui.label("Switch between assets");
                    });
                    self.image_viewer.show_help(ui);
                });
        }

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

        egui::SidePanel::left("side_panel")
            .resizable(true)
            .show_animated(ctx, self.sidebar_open, |ui| {
                // selector ui
                self.selector
                    .show(ui, &mut self.items, |item| &item.get_id());
                // Push footer to bottom
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(5.0);
                    self.show_footer(ui);
                    ui.add_space(5.0);
                });
            });

        egui::TopBottomPanel::bottom("bottom_bar").show_animated(ctx, !self.sidebar_open, |ui| {
            ui.horizontal(|ui| {
                self.selector
                    .show_horizontal(ui, &mut self.items, |item| &item.get_id());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    self.show_footer(ui);
                });
            });
        });

        self.toasts.show(ctx);
    }
}
