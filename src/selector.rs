pub struct Selector {
    pub selected_index: usize,
}

impl Selector {
    pub fn new() -> Self {
        Self { selected_index: 0 }
    }

    pub fn show<T>(&mut self, ui: &mut egui::Ui, items: &mut Vec<T>, name_fn: impl Fn(&T) -> &str) {
        // handle keyboard input
        if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            self.selected_index = (self.selected_index + items.len() - 1) % items.len();
        } else if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            self.selected_index = (self.selected_index + 1) % items.len();
        }

        ui.scope(|ui| {
            ui.spacing_mut().item_spacing.y = 0.0;

            for (i, item) in items.iter().enumerate() {
                let is_selected = i == self.selected_index;

                let desired_size =
                    egui::vec2(ui.available_width(), ui.spacing().interact_size.y + 8.0);
                let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

                // Default background (slightly lighter than panel background)
                let default_bg = ui.visuals().window_fill.linear_multiply(1.2);
                ui.painter().rect_filled(rect, 0.0, default_bg);

                // Highlight on hover
                if response.hovered() {
                    ui.painter()
                        .rect_filled(rect, 0, ui.visuals().widgets.hovered.bg_fill);
                }

                // get strip color
                let strip_color = if is_selected {
                    Some(ui.visuals().selection.stroke.color)
                } else {
                    None
                };

                if let Some(color) = strip_color {
                    // Darker background for selected items
                    ui.painter()
                        .rect_filled(rect, 0.0, ui.visuals().widgets.active.bg_fill);

                    // Colored strip on right
                    let strip_rect = egui::Rect::from_min_max(
                        egui::pos2(rect.right() - 4.0, rect.top()),
                        egui::pos2(rect.right(), rect.bottom()),
                    );
                    ui.painter().rect_filled(strip_rect, 0.0, color);
                }

                // Draw text
                let text_color = if is_selected {
                    ui.visuals().selection.stroke.color
                } else {
                    ui.visuals().text_color()
                };

                ui.painter().text(
                    rect.left_center() + egui::vec2(ui.spacing().item_spacing.x, 0.0),
                    egui::Align2::LEFT_CENTER,
                    name_fn(item),
                    egui::TextStyle::Body.resolve(ui.style()),
                    text_color,
                );

                // left click -> select item
                if response.clicked() {
                    self.selected_index = i;
                }

                // middle click -> remove item
                if response.middle_clicked() {
                    items.remove(i);
                    if self.selected_index >= items.len() && !items.is_empty() {
                        self.selected_index = items.len() - 1;
                    }
                    break;
                }
            }
        });
    }
}
