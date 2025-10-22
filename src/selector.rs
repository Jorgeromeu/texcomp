use crate::egui_ext::InputStateExt;

pub struct Selector {
    pub selected_index: usize,
}

#[derive(Copy, Clone)]
enum Orientation {
    Vertical,
    Horizontal,
}

impl Selector {
    // Layout constants
    const STRIP_WIDTH: f32 = 2.0;
    const ENTRY_HEIGHT_MULTIPLIER: f32 = 2.0;
    const ENTRY_WIDTH_PADDING: f32 = 16.0;
    const CLOSE_BUTTON_SIZE: f32 = 14.0;
    const CLOSE_BUTTON_MARGIN: f32 = 6.0;

    pub fn new() -> Self {
        Self { selected_index: 0 }
    }

    fn handle_keyboard<T>(&mut self, ui: &mut egui::Ui, items: &[T]) {
        if items.is_empty() {
            return;
        }

        let prev = ui.input(|i| i.any_pressed(&[egui::Key::ArrowUp, egui::Key::ArrowLeft]));
        let next = ui.input(|i| i.any_pressed(&[egui::Key::ArrowDown, egui::Key::ArrowRight]));

        if prev {
            self.selected_index = self.selected_index.saturating_sub(1);
        } else if next {
            self.selected_index = (self.selected_index + 1).min(items.len() - 1);
        }
    }

    fn font_id(&self, ui: &egui::Ui) -> egui::FontId {
        let mut font = egui::TextStyle::Monospace.resolve(ui.style());
        font.size = 10.0;
        font
    }

    fn calculate_size(&self, ui: &egui::Ui, text: &str, orientation: Orientation) -> egui::Vec2 {
        match orientation {
            Orientation::Vertical => {
                egui::vec2(ui.available_width(), ui.spacing().interact_size.y + 8.0)
            }
            Orientation::Horizontal => {
                let galley = ui.fonts(|f| {
                    f.layout_no_wrap(
                        text.to_string(),
                        self.font_id(ui),
                        ui.visuals().text_color(),
                    )
                });
                egui::vec2(
                    galley.size().x
                        + Self::ENTRY_WIDTH_PADDING
                        + Self::CLOSE_BUTTON_SIZE
                        + Self::CLOSE_BUTTON_MARGIN * 2.0,
                    galley.size().y * Self::ENTRY_HEIGHT_MULTIPLIER,
                )
            }
        }
    }

    fn draw_background(
        &self,
        ui: &egui::Ui,
        rect: egui::Rect,
        is_selected: bool,
        is_hovered: bool,
    ) {
        let bg = ui.visuals().window_fill.linear_multiply(1.2);
        ui.painter().rect_filled(rect, 0.0, bg);

        if is_hovered {
            ui.painter()
                .rect_filled(rect, 0.0, ui.visuals().widgets.hovered.bg_fill);
        }

        if is_selected {
            ui.painter()
                .rect_filled(rect, 0.0, ui.visuals().widgets.active.bg_fill);
        }
    }

    fn draw_selection_strip(&self, ui: &egui::Ui, rect: egui::Rect, orientation: Orientation) {
        let strip = match orientation {
            Orientation::Vertical => egui::Rect::from_min_max(
                egui::pos2(rect.right() - Self::STRIP_WIDTH, rect.top()),
                rect.max,
            ),
            Orientation::Horizontal => egui::Rect::from_min_max(
                rect.min,
                egui::pos2(rect.right(), rect.top() + Self::STRIP_WIDTH),
            ),
        };
        ui.painter()
            .rect_filled(strip, 0.0, ui.visuals().selection.stroke.color);
    }

    fn draw_close_button(&self, ui: &egui::Ui, center: egui::Pos2, is_hovered: bool) {
        if is_hovered {
            ui.painter().circle_filled(
                center,
                Self::CLOSE_BUTTON_SIZE / 2.0,
                ui.visuals().widgets.hovered.bg_fill,
            );
        }

        let color = if is_hovered {
            ui.visuals().warn_fg_color
        } else {
            ui.visuals().text_color().linear_multiply(0.6)
        };

        let half = Self::CLOSE_BUTTON_SIZE * 0.25;
        let stroke = egui::Stroke::new(1.5, color);

        ui.painter().line_segment(
            [
                egui::pos2(center.x - half, center.y - half),
                egui::pos2(center.x + half, center.y + half),
            ],
            stroke,
        );
        ui.painter().line_segment(
            [
                egui::pos2(center.x + half, center.y - half),
                egui::pos2(center.x - half, center.y + half),
            ],
            stroke,
        );
    }

