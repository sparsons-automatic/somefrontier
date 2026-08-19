use egui_macroquad::egui::{self, Id, Vec2};

#[derive(Clone, Debug, PartialEq)]
pub struct MiningRow {
    pub item: String,
    pub keep: u32,
    pub status: String,
    pub richness: String,
    pub active: bool,
    pub enabled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanetPaneView {
    pub pinned: bool,
    pub title: String,
    pub artwork: Option<String>,
    pub content_id: String,
    pub summary: String,
    pub ownership: Option<String>,
    pub scan_level: u8,
    pub max_scan_level: u8,
    pub mineables: Vec<String>,
    pub hazards: Vec<String>,
    pub hazard_effects: Option<String>,
    pub richness: String,
    pub proximity: String,
    pub in_range: bool,
    pub is_orbiting: bool,
    pub orbit_pending: bool,
    pub scan_enabled: bool,
    pub scan_label: String,
    pub basic_drones: u32,
    pub improved_drones: u32,
    pub drone_return: String,
    pub latest_operation: Option<String>,
    pub keep_click_step: i32,
    pub mining: Vec<MiningRow>,
    pub mining_revealed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlanetUiAction {
    TogglePin,
    EnterOrbit,
    LaunchScan,
    AdjustMiningKeep { row: usize, direction: i32 },
    QueueMining { row: usize },
}

pub fn draw(ctx: &egui::Context, screen: [f32; 2], view: &PlanetPaneView) -> Vec<PlanetUiAction> {
    ctx.set_style(egui::Style::default());
    let mut actions = Vec::new();
    let height = 600.0_f32.min((screen[1] - 96.0).max(280.0));

    let window_id = Id::new("planet-workspace-window");
    crate::egui_pane::show_window(
        ctx,
        window_id,
        egui::Window::new("Planet")
            .id(window_id)
            .default_pos(egui::pos2(16.0, 48.0))
            .default_size(Vec2::new(380.0, height))
            .min_width(300.0)
            .resizable(true)
            .collapsible(true),
        |ui| {
            crate::egui_pane::vertical_scroll(ui, "planet-content-scroll", |ui| {
                draw_planet_contents(ui, view, &mut actions);
            });
        },
    );

    actions
}

fn draw_planet_contents(
    ui: &mut egui::Ui,
    view: &PlanetPaneView,
    actions: &mut Vec<PlanetUiAction>,
) {
    if crate::egui_pin::toggle(ui, view.pinned) {
        actions.push(PlanetUiAction::TogglePin);
    }
    crate::egui_art::preview(ui, view.artwork.as_deref(), Vec2::new(240.0, 160.0));
    ui.heading(&view.title);
    ui.small(&view.content_id);
    ui.add(
        egui::ProgressBar::new(view.scan_level as f32 / view.max_scan_level.max(1) as f32).text(
            format!("Scan level {}/{}", view.scan_level, view.max_scan_level),
        ),
    );
    ui.horizontal(|ui| {
        if ui
            .add_enabled(
                view.in_range && !view.is_orbiting && !view.orbit_pending,
                egui::Button::new(if view.is_orbiting {
                    "In orbit"
                } else if view.orbit_pending {
                    "Entering orbit"
                } else {
                    "Orbit"
                }),
            )
            .clicked()
        {
            actions.push(PlanetUiAction::EnterOrbit);
        }
        if let Some(enabled) = scan_button_enabled(view.in_range, view.scan_enabled) {
            if ui
                .add_enabled(enabled, egui::Button::new(&view.scan_label))
                .clicked()
            {
                actions.push(PlanetUiAction::LaunchScan);
            }
        }
    });

    ui.separator();
    ui.label(&view.summary);
    egui::Grid::new("planet-facts-grid")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            fact_row(
                ui,
                "Ownership",
                view.ownership.as_deref().unwrap_or("Unknown"),
            );
            fact_row(ui, "Proximity", &view.proximity);
            fact_row(ui, "Richness", &view.richness);
            fact_row(
                ui,
                "Survey drones",
                &format!(
                    "{} basic / {} improved",
                    view.basic_drones, view.improved_drones
                ),
            );
            fact_row(ui, "Drone return", &view.drone_return);
        });

    egui::CollapsingHeader::new("Mineable resources")
        .default_open(true)
        .show(ui, |ui| {
            if view.mineables.is_empty() {
                ui.label("Unknown until composition is surveyed");
            } else {
                for item in &view.mineables {
                    ui.label(item);
                }
            }
        });
    egui::CollapsingHeader::new("Surface readings")
        .default_open(true)
        .show(ui, |ui| {
            if view.hazards.is_empty() {
                ui.label("No survey telemetry");
            } else {
                for hazard in &view.hazards {
                    ui.label(hazard);
                }
            }
            if let Some(effects) = &view.hazard_effects {
                ui.label(effects);
            }
        });

