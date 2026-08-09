use egui_macroquad::egui::{self, Id, Vec2};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContractRow {
    pub name: String,
    pub route: String,
    pub status: String,
    pub deadline: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContractDetail {
    pub name: String,
    pub description: String,
    pub status: String,
    pub deadline: String,
    pub progress: String,
    pub origin: String,
    pub destination: String,
    pub reward: String,
    pub requirement: String,
    pub focus_enabled: bool,
    pub focus_status: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContractsView {
    pub pinned: bool,
    pub rows: Vec<ContractRow>,
    pub selected: Option<usize>,
    pub detail: Option<ContractDetail>,
    pub latest_operation: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContractsUiAction {
    TogglePin,
    Select(usize),
    FocusOrigin(usize),
}

pub fn draw(ctx: &egui::Context, screen: [f32; 2], view: &ContractsView) -> Vec<ContractsUiAction> {
    ctx.set_style(egui::Style::default());
    let mut actions = Vec::new();
    let height = 620.0_f32.min((screen[1] - 96.0).max(320.0));

    egui::Window::new("Contracts")
        .id(Id::new("contracts-workspace-window"))
        .default_pos(egui::pos2(56.0, 48.0))
        .default_size(Vec2::new(820.0_f32.min(screen[0] - 64.0), height))
        .min_width(560.0)
        .resizable(true)
        .collapsible(true)
        .show(ctx, |ui| draw_contents(ui, view, &mut actions));

    actions
}

fn draw_contents(ui: &mut egui::Ui, view: &ContractsView, actions: &mut Vec<ContractsUiAction>) {
    if crate::egui_pin::toggle(ui, view.pinned) {
        actions.push(ContractsUiAction::TogglePin);
    }
    ui.horizontal(|ui| {
        ui.heading("Active contracts");
        ui.separator();
        ui.label(format!("{}/3 active", view.rows.len()));
    });
    ui.small("Select a contract to inspect it. Double-click to focus its origin station.");
    ui.separator();

    let pane_height = ui.available_height().max(220.0);
    ui.horizontal_top(|ui| {
        contract_list(ui, view, pane_height, actions);
        ui.separator();
        contract_detail(ui, view, pane_height, actions);
    });
}

fn contract_list(
    ui: &mut egui::Ui,
    view: &ContractsView,
    height: f32,
    actions: &mut Vec<ContractsUiAction>,
) {
    ui.vertical(|ui| {
        ui.set_width(330.0);
        egui::ScrollArea::vertical()
            .id_salt("active-contract-list")
            .max_height(height)
            .show(ui, |ui| {
                if view.rows.is_empty() {
                    ui.label("No active contracts");
                    ui.small("Accept work from a station contract desk.");
                    return;
                }

                for (index, row) in view.rows.iter().enumerate() {
                    let response = ui.add_sized(
                        [ui.available_width(), 64.0],
                        egui::Button::new(format!(
                            "{}\n{} · {}\n{}",
                            row.name, row.status, row.deadline, row.route
                        ))
                        .selected(view.selected == Some(index)),
                    );
                    if response.double_clicked() {
                        actions.push(ContractsUiAction::FocusOrigin(index));
                    } else if response.clicked() {
                        actions.push(ContractsUiAction::Select(index));
                    }
                }
            });
    });
}

fn contract_detail(
    ui: &mut egui::Ui,
    view: &ContractsView,
    height: f32,
    actions: &mut Vec<ContractsUiAction>,
) {
    ui.vertical(|ui| {
        ui.set_min_width(260.0);
        egui::ScrollArea::vertical()
            .id_salt("active-contract-detail")
            .max_height(height)
            .show(ui, |ui| {
                let Some(detail) = &view.detail else {
                    ui.heading("Contract details");
                    ui.label("Select an active contract from the list.");
                    if let Some(operation) = &view.latest_operation {
                        ui.separator();
                        ui.small(operation);
                    }
                    return;
                };

                ui.heading(&detail.name);
                ui.strong(format!("{} · {}", detail.status, detail.deadline));
                ui.label(&detail.description);
                ui.add_space(6.0);
                egui::Grid::new("active-contract-facts")
                    .num_columns(2)
                    .striped(true)
                    .show(ui, |ui| {
                        fact_row(ui, "Progress", &detail.progress);
                        fact_row(ui, "Origin", &detail.origin);
                        fact_row(ui, "Destination", &detail.destination);
                        fact_row(ui, "Reward", &detail.reward);
                        fact_row(ui, "Requirement", &detail.requirement);
                    });
                ui.add_space(8.0);
                if ui
                    .add_enabled(
                        detail.focus_enabled,
                        egui::Button::new("Focus origin station"),
                    )
                    .on_disabled_hover_text(&detail.focus_status)
                    .clicked()
                {
                    if let Some(index) = view.selected {
                        actions.push(ContractsUiAction::FocusOrigin(index));
                    }
                }
                ui.small(&detail.focus_status);
                if let Some(operation) = &view.latest_operation {
                    ui.separator();
                    ui.small(operation);
                }
            });
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
    fn contract_actions_keep_selection_and_focus_distinct() {
        assert_ne!(
            ContractsUiAction::Select(1),
            ContractsUiAction::FocusOrigin(1)
        );
    }
}
