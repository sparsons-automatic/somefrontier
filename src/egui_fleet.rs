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
    pub transfer_available: bool,
    pub transfer_status: String,
    pub cargo_rows: Vec<CargoRow>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShipOffer {
    pub ship_id: String,
    pub name: String,
    pub price: u32,
    pub affordable: bool,
    pub cargo_capacity: String,
    pub mass: String,
    pub forward_acceleration: f32,
    pub turn_acceleration: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShipyardView {
    pub station_name: String,
    pub offers: Vec<ShipOffer>,
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
    pub shipyard: Option<ShipyardView>,
    pub latest_operation: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FleetUiAction {
    TogglePin,
    SelectShip(usize),
    SwitchControl,
    AdjustTransferQuantity(i32),
    TransferToSelected(String),
    TransferToActive(String),
    BuyShip(String),
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
    ui.small("Select a ship to inspect it. Press F to close the fleet workspace.");

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
            if ui
                .add_sized(
                    [ui.available_width(), 64.0],
                    egui::Button::new(label).selected(view.selected == Some(index)),
                )
                .clicked()
            {
                actions.push(FleetUiAction::SelectShip(index));
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

    ui.separator();
    ui.heading("Cargo transfer");
    ui.label(&selected.transfer_status);
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

    if let Some(shipyard) = &view.shipyard {
        ui.separator();
        ui.heading(format!("Shipyard · {}", shipyard.station_name));
        if shipyard.offers.is_empty() {
            ui.label("No hulls are currently offered.");
        }
        for offer in &shipyard.offers {
            ui.small(format!(
                "Cargo {} · Mass {} · Thrust {:.0} · Turn {:.1}",
                offer.cargo_capacity,
                offer.mass,
                offer.forward_acceleration,
                offer.turn_acceleration
            ));
            if ui
                .add_enabled(
                    offer.affordable,
                    egui::Button::new(format!("Buy {} · {} cr", offer.name, offer.price)),
                )
                .clicked()
            {
                actions.push(FleetUiAction::BuyShip(offer.ship_id.clone()));
            }
        }
    }

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
    }
}
