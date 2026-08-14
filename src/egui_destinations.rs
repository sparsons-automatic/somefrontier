use egui_macroquad::egui::{self, Id, Vec2};

#[derive(Clone, Debug, PartialEq)]
pub struct DestinationCandidate {
    pub name: String,
    pub artwork: Option<String>,
    pub content_id: String,
    pub system: String,
    pub summary: String,
    pub distance: String,
    pub scan_status: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RouteStopState {
    Completed,
    Traveling,
    Mining,
    Blocked,
    Skipped,
    Paused,
    Pending,
}

impl RouteStopState {
    fn label(self) -> &'static str {
        match self {
            Self::Completed => "Reached",
            Self::Traveling => "Traveling",
            Self::Mining => "Mining",
            Self::Blocked => "Blocked",
            Self::Skipped => "Skipped",
            Self::Paused => "Paused",
            Self::Pending => "Pending",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RouteStopView {
    pub name: String,
    pub system: String,
    pub distance: String,
    pub state: RouteStopState,
    pub mining_progress: Option<String>,
    pub reason: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DestinationsView {
    pub pinned: bool,
    pub map_open: bool,
    pub selected: Option<DestinationCandidate>,
    pub route: Vec<RouteStopView>,
    pub route_active: bool,
    pub route_started: bool,
    pub mining_mode_unlocked: bool,
    pub mining_mode_enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DestinationsUiAction {
    TogglePin,
    FlySelected,
    AddSelected,
    Remove(usize),
    MoveUp(usize),
    MoveDown(usize),
    Activate,
    Pause,
    ToggleMiningMode,
    Clear,
}

pub fn draw(
    ctx: &egui::Context,
    screen: [f32; 2],
    view: &DestinationsView,
) -> Vec<DestinationsUiAction> {
    ctx.set_style(egui::Style::default());
    let mut actions = Vec::new();
    let height = 620.0_f32.min((screen[1] - 80.0).max(300.0));
    let x = (screen[0] - 382.0).max(16.0);

    egui::Window::new("Destinations")
        .id(Id::new("starmap-destinations-window"))
        .default_pos(egui::pos2(x, 40.0))
        .default_size(Vec2::new(350.0, height))
        .min_width(300.0)
        .resizable(true)
        .collapsible(true)
        .show(ctx, |ui| draw_contents(ui, view, &mut actions));

    actions
}

fn draw_contents(
    ui: &mut egui::Ui,
    view: &DestinationsView,
    actions: &mut Vec<DestinationsUiAction>,
) {
    if crate::egui_pin::toggle(ui, view.pinned) {
        actions.push(DestinationsUiAction::TogglePin);
    }
    ui.heading("Navigation console");
    ui.small(if view.map_open {
        "Select a planet on the starmap, then fly now or add it to the itinerary."
    } else {
        "Press M to reopen the starmap and choose another destination."
    });
    ui.separator();

    ui.strong("Selected destination");
    if let Some(selected) = &view.selected {
        crate::egui_art::preview(ui, selected.artwork.as_deref(), Vec2::new(220.0, 140.0));
        ui.heading(&selected.name);
        ui.small(&selected.content_id);
        egui::Grid::new("destination-selection-facts")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                fact_row(ui, "System", &selected.system);
                fact_row(ui, "Distance", &selected.distance);
                fact_row(ui, "Survey", &selected.scan_status);
            });
        ui.label(&selected.summary);
        ui.horizontal(|ui| {
            if ui.button("Fly Now").clicked() {
                actions.push(DestinationsUiAction::FlySelected);
            }
            if ui.button("Add Stop").clicked() {
                actions.push(DestinationsUiAction::AddSelected);
            }
        });
    } else {
        ui.label("Click a visible planet on the starmap.");
    }

    ui.separator();
    ui.horizontal(|ui| {
        ui.strong("Itinerary");
        ui.label(format!("{} stop(s)", view.route.len()));
    });
    if view.mining_mode_unlocked {
        let label = if view.mining_mode_enabled {
            "Mining holds: enabled"
        } else {
            "Mining holds: disabled"
        };
        if ui
            .add_enabled(!view.route_active, egui::Button::new(label).small())
            .on_hover_text("Use configured planet ore work as quotas at itinerary stops")
            .clicked()
        {
            actions.push(DestinationsUiAction::ToggleMiningMode);
        }
    } else {
        ui.small("Mining holds locked · research Automated Extraction Routes");
    }
    let route_controls_enabled = !view.route_active;
    egui::ScrollArea::vertical()
        .id_salt("destination-itinerary-list")
        .max_height(270.0)
        .show(ui, |ui| {
            if view.route.is_empty() {
                ui.label("No stops added yet.");
                return;
            }

            for (index, stop) in view.route.iter().enumerate() {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.strong(format!("{}. {}", index + 1, stop.name));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(stop.state.label());
                        });
                    });
                    ui.small(format!("{} · {}", stop.system, stop.distance));
                    if let Some(progress) = &stop.mining_progress {
                        ui.small(progress);
                    }
                    if let Some(reason) = &stop.reason {
                        ui.small(reason);
                    }
                    ui.horizontal(|ui| {
                        if ui
                            .add_enabled(
                                route_controls_enabled && index > 0,
                                egui::Button::new("↑").small(),
                            )
                            .on_hover_text("Move stop earlier")
                            .clicked()
                        {
                            actions.push(DestinationsUiAction::MoveUp(index));
                        }
                        if ui
                            .add_enabled(
                                route_controls_enabled && index + 1 < view.route.len(),
                                egui::Button::new("↓").small(),
                            )
                            .on_hover_text("Move stop later")
                            .clicked()
                        {
                            actions.push(DestinationsUiAction::MoveDown(index));
                        }
                        if ui
                            .add_enabled(
                                route_controls_enabled,
                                egui::Button::new("Remove").small(),
                            )
                            .clicked()
                        {
                            actions.push(DestinationsUiAction::Remove(index));
                        }
                    });
                });
                ui.add_space(3.0);
            }
        });

    ui.separator();
    ui.horizontal(|ui| {
        if view.route_active {
            if ui.button("Pause Route").clicked() {
                actions.push(DestinationsUiAction::Pause);
            }
        } else if ui
            .add_enabled(
                !view.route.is_empty(),
                egui::Button::new(if view.route_started {
                    "Resume Route"
                } else {
                    "Activate Route"
                }),
            )
            .clicked()
        {
            actions.push(DestinationsUiAction::Activate);
        }
        if ui
            .add_enabled(!view.route.is_empty(), egui::Button::new("Clear"))
            .clicked()
        {
            actions.push(DestinationsUiAction::Clear);
        }
    });
}

fn fact_row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.label(label);
    ui.label(value);
    ui.end_row();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_stop_labels_distinguish_progress_states() {
        assert_eq!(RouteStopState::Completed.label(), "Reached");
        assert_eq!(RouteStopState::Traveling.label(), "Traveling");
        assert_eq!(RouteStopState::Mining.label(), "Mining");
        assert_eq!(RouteStopState::Blocked.label(), "Blocked");
        assert_eq!(RouteStopState::Skipped.label(), "Skipped");
        assert_eq!(RouteStopState::Paused.label(), "Paused");
        assert_eq!(RouteStopState::Pending.label(), "Pending");
    }
}
