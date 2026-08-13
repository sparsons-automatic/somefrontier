use egui_macroquad::egui::{self, Id, Vec2};

#[derive(Clone, Debug, PartialEq)]
pub struct ShipRow {
    pub callsign: String,
    pub hull_name: String,
    pub system: String,
    pub condition: String,
    pub active: bool,
    pub resolved: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CargoRow {
    pub item_id: String,
    pub name: String,
    pub count: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MiningJobOption {
    pub id: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MiningRouteLegView {
    pub planet: String,
    pub resource: String,
    pub quota: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DroneSmeltRow {
    pub name: String,
    pub queued: u32,
    pub progress: f32,
    pub unlocked: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MiningDroneView {
    pub profile: String,
    pub destinations: Vec<MiningJobOption>,
    pub resources: Vec<MiningJobOption>,
    pub selected_planet: Option<String>,
    pub selected_resource: Option<String>,
    pub quota: u32,
    pub max_quota: u32,
    pub route: Vec<MiningRouteLegView>,
    pub add_leg_enabled: bool,
    pub route_edit_status: Option<String>,
    pub deliver_to_ship: bool,
    pub delivery_ships: Vec<MiningJobOption>,
    pub selected_delivery_ship: Option<String>,
    pub job_status: Option<String>,
    pub start_enabled: bool,
    pub pause_enabled: bool,
    pub paused: bool,
    pub recall_enabled: bool,
    pub cancel_enabled: bool,
    pub smelter_installed: bool,
    pub smelt_rows: Vec<DroneSmeltRow>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectedShip {
    pub callsign: String,
    pub hull_name: String,
    pub artwork: Option<String>,
    pub system: String,
    pub distance: String,
    pub hull: String,
    pub shields: String,
    pub energy: String,
    pub weapon_energy: String,
    pub cargo: String,
    pub turrets: usize,
    pub active: bool,
    pub switch_enabled: bool,
    pub share_inventory: bool,
    pub linked_inventory_status: String,
    pub transfer_available: bool,
    pub transfer_status: String,
    pub cargo_rows: Vec<CargoRow>,
    pub mining_drone: Option<MiningDroneView>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FleetPaneView {
    pub pinned: bool,
    pub ships: Vec<ShipRow>,
    pub selected: Option<usize>,
    pub selected_ship: Option<SelectedShip>,
    pub active_cargo: Vec<CargoRow>,
    pub transfer_quantity: u32,
    pub quantity_step: u32,
    pub credits: u32,
    pub latest_operation: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FleetUiAction {
    TogglePin,
    SelectShip(usize),
    QueueDroneSmelt(usize),
    SwitchControl,
    SetShareInventory(bool),
    AdjustTransferQuantity(i32),
    TransferToSelected(String),
    TransferToActive(String),
    TakeAllFromSelected,
    SelectMiningPlanet(String),
    SelectMiningResource(String),
    AdjustMiningQuota(i32),
    AddMiningRouteLeg,
    RemoveMiningRouteLeg(usize),
    ClearMiningRoute,
    SetMiningDeliverToShip(bool),
    SelectMiningDeliveryShip(String),
    StartMiningJob,
    ToggleMiningJobPause,
    RecallMiningJob,
    CancelMiningJob,
}

pub fn draw(ctx: &egui::Context, screen: [f32; 2], view: &FleetPaneView) -> Vec<FleetUiAction> {
    ctx.set_style(egui::Style::default());
    let mut actions = Vec::new();
    let height = 720.0_f32.min((screen[1] - 96.0).max(320.0));

    egui::Window::new("Fleet")
        .id(Id::new("fleet-workspace-window"))
        .default_pos(egui::pos2(48.0, 48.0))
        .default_size(Vec2::new(760.0, height))
        .min_width(440.0)
        .resizable(true)
        .collapsible(true)
        .vscroll(true)
        .show(ctx, |ui| draw_contents(ui, view, &mut actions));

    actions
}

fn draw_contents(ui: &mut egui::Ui, view: &FleetPaneView, actions: &mut Vec<FleetUiAction>) {
    if crate::egui_pin::toggle(ui, view.pinned) {
        actions.push(FleetUiAction::TogglePin);
    }
    ui.horizontal(|ui| {
        ui.heading("Owned ships");
        ui.separator();
        ui.label(format!("{} hulls", view.ships.len()));
        ui.separator();
        ui.label(format!("Credits {}", view.credits));
    });
    ui.small(
        "Click a ship to inspect its inventory. Double-click an available ship to switch control.",
    );

    if view.ships.is_empty() {
        ui.label("No owned ships are available.");
    } else {
        for (index, ship) in view.ships.iter().enumerate() {
            let active = if ship.active { " · ACTIVE" } else { "" };
            let unavailable = if ship.resolved {
                ""
            } else {
                " · CONTENT MISSING"
            };
            let label = format!(
                "{}{}\n{} · {}\n{}{}",
                ship.callsign, active, ship.hull_name, ship.system, ship.condition, unavailable
            );
            let response = ui.add_sized(
                [ui.available_width(), 64.0],
                egui::Button::new(label).selected(view.selected == Some(index)),
            );
            if response.clicked() {
                actions.push(FleetUiAction::SelectShip(index));
            }
            if response.double_clicked() && !ship.active && ship.resolved {
                actions.push(FleetUiAction::SwitchControl);
            }
        }
    }

    let Some(selected) = &view.selected_ship else {
        return;
    };
    ui.separator();
    crate::egui_art::preview(ui, selected.artwork.as_deref(), Vec2::new(280.0, 170.0));
    ui.heading(&selected.callsign);
    egui::Grid::new("fleet-selected-facts")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            fact_row(ui, "Hull", &selected.hull_name);
            fact_row(ui, "System", &selected.system);
            fact_row(ui, "Distance", &selected.distance);
            fact_row(ui, "Hull condition", &selected.hull);
            fact_row(ui, "Shields", &selected.shields);
            fact_row(ui, "Drive energy", &selected.energy);
            fact_row(ui, "Weapon capacitor", &selected.weapon_energy);
            fact_row(ui, "Cargo", &selected.cargo);
            fact_row(ui, "Turret banks", &selected.turrets.to_string());
        });
    if ui
        .add_enabled(
            selected.switch_enabled,
            egui::Button::new(if selected.active {
                "Currently controlled"
            } else {
                "Switch control"
            }),
        )
        .clicked()
    {
        actions.push(FleetUiAction::SwitchControl);
    }
    let mut share_inventory = selected.share_inventory;
    if ui
        .checkbox(&mut share_inventory, "Share inventory with nearby fleet")
        .changed()
    {
        actions.push(FleetUiAction::SetShareInventory(share_inventory));
    }
    ui.small(&selected.linked_inventory_status);

    if let Some(drone) = &selected.mining_drone {
        ui.separator();
        ui.heading("Autonomous mining");
        ui.small(&drone.profile);
        if let Some(status) = &drone.job_status {
            ui.label(status);
        }
        ui.horizontal(|ui| {
            ui.label("Destination");
            let selected_label = drone
                .selected_planet
                .as_ref()
                .and_then(|id| drone.destinations.iter().find(|option| &option.id == id))
                .map(|option| option.label.as_str())
                .unwrap_or("Select scanned planet");
            egui::ComboBox::from_id_salt("fleet-mining-destination")
                .selected_text(selected_label)
                .show_ui(ui, |ui| {
                    for option in &drone.destinations {
                        if ui
                            .selectable_label(
                                drone.selected_planet.as_ref() == Some(&option.id),
                                &option.label,
                            )
                            .clicked()
                        {
                            actions.push(FleetUiAction::SelectMiningPlanet(option.id.clone()));
                        }
                    }
                });
        });
        ui.horizontal(|ui| {
            ui.label("Resource");
            let selected_label = drone
                .selected_resource
                .as_ref()
                .and_then(|id| drone.resources.iter().find(|option| &option.id == id))
                .map(|option| option.label.as_str())
                .unwrap_or("Select resource");
            egui::ComboBox::from_id_salt("fleet-mining-resource")
                .selected_text(selected_label)
                .show_ui(ui, |ui| {
                    for option in &drone.resources {
                        if ui
                            .selectable_label(
                                drone.selected_resource.as_ref() == Some(&option.id),
                                &option.label,
                            )
                            .clicked()
                        {
                            actions.push(FleetUiAction::SelectMiningResource(option.id.clone()));
                        }
                    }
                });
        });
        ui.horizontal(|ui| {
            ui.label("Quota");
            if ui.button("−").clicked() {
                actions.push(FleetUiAction::AdjustMiningQuota(-1));
            }
            ui.label(format!("{} / {}", drone.quota, drone.max_quota));
            if ui.button("+").clicked() {
                actions.push(FleetUiAction::AdjustMiningQuota(1));
            }
        });
        ui.horizontal(|ui| {
            if ui
                .add_enabled(drone.add_leg_enabled, egui::Button::new("Add route leg"))
                .clicked()
            {
                actions.push(FleetUiAction::AddMiningRouteLeg);
            }
            if ui
                .add_enabled(!drone.route.is_empty(), egui::Button::new("Clear route"))
                .clicked()
            {
                actions.push(FleetUiAction::ClearMiningRoute);
            }
        });
        if let Some(status) = &drone.route_edit_status {
            ui.small(status);
        }
        if drone.route.is_empty() {
            ui.small("No route legs yet. Add the selected planet, ore, and quota.");
        } else {
            ui.strong("Mining route");
            for (index, leg) in drone.route.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "{}. {} · {} ×{}",
                        index + 1,
                        leg.planet,
                        leg.resource,
                        leg.quota
                    ));
                    if ui.small_button("Remove").clicked() {
                        actions.push(FleetUiAction::RemoveMiningRouteLeg(index));
                    }
                });
            }
        }
        let mut auto_transfer = drone.deliver_to_ship;
        if ui
            .checkbox(
                &mut auto_transfer,
                "Auto-transfer inventory to a fleet ship",
            )
            .changed()
        {
            actions.push(FleetUiAction::SetMiningDeliverToShip(auto_transfer));
        }
        if drone.deliver_to_ship {
            ui.horizontal(|ui| {
                ui.label("Deliver to");
                let selected_label = drone
                    .selected_delivery_ship
                    .as_ref()
                    .and_then(|id| drone.delivery_ships.iter().find(|option| &option.id == id))
                    .map(|option| option.label.as_str())
                    .unwrap_or("Select fleet ship");
                egui::ComboBox::from_id_salt("fleet-mining-delivery-ship")
                    .selected_text(selected_label)
                    .show_ui(ui, |ui| {
                        for option in &drone.delivery_ships {
                            if ui
                                .selectable_label(
                                    drone.selected_delivery_ship.as_ref() == Some(&option.id),
                                    &option.label,
                                )
                                .clicked()
                            {
                                actions.push(FleetUiAction::SelectMiningDeliveryShip(
                                    option.id.clone(),
                                ));
                            }
                        }
                    });
            });
            ui.small(
                "The drone unloads its entire inventory on arrival. If the receiving hold fills, \
                 remaining cargo stays aboard.",
            );
        } else {
            ui.small("The drone will hold at the final mining target with its cargo aboard.");
        }
        ui.separator();
        ui.heading("Onboard smelting");
        if drone.smelter_installed {
            ui.small("Queues run autonomously from this drone's own inventory.");
            for (index, row) in drone.smelt_rows.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "{} · queued {} · {:.0}%",
                        row.name,
                        row.queued,
                        row.progress * 100.0
                    ));
                    if ui
                        .add_enabled(row.unlocked, egui::Button::new("Queue"))
                        .clicked()
                    {
                        actions.push(FleetUiAction::QueueDroneSmelt(index));
                    }
                });
            }
        } else {
            ui.small("Install the Drone smelter upgrade to process ore autonomously.");
        }
        ui.horizontal_wrapped(|ui| {
            if ui
                .add_enabled(drone.start_enabled, egui::Button::new("Start job"))
                .clicked()
            {
                actions.push(FleetUiAction::StartMiningJob);
            }
            if ui
                .add_enabled(
                    drone.pause_enabled,
                    egui::Button::new(if drone.paused { "Resume" } else { "Pause" }),
                )
                .clicked()
            {
                actions.push(FleetUiAction::ToggleMiningJobPause);
            }
            if ui
                .add_enabled(drone.recall_enabled, egui::Button::new("Recall"))
                .clicked()
            {
                actions.push(FleetUiAction::RecallMiningJob);
            }
            if ui
                .add_enabled(drone.cancel_enabled, egui::Button::new("Cancel job"))
                .clicked()
            {
                actions.push(FleetUiAction::CancelMiningJob);
            }
        });
    }

    ui.separator();
    ui.heading("Cargo transfer");
    ui.label(&selected.transfer_status);
    if ui
        .add_enabled(
            selected.transfer_available && !selected.cargo_rows.is_empty(),
            egui::Button::new("Take all from selected ship"),
        )
        .on_hover_text("Move every item that fits into the active ship; excess cargo stays aboard")
        .clicked()
    {
        actions.push(FleetUiAction::TakeAllFromSelected);
    }
    ui.horizontal(|ui| {
        ui.label("Quantity");
        if ui
            .add_enabled(selected.transfer_available, egui::Button::new("−").small())
            .on_hover_text(format!("Decrease by {}", view.quantity_step))
            .clicked()
        {
            actions.push(FleetUiAction::AdjustTransferQuantity(-1));
        }
        ui.label(view.transfer_quantity.to_string());
        if ui
            .add_enabled(selected.transfer_available, egui::Button::new("+").small())
            .on_hover_text(format!("Increase by {}", view.quantity_step))
            .clicked()
        {
            actions.push(FleetUiAction::AdjustTransferQuantity(1));
        }
    });
    ui.columns(2, |columns| {
        cargo_column(
            &mut columns[0],
            "Active cargo",
            &view.active_cargo,
            selected.transfer_available,
            true,
            actions,
        );
        cargo_column(
            &mut columns[1],
            "Selected cargo",
            &selected.cargo_rows,
            selected.transfer_available,
            false,
            actions,
        );
    });

    if let Some(latest) = &view.latest_operation {
        ui.separator();
        ui.label(format!("Latest: {latest}"));
    }
}

