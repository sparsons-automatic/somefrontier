use egui_macroquad::egui::{self, Id, Vec2};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContextMenuEntryView {
    pub label: String,
    pub detail: String,
    pub enabled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContextMenuView {
    pub position: [f32; 2],
    pub heading: Option<String>,
    pub entries: Vec<ContextMenuEntryView>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextMenuOutcome {
    Choose(usize),
    Dismiss,
}

pub fn draw(ctx: &egui::Context, view: &ContextMenuView) -> Option<ContextMenuOutcome> {
    ctx.set_style(egui::Style::default());
    let mut chosen = None;
    let area = egui::Area::new(Id::new("world-context-menu"))
        .order(egui::Order::Foreground)
        .fixed_pos(egui::pos2(view.position[0], view.position[1]))
        .constrain(true)
        .show(ctx, |ui| {
            egui::Frame::menu(ui.style()).show(ui, |ui| {
                ui.set_min_width(190.0);
                if let Some(heading) = &view.heading {
                    ui.strong(heading);
                    ui.separator();
                }
                for (index, entry) in view.entries.iter().enumerate() {
                    let response = ui
                        .add_enabled(
                            entry.enabled,
                            egui::Button::new(&entry.label)
                                .frame(false)
                                .min_size(Vec2::new(190.0, 0.0)),
                        )
                        .on_hover_text(&entry.detail);
                    if response.clicked() {
                        chosen = Some(ContextMenuOutcome::Choose(index));
                    }
                }
            });
        });

    if chosen.is_some() {
        return chosen;
    }

    let clicked_outside = ctx.input(|input| {
        input.pointer.any_click()
            && input
                .pointer
                .interact_pos()
                .is_none_or(|position| !area.response.rect.contains(position))
    });
    clicked_outside.then_some(ContextMenuOutcome::Dismiss)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_menu_entries_preserve_disabled_explanations() {
        let entry = ContextMenuEntryView {
            label: "Scan".to_string(),
            detail: "Requires survey drone".to_string(),
            enabled: false,
        };

        assert!(!entry.enabled);
        assert_eq!(entry.detail, "Requires survey drone");
    }
}