    ui.separator();
    ui.heading("Mining");
    if view.mining_revealed {
        draw_mining_grid(ui, view, actions);
    } else {
        ui.label("Survey composition before mining actions become available.");
    }
    if let Some(latest) = &view.latest_operation {
        ui.separator();
        ui.label(format!("Latest: {latest}"));
    }
}

fn scan_button_enabled(in_range: bool, scan_enabled: bool) -> Option<bool> {
    in_range.then_some(scan_enabled)
}

fn draw_mining_grid(ui: &mut egui::Ui, view: &PlanetPaneView, actions: &mut Vec<PlanetUiAction>) {
    egui::Grid::new("planet-mining-grid")
        .striped(true)
        .min_col_width(44.0)
        .show(ui, |ui| {
            for header in ["Item", "Keep", "Status", "Richness", "Active"] {
                ui.strong(header);
            }
            ui.end_row();

            for (row_index, row) in view.mining.iter().enumerate() {
                let item_response = ui.add_enabled(
                    row.enabled,
                    egui::Label::new(mining_item_label(row)).sense(egui::Sense::click()),
                );
                let mut row_rect = item_response.rect;
                if item_response.clicked() {
                    actions.push(PlanetUiAction::QueueMining { row: row_index });
                }
                let keep_response = ui.horizontal(|ui| {
                    if mining_keep_button(
                        ui,
                        "−",
                        view.keep_click_step,
                        row.enabled,
                        "Decrease Keep (Shift: 10, Control: 20)",
                    )
                    .clicked()
                    {
                        actions.push(PlanetUiAction::AdjustMiningKeep {
                            row: row_index,
                            direction: -1,
                        });
                    }
                    ui.label(row.keep.to_string());
                    if mining_keep_button(
                        ui,
                        "+",
                        view.keep_click_step,
                        row.enabled,
                        "Increase Keep (Shift: 10, Control: 20)",
                    )
                    .clicked()
                    {
                        actions.push(PlanetUiAction::AdjustMiningKeep {
                            row: row_index,
                            direction: 1,
                        });
                    }
                });
                row_rect = row_rect.union(keep_response.response.rect);
                let status_response = ui.add_enabled(row.enabled, egui::Label::new(&row.status));
                row_rect = row_rect.union(status_response.rect);
                let richness_response = ui.label(&row.richness);
                row_rect = row_rect.union(richness_response.rect);
                let active_response = if row.active {
                    ui.colored_label(mining_active_color(), "●")
                } else {
                    ui.weak("—")
                };
                row_rect = row_rect.union(active_response.rect);
                if row.active {
                    ui.painter().rect_filled(
                        row_rect.expand2(egui::vec2(4.0, 2.0)),
                        2.0,
                        mining_active_tint(),
                    );
                }
                ui.end_row();
            }
        });
}

fn mining_active_color() -> egui::Color32 {
    egui::Color32::from_rgb(104, 211, 153)
}

fn mining_active_tint() -> egui::Color32 {
    egui::Color32::from_rgba_premultiplied(45, 92, 67, 22)
}

fn mining_item_label(row: &MiningRow) -> egui::RichText {
    if row.active {
        egui::RichText::new(format!("▶ {}", row.item))
            .strong()
            .color(mining_active_color())
    } else {
        egui::RichText::new(&row.item)
    }
}

fn mining_keep_button(
    ui: &mut egui::Ui,
    icon: &str,
    step: i32,
    enabled: bool,
    tooltip: &str,
) -> egui::Response {
    let label = if step > 1 {
        format!("{icon}{step}")
    } else {
        icon.to_string()
    };
    ui.add_enabled(enabled, egui::Button::new(label).small())
        .on_hover_text(tooltip)
}

fn fact_row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.label(label);
    ui.label(value);
    ui.end_row();
}

#[cfg(test)]
mod tests {
    use super::scan_button_enabled;

    #[test]
    fn planet_window_height_fits_small_screens() {
        let screen = [640.0, 320.0];
        let height = 600.0_f32.min((screen[1] - 96.0_f32).max(280.0_f32));

        assert_eq!(height, 280.0);
    }

    #[test]
    fn scan_button_is_hidden_until_the_ship_is_in_range() {
        assert_eq!(scan_button_enabled(false, false), None);
        assert_eq!(scan_button_enabled(false, true), None);
        assert_eq!(scan_button_enabled(true, false), Some(false));
        assert_eq!(scan_button_enabled(true, true), Some(true));
    }
}
