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
            self.selected_index = (self.selected_index + items.len() - 1) % items.len();
        } else if next {
            self.selected_index = (self.selected_index + 1) % items.len();
        }
    }

    fn show_item(
        &mut self,
        ui: &mut egui::Ui,
        index: usize,
        text: &str,
        orientation: Orientation,
    ) -> (bool, bool) {
        let is_selected = index == self.selected_index;
        let font_id = egui::TextStyle::Monospace.resolve(ui.style());

        // Calculate dimensions
        let size = match orientation {
            Orientation::Vertical => {
                egui::vec2(ui.available_width(), ui.spacing().interact_size.y + 8.0)
            }
            Orientation::Horizontal => {
                let galley = ui.fonts(|f| {
                    f.layout_no_wrap(text.to_string(), font_id.clone(), ui.visuals().text_color())
                });
                egui::vec2(
                    galley.size().x
                        + Self::ENTRY_WIDTH_PADDING
                        + Self::CLOSE_BUTTON_SIZE
                        + Self::CLOSE_BUTTON_MARGIN * 2.0,
                    galley.size().y * Self::ENTRY_HEIGHT_MULTIPLIER,
                )
            }
        };

        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

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

        // Draw background layers
        let bg = ui.visuals().window_fill.linear_multiply(1.2);
        ui.painter().rect_filled(rect, 0.0, bg);

        if response.hovered() {
            ui.painter()
                .rect_filled(rect, 0.0, ui.visuals().widgets.hovered.bg_fill);
        }

        if is_selected {
            ui.painter()
                .rect_filled(rect, 0.0, ui.visuals().widgets.active.bg_fill);

            // Draw strip
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

        // Draw close button
        if close_response.hovered() {
            ui.painter().circle_filled(
                close_center,
                Self::CLOSE_BUTTON_SIZE / 2.0,
                ui.visuals().widgets.hovered.bg_fill,
            );
        }

        let close_color = if close_response.hovered() {
            ui.visuals().warn_fg_color
        } else {
            ui.visuals().text_color().linear_multiply(0.6)
        };

        let half = Self::CLOSE_BUTTON_SIZE * 0.25;
        ui.painter().line_segment(
            [
                egui::pos2(close_center.x - half, close_center.y - half),
                egui::pos2(close_center.x + half, close_center.y + half),
            ],
            egui::Stroke::new(1.5, close_color),
        );
        ui.painter().line_segment(
            [
                egui::pos2(close_center.x + half, close_center.y - half),
                egui::pos2(close_center.x - half, close_center.y + half),
            ],
            egui::Stroke::new(1.5, close_color),
        );

        // Draw text
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
            .text(text_pos, align, text, font_id, text_color);

        (response.clicked(), close_response.clicked())
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
            if self.selected_index >= items.len() && !items.is_empty() {
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
