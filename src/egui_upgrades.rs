use egui_macroquad::egui::{self, Id, Vec2};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CostRow {
    pub item: String,
    pub required: u32,
    pub available: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpgradeRow {
    pub name: String,
    pub level: u32,
    pub effect: String,
    pub cost: Vec<CostRow>,
    pub affordable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpgradesView {
    pub pinned: bool,
    pub ship_name: String,
    pub artwork: Option<String>,
    pub purchase_count: u32,
    pub rows: Vec<UpgradeRow>,
    pub latest_operation: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpgradesUiAction {
    TogglePin,
    Purchase(usize),
}

pub fn draw(ctx: &egui::Context, screen: [f32; 2], view: &UpgradesView) -> Vec<UpgradesUiAction> {
    ctx.set_style(egui::Style::default());
    let mut actions = Vec::new();
    let height = 640.0_f32.min((screen[1] - 96.0).max(320.0));

    egui::Window::new("Ship Upgrades")
        .id(Id::new("ship-upgrades-window"))
        .default_pos(egui::pos2(64.0, 48.0))
        .default_size(Vec2::new(940.0_f32.min(screen[0] - 64.0), height))
        .min_width(680.0)
        .resizable(true)
        .collapsible(true)
        .vscroll(true)
        .show(ctx, |ui| draw_contents(ui, view, &mut actions));

    actions
}

fn draw_contents(ui: &mut egui::Ui, view: &UpgradesView, actions: &mut Vec<UpgradesUiAction>) {
    let style = ui.style_mut();
    style
        .text_styles
        .insert(egui::TextStyle::Heading, egui::FontId::proportional(24.0));
    style
        .text_styles
        .insert(egui::TextStyle::Body, egui::FontId::proportional(17.0));
    style
        .text_styles
        .insert(egui::TextStyle::Button, egui::FontId::proportional(16.0));
    style
        .text_styles
        .insert(egui::TextStyle::Small, egui::FontId::proportional(14.0));

    if crate::egui_pin::toggle(ui, view.pinned) {
        actions.push(UpgradesUiAction::TogglePin);
    }
    crate::egui_art::preview(ui, view.artwork.as_deref(), Vec2::new(260.0, 150.0));
    ui.horizontal(|ui| {
        ui.heading(&view.ship_name);
        ui.separator();
        ui.label("Ship upgrades");
    });
    ui.small(format!(
        "Upgrade actions buy up to {} level(s) with the current key modifier.",
        view.purchase_count
    ));
    ui.separator();

    egui::Grid::new("ship-upgrades-table")
        .num_columns(4)
        .striped(true)
        .min_col_width(64.0)
        .spacing([16.0, 10.0])
        .show(ui, |ui| {
            ui.add_sized(
                [220.0, 24.0],
                egui::Label::new(egui::RichText::new("Upgrade").strong()),
            );
            ui.add_sized(
                [58.0, 24.0],
                egui::Label::new(egui::RichText::new("Level").strong()),
            );
            ui.add_sized(
                [210.0, 24.0],
                egui::Label::new(egui::RichText::new("Next cost").strong()),
            );
            ui.add_sized(
                [112.0, 24.0],
                egui::Label::new(egui::RichText::new("Action").strong()),
            );
            ui.end_row();

            for (index, row) in view.rows.iter().enumerate() {
                ui.vertical(|ui| {
                    ui.set_min_width(220.0);
                    ui.strong(&row.name);
                    ui.small(&row.effect);
                });
                ui.add_sized([58.0, 28.0], egui::Label::new(row.level.to_string()));
                ui.vertical(|ui| {
                    ui.set_min_width(210.0);
                    for cost in &row.cost {
                        let text = format!("{} {}/{}", cost.item, cost.available, cost.required);
                        if cost.available >= cost.required {
                            ui.label(text);
                        } else {
                            ui.colored_label(ui.visuals().warn_fg_color, text);
                        }
                    }
                });
                if ui
                    .add_enabled(
                        row.affordable,
                        egui::Button::new(format!("Upgrade +{}", view.purchase_count))
                            .min_size(Vec2::new(112.0, 30.0)),
                    )
                    .on_disabled_hover_text("Required components are missing")
                    .clicked()
                {
                    actions.push(UpgradesUiAction::Purchase(index));
                }
                ui.end_row();
            }
        });

    if let Some(operation) = &view.latest_operation {
        ui.separator();
        ui.small(operation);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn purchase_actions_preserve_upgrade_identity() {
        assert_ne!(UpgradesUiAction::Purchase(0), UpgradesUiAction::Purchase(1));
    }
}
