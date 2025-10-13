pub struct Selector<'a, T> {
    items: &'a mut Vec<T>,
    selected: &'a mut usize,
    secondary_selected: &'a mut Option<usize>,
}

impl<'a, T> Selector<'a, T> {
    pub fn new(
        items: &'a mut Vec<T>,
        selected: &'a mut usize,
        secondary_selected: &'a mut Option<usize>,
    ) -> Self {
        Self {
            items,
            selected,
            secondary_selected,
        }
    }

    pub fn handle_input(&mut self, ui: &egui::Context) {
        if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            *self.selected = (*self.selected + self.items.len() - 1) % self.items.len();
        } else if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            *self.selected = (*self.selected + 1) % self.items.len();
        }
    }

    pub fn get_selected(&self) -> Option<&T> {
        self.items.get(*self.selected)
    }

    pub fn show(&mut self, ui: &mut egui::Ui, name_fn: impl Fn(&T) -> &str) {
        ui.scope(|ui| {
            ui.spacing_mut().item_spacing.y = 0.0;

            for (i, item) in self.items.iter().enumerate() {
                let is_selected = i == *self.selected;
                let is_secondary_selected = Some(i) == *self.secondary_selected;

                let desired_size = egui::vec2(ui.available_width(), ui.spacing().interact_size.y);
                let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

                // Highlight on hover
                if response.hovered() {
                    ui.painter()
                        .rect_filled(rect, 0, ui.visuals().widgets.hovered.bg_fill);
                }

                // get strip color
                let strip_color = if is_selected {
                    Some(ui.visuals().selection.stroke.color)
                } else if is_secondary_selected {
                    Some(egui::Color32::from_rgb(220, 100, 100))
                } else {
                    None
                };

                if let Some(color) = strip_color {
                    // Darker background
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
                    *self.selected = i;
                }

                // right click -> toggle secondary selection
                if response.secondary_clicked() {
                    *self.secondary_selected = if self.secondary_selected.is_none() {
                        Some(i)
                    } else {
                        None
                    };
                }

                // middle click -> remove item
                if response.middle_clicked() {
                    self.items.remove(i);
                    if *self.selected >= self.items.len() && !self.items.is_empty() {
                        *self.selected = self.items.len() - 1;
                    }
                    break;
                }
            }
        });
    }
}