    fn draw_text(
        &self,
        ui: &egui::Ui,
        content_rect: egui::Rect,
        text: &str,
        is_selected: bool,
        orientation: Orientation,
    ) {
        let text_color = if is_selected {
            ui.visuals().selection.stroke.color
        } else {
            ui.visuals().text_color()
        };

        let (text_pos, align) = match orientation {
            Orientation::Vertical => (
                egui::pos2(
                    content_rect.left() + ui.spacing().item_spacing.x,
                    content_rect.center().y,
                ),
                egui::Align2::LEFT_CENTER,
            ),
            Orientation::Horizontal => {
                let text_area_right = content_rect.right()
                    - Self::CLOSE_BUTTON_SIZE
                    - Self::CLOSE_BUTTON_MARGIN * 2.0;
                (
                    egui::pos2(
                        (content_rect.left() + text_area_right) / 2.0,
                        content_rect.center().y,
                    ),
                    egui::Align2::CENTER_CENTER,
                )
            }
        };

        ui.painter()
            .text(text_pos, align, text, self.font_id(ui), text_color);
    }

    fn show_item(
        &mut self,
        ui: &mut egui::Ui,
        index: usize,
        text: &str,
        orientation: Orientation,
    ) -> (bool, bool) {
        let is_selected = index == self.selected_index;
        let size = self.calculate_size(ui, text, orientation);
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

        // Check for middle click
        let middle_clicked = response.middle_clicked();

        // Content area (excluding strip for horizontal)
        let content_rect = match orientation {
            Orientation::Vertical => rect,
            Orientation::Horizontal => egui::Rect::from_min_max(
                egui::pos2(rect.left(), rect.top() + Self::STRIP_WIDTH),
                rect.max,
            ),
        };

        // Close button
        let close_center = egui::pos2(
            content_rect.right() - Self::CLOSE_BUTTON_SIZE / 2.0 - Self::CLOSE_BUTTON_MARGIN,
            content_rect.center().y,
        );
        let close_response = ui.interact(
            egui::Rect::from_center_size(
                close_center,
                egui::vec2(Self::CLOSE_BUTTON_SIZE, Self::CLOSE_BUTTON_SIZE),
            ),
            ui.id().with(("close", index)),
            egui::Sense::click(),
        );

        // Draw everything
        self.draw_background(ui, rect, is_selected, response.hovered());

        if is_selected {
            self.draw_selection_strip(ui, rect, orientation);
        }

        self.draw_close_button(ui, close_center, close_response.hovered());
        self.draw_text(ui, content_rect, text, is_selected, orientation);

        // Return: (item clicked, should close)
        (
            response.clicked(),
            close_response.clicked() || middle_clicked,
        )
    }

    fn show_impl<T>(
        &mut self,
        ui: &mut egui::Ui,
        items: &mut Vec<T>,
        name_fn: impl Fn(&T) -> &str,
        orientation: Orientation,
    ) {
        self.handle_keyboard(ui, items);

        let mut removed_index = None;

        for (i, item) in items.iter().enumerate() {
            let (clicked, close_clicked) = self.show_item(ui, i, name_fn(item), orientation);

            if clicked {
                self.selected_index = i;
            }
            if close_clicked {
                removed_index = Some(i);
            }
        }

        if let Some(idx) = removed_index {
            items.remove(idx);
            if !items.is_empty() && self.selected_index >= items.len() {
                self.selected_index = items.len() - 1;
            }
        }
    }

    pub fn show<T>(&mut self, ui: &mut egui::Ui, items: &mut Vec<T>, name_fn: impl Fn(&T) -> &str) {
        ui.scope(|ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            self.show_impl(ui, items, name_fn, Orientation::Vertical);
        });
    }

    pub fn show_horizontal<T>(
        &mut self,
        ui: &mut egui::Ui,
        items: &mut Vec<T>,
        name_fn: impl Fn(&T) -> &str,
    ) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            self.show_impl(ui, items, name_fn, Orientation::Horizontal);
        });
    }
}

impl Default for Selector {
    fn default() -> Self {
        Self::new()
    }
}
