use egui_macroquad::egui::{self, Align2, Id, Vec2};

#[derive(Clone, Debug, PartialEq)]
pub struct LoadingView<'a> {
    pub label: &'a str,
    pub detail: Option<&'a str>,
    pub progress: Option<f32>,
    pub count: Option<&'a str>,
    pub opacity: f32,
    pub panel_center_y: f32,
}

pub fn draw(ctx: &egui::Context, screen: [f32; 2], view: &LoadingView<'_>) {
    ctx.set_style(egui::Style::default());
    let width = (screen[0] - 48.0).clamp(320.0, 520.0);
    let height = 142.0;
    let center_offset = view.panel_center_y - screen[1] * 0.5;

    egui::Window::new("Loading")
        .id(Id::new("startup-loading-panel"))
        .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, center_offset))
        .interactable(false)
        .movable(false)
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .fixed_size(Vec2::new(width, height))
        .show(ctx, |ui| {
            ui.set_opacity(view.opacity.clamp(0.0, 1.0));
            ui.vertical_centered(|ui| {
                ui.horizontal_centered(|ui| {
                    ui.add(egui::Spinner::new().size(18.0));
                    ui.add(egui::Label::new(egui::RichText::new(view.label).heading()).wrap());
                });
                if let Some(detail) = view.detail {
                    ui.add_space(4.0);
                    ui.label(detail);
                }
                ui.add_space(8.0);
                if let Some(progress) = view.progress {
                    ui.add(
                        egui::ProgressBar::new(progress.clamp(0.0, 1.0))
                            .show_percentage()
                            .animate(true),
                    );
                } else {
                    ui.add(egui::ProgressBar::new(0.0).animate(true));
                }
                if let Some(count) = view.count {
                    ui.small(count);
                }
            });
        });
}
