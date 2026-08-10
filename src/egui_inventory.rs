use egui_macroquad::egui::{self, Id, Vec2};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProductionTab {
    Smelt,
    Craft,
    Process,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProductionRow {
    pub item: String,
    pub keep: u32,
    pub status: String,
    pub percent: String,
    pub active: bool,
    pub enabled: bool,
    pub unlocked: bool,
    pub requirements: Vec<String>,
}

impl ProductionRow {
    fn accepts_input(&self) -> bool {
        self.unlocked
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InventoryRow {
    pub item: String,
    pub quantity: u32,
    pub mass: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EquipmentChoice {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EquipmentSlot {
    pub label: String,
    pub installed_id: Option<String>,
    pub installed_name: String,
    pub choices: Vec<EquipmentChoice>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShipPane {
    pub ship_name: String,
    pub artwork: Option<String>,
    pub cargo: String,
    pub hull: String,
    pub shield: String,
    pub energy: String,
    pub weapon_energy: String,
    pub mass: String,
    pub engine: String,
    pub turn: String,
    pub power: Vec<String>,
    pub shields: Vec<EquipmentSlot>,
    pub weapons: Vec<EquipmentSlot>,
    pub feedback: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InventoryOverlayView {
    pub credits: u32,
    pub keep_click_step: i32,
    pub production_tab: ProductionTab,
    pub production: Vec<ProductionRow>,
    pub inventory: Vec<InventoryRow>,
    pub ship: ShipPane,
    pub show_ship: bool,
    pub show_production: bool,
    pub show_inventory: bool,
    pub ship_pinned: bool,
    pub production_pinned: bool,
    pub inventory_pinned: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InventoryUiAction {
    SetProductionTab(ProductionTab),
    AdjustKeep { row: usize, direction: i32 },
    QueueRecipe { row: usize },
    InstallShield { slot: usize, shield_id: String },
    InstallWeapon { slot: usize, weapon_id: String },
    OpenUpgrades,
    ToggleShipPin,
    ToggleProductionPin,
    ToggleInventoryPin,
}

pub fn draw(
    ctx: &egui::Context,
    screen: [f32; 2],
    view: &InventoryOverlayView,
) -> Vec<InventoryUiAction> {
    ctx.set_style(egui::Style::default());
    let mut actions = Vec::new();
    let ship_height = default_window_height(screen[1], 520.0);

    if view.show_ship {
        egui::Window::new("Ship")
            .id(Id::new("inventory-ship-window"))
            .default_pos(egui::pos2(16.0, 48.0))
            .default_size(Vec2::new(300.0, ship_height))
            .min_width(240.0)
            .resizable(true)
            .collapsible(true)
            .vscroll(true)
            .show(ctx, |ui| {
                if crate::egui_pin::toggle(ui, view.ship_pinned) {
                    actions.push(InventoryUiAction::ToggleShipPin);
                }
                draw_ship_pane(ui, &view.ship, &mut actions);
            });
    }

    actions.extend(draw_work_windows(ctx, screen, view));

    actions
}

pub fn draw_work_windows(
    ctx: &egui::Context,
    screen: [f32; 2],
    view: &InventoryOverlayView,
) -> Vec<InventoryUiAction> {
    ctx.set_style(egui::Style::default());
    let mut actions = Vec::new();
    let table_height = default_window_height(screen[1], 360.0);

    if view.show_production {
        egui::Window::new("Production")
            .id(Id::new("inventory-production-window"))
            .default_pos(egui::pos2((screen[0] * 0.25).max(180.0), 48.0))
            .default_size(Vec2::new(480.0, table_height))
            .min_width(360.0)
            .resizable(true)
            .collapsible(true)
            .scroll([true, true])
            .show(ctx, |ui| {
                if crate::egui_pin::toggle(ui, view.production_pinned) {
                    actions.push(InventoryUiAction::ToggleProductionPin);
                }
                draw_production_pane(ui, view, &mut actions);
            });
    }

    if view.show_inventory {
        egui::Window::new("Inventory")
            .id(Id::new("inventory-cargo-window"))
            .default_pos(egui::pos2((screen[0] * 0.68).max(360.0), 48.0))
            .default_size(Vec2::new(330.0, table_height))
            .min_width(240.0)
            .resizable(true)
            .collapsible(true)
            .scroll([true, true])
            .show(ctx, |ui| {
                if crate::egui_pin::toggle(ui, view.inventory_pinned) {
                    actions.push(InventoryUiAction::ToggleInventoryPin);
                }
                draw_inventory_pane(ui, view);
            });
    }

    actions
}

fn default_window_height(screen_height: f32, preferred_height: f32) -> f32 {
    preferred_height.min((screen_height - 96.0).max(240.0))
}

fn draw_ship_pane(ui: &mut egui::Ui, ship: &ShipPane, actions: &mut Vec<InventoryUiAction>) {
    crate::egui_art::preview(ui, ship.artwork.as_deref(), Vec2::new(220.0, 150.0));
    ui.heading(&ship.ship_name);
    ui.label(&ship.cargo);
    ui.separator();
    egui::Grid::new("ship-vitals")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            ship_stat(ui, "Hull", &ship.hull);
            ship_stat(ui, "Shield", &ship.shield);
            ship_stat(ui, "Drive energy", &ship.energy);
            ship_stat(ui, "Weapon capacitor", &ship.weapon_energy);
            ship_stat(ui, "Mass", &ship.mass);
            ship_stat(ui, "Engine", &ship.engine);
            ship_stat(ui, "Turn", &ship.turn);
        });

    section_label(ui, "Power");
    for line in &ship.power {
        truncated_label(ui, line);
    }
    section_label(ui, "Shield systems");
    for (slot_index, slot) in ship.shields.iter().enumerate() {
        equipment_combo(ui, slot, "shield-slot", slot_index, |id| {
            actions.push(InventoryUiAction::InstallShield {
                slot: slot_index,
                shield_id: id,
            });
        });
    }
    section_label(ui, "Automatic weapons");
    for (slot_index, slot) in ship.weapons.iter().enumerate() {
        equipment_combo(ui, slot, "weapon-slot", slot_index, |id| {
            actions.push(InventoryUiAction::InstallWeapon {
                slot: slot_index,
                weapon_id: id,
            });
        });
    }
    if ui.button("Inspect ship upgrades").clicked() {
        actions.push(InventoryUiAction::OpenUpgrades);
    }
    section_label(ui, "Operations");
    for line in ship.feedback.iter().take(6) {
        truncated_label(ui, line);
    }
}

fn draw_production_pane(
    ui: &mut egui::Ui,
    view: &InventoryOverlayView,
    actions: &mut Vec<InventoryUiAction>,
) {
    ui.horizontal(|ui| {
        for (label, tab) in [
            ("Smelt", ProductionTab::Smelt),
            ("Craft", ProductionTab::Craft),
            ("Process", ProductionTab::Process),
        ] {
            if ui
                .selectable_label(view.production_tab == tab, label)
                .clicked()
            {
                actions.push(InventoryUiAction::SetProductionTab(tab));
            }
        }
    });
    ui.separator();

    let selected_id = Id::new("production-selected-row");
    let mut selected = ui
        .ctx()
        .data(|data| data.get_temp::<usize>(selected_id))
        .unwrap_or(0)
        .min(view.production.len().saturating_sub(1));
    let down = ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown));
    let up = ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp));
    let enter = ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
    if down && !view.production.is_empty() {
        selected = (selected + 1).min(view.production.len() - 1);
    }
    if up {
        selected = selected.saturating_sub(1);
    }
    if enter
        && view
            .production
            .get(selected)
            .is_some_and(ProductionRow::accepts_input)
    {
        actions.push(InventoryUiAction::QueueRecipe { row: selected });
    }

    egui::Grid::new("production-grid")
        .striped(true)
        .min_col_width(44.0)
        .show(ui, |ui| {
            for header in ["Item", "Keep", "Status", "%", "Active"] {
                ui.strong(header);
            }
            ui.end_row();

            for (row_index, row) in view.production.iter().enumerate() {
                let response = ui.selectable_label(row_index == selected, &row.item);
                if response.clicked() {
                    selected = row_index;
                    if row.accepts_input() {
                        actions.push(InventoryUiAction::QueueRecipe { row: row_index });
                    }
                }
                if !row.requirements.is_empty() {
                    response.on_hover_text(row.requirements.join("\n"));
                }

                ui.horizontal(|ui| {
                    if keep_button(
                        ui,
                        "−",
                        view.keep_click_step,
                        "Decrease Keep (Shift: 10, Control: 20)",
                        row.accepts_input(),
                    )
                    .clicked()
                    {
                        actions.push(InventoryUiAction::AdjustKeep {
                            row: row_index,
                            direction: -1,
                        });
                    }
                    ui.label(row.keep.to_string());
                    if keep_button(
                        ui,
                        "+",
                        view.keep_click_step,
                        "Increase Keep (Shift: 10, Control: 20)",
                        row.accepts_input(),
                    )
                    .clicked()
                    {
                        actions.push(InventoryUiAction::AdjustKeep {
                            row: row_index,
                            direction: 1,
                        });
                    }
                });
                ui.add_enabled(row.enabled, egui::Label::new(&row.status));
                ui.add_enabled(row.enabled, egui::Label::new(&row.percent));
                let mut active = row.active;
                ui.add_enabled(false, egui::Checkbox::new(&mut active, ""));
                ui.end_row();
            }
        });
    ui.ctx()
        .data_mut(|data| data.insert_temp(selected_id, selected));
}

fn draw_inventory_pane(ui: &mut egui::Ui, view: &InventoryOverlayView) {
    ui.label(format!("Credits {}", view.credits));
    ui.separator();
    egui::Grid::new("inventory-grid")
        .striped(true)
        .min_col_width(48.0)
        .show(ui, |ui| {
            for header in ["Item", "Qty", "Mass"] {
                ui.strong(header);
            }
            ui.end_row();

            if view.inventory.is_empty() {
                ui.label("empty");
                ui.label("0");
                ui.label("0 kg");
                ui.end_row();
            } else {
                for row in &view.inventory {
                    ui.label(&row.item).on_hover_text(&row.item);
                    ui.label(row.quantity.to_string());
                    ui.label(&row.mass);
                    ui.end_row();
                }
            }
        });
}

fn equipment_combo(
    ui: &mut egui::Ui,
    slot: &EquipmentSlot,
    id_prefix: &str,
    index: usize,
    mut install: impl FnMut(String),
) {
    ui.label(&slot.label);
    egui::ComboBox::from_id_salt((id_prefix, index))
        .selected_text(&slot.installed_name)
        .width(ui.available_width())
        .show_ui(ui, |ui| {
            if slot.choices.is_empty() {
                ui.label("No compatible equipment in cargo");
            }
            for choice in &slot.choices {
                let selected = slot.installed_id.as_deref() == Some(choice.id.as_str());
                if ui.selectable_label(selected, &choice.name).clicked() && !selected {
                    install(choice.id.clone());
                }
            }
        });
}

fn keep_button(
    ui: &mut egui::Ui,
    icon: &str,
    step: i32,
    tooltip: &str,
    enabled: bool,
) -> egui::Response {
    let label = keep_chip_label(icon, step);
    ui.add_enabled(enabled, egui::Button::new(label).small())
        .on_hover_text(tooltip)
}

fn keep_chip_label(icon: &str, step: i32) -> String {
    if step > 1 {
        format!("{icon}{step}")
    } else {
        icon.to_string()
    }
}

fn section_label(ui: &mut egui::Ui, text: &str) {
    ui.separator();
    ui.strong(text);
}

fn ship_stat(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.label(label);
    truncated_label(ui, value);
    ui.end_row();
}

fn truncated_label(ui: &mut egui::Ui, text: &str) {
    ui.add(egui::Label::new(text).truncate())
        .on_hover_text(text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_windows_fit_the_screen_without_forcing_full_height() {
        assert_eq!(default_window_height(320.0, 520.0), 240.0);
        assert_eq!(default_window_height(760.0, 520.0), 520.0);
        assert_eq!(default_window_height(760.0, 360.0), 360.0);
    }

    #[test]
    fn unlocked_rows_accept_input_even_when_materials_are_unavailable() {
        let row = ProductionRow {
            item: "Test plate".to_string(),
            keep: 100,
            status: "0/100".to_string(),
            percent: String::new(),
            active: false,
            enabled: false,
            unlocked: true,
            requirements: Vec::new(),
        };

        assert!(row.accepts_input());
    }

    #[test]
    fn keep_chips_expand_to_show_the_active_modifier_step() {
        assert_eq!(keep_chip_label("+", 1), "+");
        assert_eq!(keep_chip_label("−", 10), "−10");
        assert_eq!(keep_chip_label("+", 20), "+20");
    }
}