fn cargo_column(
    ui: &mut egui::Ui,
    heading: &str,
    rows: &[CargoRow],
    enabled: bool,
    to_selected: bool,
    actions: &mut Vec<FleetUiAction>,
) {
    ui.strong(heading);
    if rows.is_empty() {
        ui.label("Empty");
        return;
    }
    for row in rows {
        let label = if to_selected {
            format!("{} x{}  →", row.name, row.count)
        } else {
            format!("←  {} x{}", row.name, row.count)
        };
        if ui
            .add_enabled(enabled, egui::Button::new(label))
            .on_hover_text(&row.item_id)
            .clicked()
        {
            actions.push(if to_selected {
                FleetUiAction::TransferToSelected(row.item_id.clone())
            } else {
                FleetUiAction::TransferToActive(row.item_id.clone())
            });
        }
    }
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
    fn cargo_actions_preserve_item_identity_and_direction() {
        assert_ne!(
            FleetUiAction::TransferToSelected("core:iron_ore".to_string()),
            FleetUiAction::TransferToActive("core:iron_ore".to_string())
        );
        assert_ne!(
            FleetUiAction::TakeAllFromSelected,
            FleetUiAction::TransferToActive("core:iron_ore".to_string())
        );
    }

    #[test]
    fn auto_transfer_action_preserves_checkbox_state() {
        assert_ne!(
            FleetUiAction::SetMiningDeliverToShip(true),
            FleetUiAction::SetMiningDeliverToShip(false)
        );
    }
}
