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
    pub group: String,
    pub count: u32,
    pub mass: String,
    pub produced_by: Vec<String>,
    pub equipment_links: Vec<String>,
    pub could_make: Vec<String>,
    pub locations: Vec<CargoLocation>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CargoLocation {
    pub ship: String,
    pub count: u32,
    pub mass: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SharedInventoryView {
    pub rows: Vec<CargoRow>,
    pub linked_hulls: usize,
    pub total_hulls: usize,
    pub total_mass: String,
    pub total_capacity: String,
    pub unavailable: Vec<String>,
    pub consolidate_enabled: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FleetPage {
    #[default]
    Stats,
    Upgrades,
    Inventory,
    Production,
}

impl FleetPage {
    fn label(self) -> &'static str {
        match self {
            Self::Stats => "Ship stats",
            Self::Upgrades => "Upgrades",
            Self::Inventory => "Inventory",
            Self::Production => "Production",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpgradeCostRow {
    pub item: String,
    pub required: u32,
    pub available: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpgradeNode {
    pub upgrade_index: usize,
    pub name: String,
    pub level: u32,
    pub effect: String,
    pub cost: Vec<UpgradeCostRow>,
    pub affordable: bool,
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
    pub active: bool,
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
    pub mining_drone: Option<MiningDroneView>,
    pub systems: Option<crate::egui_inventory::ShipPane>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FleetPaneView {
    pub pinned: bool,
    pub ships: Vec<ShipRow>,
    pub selected: Option<usize>,
    pub selected_ship: Option<SelectedShip>,
    pub page: FleetPage,
    pub upgrades: Vec<UpgradeNode>,
    pub selected_upgrade: Option<usize>,
    pub upgrade_purchase_count: u32,
    pub production: Option<crate::egui_inventory::InventoryOverlayView>,
    pub inventory: SharedInventoryView,
    pub credits: u32,
    pub latest_operation: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FleetUiAction {
    TogglePin,
    SelectShip(usize),
    SetPage(FleetPage),
    SelectUpgrade(usize),
    PurchaseUpgrade(usize),
    SetProductionTab(crate::egui_inventory::ProductionTab),
    AdjustProductionKeep { row: usize, direction: i32 },
    QueueProductionRecipe(usize),
    QueueDroneSmelt(usize),
    InstallShield { slot: usize, shield_id: String },
    InstallWeapon { slot: usize, weapon_id: String },
    ToggleAutoRepair,
    ConsolidateToActive,
    SwitchControl,
    SetShareInventory(bool),
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
    let height = 760.0_f32.min((screen[1] - 72.0).max(360.0));
    let width = 1500.0_f32.min((screen[0] - 40.0).max(320.0));

    egui::Window::new("Fleet")
        .id(Id::new("fleet-workspace-window"))
        .default_pos(egui::pos2(36.0, 36.0))
        .default_size(Vec2::new(width, height))
        .min_size(Vec2::new(460.0_f32.min(width), 360.0))
        .resizable(true)
        .collapsible(true)
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
    ui.small("Choose a controllable asset, then use its control pages. Double-click a hull to switch control.");

    if view.ships.is_empty() {
        ui.label("No owned ships are available.");
    } else {
        egui::ScrollArea::horizontal()
            .id_salt("fleet-ship-tabs")
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    for (index, ship) in view.ships.iter().enumerate() {
                        let active = if ship.active { " · ACTIVE" } else { "" };
                        let unavailable = if ship.resolved { "" } else { " · MISSING" };
                        let label = format!(
                            "{}{}\n{} · {}\n{}{}",
                            ship.callsign,
                            active,
                            ship.hull_name,
                            ship.system,
                            ship.condition,
                            unavailable
                        );
                        let response = ui.add_sized(
                            [230.0, 72.0],
                            egui::Button::new(label).selected(view.selected == Some(index)),
                        );
                        if response.clicked() {
                            actions.push(FleetUiAction::SelectShip(index));
                        }
                        if response.double_clicked() && !ship.active && ship.resolved {
                            actions.push(FleetUiAction::SwitchControl);
                        }
                    }
                });
            });
    }

    let Some(selected) = &view.selected_ship else {
        return;
    };
    ui.separator();
    let body_size = ui.available_size().max(Vec2::splat(1.0));
    let (body_rect, _) = ui.allocate_exact_size(body_size, egui::Sense::hover());
    let rail_width = 138.0_f32.min((body_rect.width() - 96.0).max(96.0));
    let gap = 10.0_f32.min((body_rect.width() - rail_width).max(0.0));
    let rail_rect =
        egui::Rect::from_min_size(body_rect.min, Vec2::new(rail_width, body_rect.height()));
    let content_rect = egui::Rect::from_min_max(
        egui::pos2(rail_rect.right() + gap, body_rect.top()),
        body_rect.right_bottom(),
    );

    let mut rail_ui = ui.new_child(egui::UiBuilder::new().max_rect(rail_rect));
    rail_ui.set_clip_rect(rail_rect);
    rail_ui.strong("CONTROL PANEL");
    rail_ui.add_space(6.0);
    for page in [
        FleetPage::Stats,
        FleetPage::Upgrades,
        FleetPage::Inventory,
        FleetPage::Production,
    ] {
        if rail_ui
            .add_sized(
                [(rail_width - 6.0).max(80.0), 36.0],
                egui::Button::new(page.label()).selected(view.page == page),
            )
            .clicked()
        {
            actions.push(FleetUiAction::SetPage(page));
        }
    }
    if content_rect.width() <= 0.0 || content_rect.height() <= 0.0 {
        return;
    }
    let mut content_ui = ui.new_child(egui::UiBuilder::new().max_rect(content_rect));
    content_ui.set_clip_rect(content_rect);
    if view.page == FleetPage::Production {
        draw_production(&mut content_ui, view, selected, actions);
        return;
    }
    crate::egui_pane::both_scroll(&mut content_ui, "fleet-page-scroll", |ui| {
        if view.page == FleetPage::Stats {
            draw_stats_hero(ui, selected, actions);

            if let Some(systems) = &selected.systems {
                draw_ship_systems(ui, systems, actions);
            }

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
                                    actions
                                        .push(FleetUiAction::SelectMiningPlanet(option.id.clone()));
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
                                    actions.push(FleetUiAction::SelectMiningResource(
                                        option.id.clone(),
                                    ));
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
                            .and_then(|id| {
                                drone.delivery_ships.iter().find(|option| &option.id == id)
                            })
                            .map(|option| option.label.as_str())
                            .unwrap_or("Select fleet ship");
                        egui::ComboBox::from_id_salt("fleet-mining-delivery-ship")
                            .selected_text(selected_label)
                            .show_ui(ui, |ui| {
                                for option in &drone.delivery_ships {
                                    if ui
                                        .selectable_label(
                                            drone.selected_delivery_ship.as_ref()
                                                == Some(&option.id),
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
                    ui.small(
                        "The drone will hold at the final mining target with its cargo aboard.",
                    );
                }
                ui.separator();
                ui.heading("Onboard smelting");
                if drone.smelter_installed {
                    ui.small("Queues run autonomously from this drone's own inventory.");
                    for (index, row) in drone.smelt_rows.iter().enumerate() {
                        if draw_drone_smelt_row(ui, row) {
                            actions.push(FleetUiAction::QueueDroneSmelt(index));
                        }
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
        }

        if view.page == FleetPage::Upgrades {
            draw_upgrades(ui, view, actions);
        }

        if view.page == FleetPage::Inventory {
            draw_shared_inventory(ui, &view.inventory, view.credits, actions);
        }

        if let Some(latest) = &view.latest_operation {
            ui.separator();
            ui.label(format!("Latest: {latest}"));
        }
    });
}

fn draw_stats_hero(ui: &mut egui::Ui, selected: &SelectedShip, actions: &mut Vec<FleetUiAction>) {
    let available_width = ui.available_width();
    if available_width >= 720.0 {
        let hero_width = stats_hero_width(available_width);
        ui.horizontal_top(|ui| {
            ui.allocate_ui_with_layout(
                Vec2::new(hero_width, 518.0),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    crate::egui_art::preview(
                        ui,
                        selected.artwork.as_deref(),
                        Vec2::new(840.0, 510.0),
                    );
                },
            );
            ui.add_space(10.0);
            ui.vertical(|ui| draw_stats_details(ui, selected, actions));
        });
    } else {
        crate::egui_art::preview(ui, selected.artwork.as_deref(), Vec2::new(840.0, 510.0));
        draw_stats_details(ui, selected, actions);
    }
}

fn stats_hero_width(available_width: f32) -> f32 {
    (available_width - 316.0).clamp(280.0, 840.0)
}

fn draw_stats_details(
    ui: &mut egui::Ui,
    selected: &SelectedShip,
    actions: &mut Vec<FleetUiAction>,
) {
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
}

fn draw_ship_systems(
    ui: &mut egui::Ui,
    systems: &crate::egui_inventory::ShipPane,
    actions: &mut Vec<FleetUiAction>,
) {
    ui.separator();
    ui.heading("Installed systems");
    for line in &systems.power {
        ui.label(line);
    }
    ui.strong("Auto Repair");
    ui.label(&systems.auto_repair_status);
    if let Some(label) = &systems.auto_repair_toggle {
        if ui.button(label).clicked() {
            actions.push(FleetUiAction::ToggleAutoRepair);
        }
    }
    ui.strong("Shield systems");
    for (slot, system) in systems.shields.iter().enumerate() {
        fleet_equipment_combo(ui, system, "fleet-shield-slot", slot, |shield_id| {
            actions.push(FleetUiAction::InstallShield { slot, shield_id });
        });
    }
    ui.strong("Automatic weapons");
    for (slot, system) in systems.weapons.iter().enumerate() {
        fleet_equipment_combo(ui, system, "fleet-weapon-slot", slot, |weapon_id| {
            actions.push(FleetUiAction::InstallWeapon { slot, weapon_id });
        });
    }
}

fn fleet_equipment_combo(
    ui: &mut egui::Ui,
    system: &crate::egui_inventory::EquipmentSlot,
    id_prefix: &str,
    slot: usize,
    mut install: impl FnMut(String),
) {
    ui.horizontal(|ui| {
        ui.label(&system.label);
        egui::ComboBox::from_id_salt((id_prefix, slot))
            .selected_text(&system.installed_name)
            .show_ui(ui, |ui| {
                if system.choices.is_empty() {
                    ui.label("No compatible equipment in cargo");
                }
                for choice in &system.choices {
                    let selected = system.installed_id.as_deref() == Some(choice.id.as_str());
                    if ui.selectable_label(selected, &choice.name).clicked() && !selected {
                        install(choice.id.clone());
                    }
                }
            });
    });
}

fn draw_production(
    ui: &mut egui::Ui,
    view: &FleetPaneView,
    selected: &SelectedShip,
    actions: &mut Vec<FleetUiAction>,
) {
    ui.heading("Production");
    if let Some(production) = &view.production {
        ui.small("Production queues and Keep targets belong to the currently controlled hull.");
        for action in crate::egui_inventory::draw_production_page(ui, production) {
            match action {
                crate::egui_inventory::InventoryUiAction::SetProductionTab(tab) => {
                    actions.push(FleetUiAction::SetProductionTab(tab));
                }
                crate::egui_inventory::InventoryUiAction::AdjustKeep { row, direction } => {
                    actions.push(FleetUiAction::AdjustProductionKeep { row, direction });
                }
                crate::egui_inventory::InventoryUiAction::QueueRecipe { row } => {
                    actions.push(FleetUiAction::QueueProductionRecipe(row));
                }
                _ => {}
            }
        }
        return;
    }

    if let Some(drone) = &selected.mining_drone {
        ui.small("This autonomous asset exposes only its installed onboard production systems.");
        ui.separator();
        ui.heading("Onboard smelting");
        if !drone.smelter_installed {
            ui.label("Install the Drone smelter upgrade to process ore autonomously.");
            return;
        }
        if drone.smelt_rows.is_empty() {
            ui.label("No smelting recipes are available.");
            return;
        }
        for (index, row) in drone.smelt_rows.iter().enumerate() {
            if draw_drone_smelt_row(ui, row) {
                actions.push(FleetUiAction::QueueDroneSmelt(index));
            }
        }
        return;
    }

    ui.label("General production is available on the currently controlled hull.");
    ui.small("Double-click this hull's tab or use Ship stats → Switch control to manage its production queues.");
}

fn draw_drone_smelt_row(ui: &mut egui::Ui, row: &DroneSmeltRow) -> bool {
    let active_color = egui::Color32::from_rgb(104, 211, 153);
    let mut frame = egui::Frame::group(ui.style()).inner_margin(6.0);
    if row.active {
        frame = frame
            .fill(egui::Color32::from_rgba_premultiplied(45, 92, 67, 90))
            .stroke(egui::Stroke::new(1.0_f32, active_color));
    }
    frame
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                if row.active {
                    ui.colored_label(
                        active_color,
                        egui::RichText::new(format!("▶ {}", row.name)).strong(),
                    );
                } else {
                    ui.label(&row.name);
                }
                if row.active {
                    ui.colored_label(active_color, format!("queued {}", row.queued));
                } else {
                    ui.label(format!("queued {}", row.queued));
                }
                ui.add(
                    egui::ProgressBar::new(row.progress.clamp(0.0, 1.0))
                        .desired_width(90.0)
                        .text(format!("{:.0}%", row.progress * 100.0)),
                );
                ui.add_enabled(row.unlocked, egui::Button::new("Queue"))
                    .clicked()
            })
            .inner
        })
        .inner
}

fn draw_upgrades(ui: &mut egui::Ui, view: &FleetPaneView, actions: &mut Vec<FleetUiAction>) {
    ui.heading("Upgrade matrix");
    ui.small("Select a module card to inspect its next level. Installed levels and material availability belong to the selected hull.");
    ui.separator();

    let selected_index = view
        .selected_upgrade
        .filter(|selected| {
            view.upgrades
                .iter()
                .any(|node| node.upgrade_index == *selected)
        })
        .or_else(|| view.upgrades.first().map(|node| node.upgrade_index));
    ui.columns(2, |columns| {
        columns[0].set_min_width(360.0);
        egui::Grid::new("fleet-upgrade-matrix")
            .num_columns(2)
            .spacing([10.0, 10.0])
            .show(&mut columns[0], |ui| {
                for (position, node) in view.upgrades.iter().enumerate() {
                    let status = if node.affordable {
                        "READY"
                    } else {
                        "MATERIALS"
                    };
                    let label = format!("{}\nLevel {} · {}", node.name, node.level, status);
                    if ui
                        .add_sized(
                            [176.0, 68.0],
                            egui::Button::new(label)
                                .selected(selected_index == Some(node.upgrade_index)),
                        )
                        .on_hover_text(&node.effect)
                        .clicked()
                    {
                        actions.push(FleetUiAction::SelectUpgrade(node.upgrade_index));
                    }
                    if position % 2 == 1 {
                        ui.end_row();
                    }
                }
            });

        columns[1].separator();
        columns[1].heading("Module detail");
        let Some(node) = selected_index.and_then(|selected| {
            view.upgrades
                .iter()
                .find(|node| node.upgrade_index == selected)
        }) else {
            columns[1].label("No upgrades are available for this hull.");
            return;
        };
        columns[1].heading(&node.name);
        columns[1].label(format!("Installed level {}", node.level));
        columns[1].label(&node.effect);
        columns[1].separator();
        columns[1].strong("Next-level requirements");
        for cost in &node.cost {
            let text = format!("{}  {}/{}", cost.item, cost.available, cost.required);
            if cost.available >= cost.required {
                columns[1].label(text);
            } else {
                columns[1].colored_label(columns[1].visuals().warn_fg_color, text);
            }
        }
        if columns[1]
            .add_enabled(
                node.affordable,
                egui::Button::new(format!("Install +{}", view.upgrade_purchase_count)),
            )
            .on_disabled_hover_text("Required components are missing")
            .clicked()
        {
            actions.push(FleetUiAction::PurchaseUpgrade(node.upgrade_index));
        }
    });
}

fn draw_shared_inventory(
    ui: &mut egui::Ui,
    inventory: &SharedInventoryView,
    credits: u32,
    actions: &mut Vec<FleetUiAction>,
) {
    ui.heading("Shared fleet inventory");
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.strong(format!("{} linked hulls", inventory.linked_hulls));
            ui.separator();
            ui.label(format!(
                "{} / {} distributed",
                inventory.total_mass, inventory.total_capacity
            ));
            ui.separator();
            ui.label(format!("Credits {credits}"));
        });
        ui.small(
            "Cargo remains physically aboard each hull; production consumes local stock first.",
        );
    });
    if ui
        .add_enabled(
            inventory.consolidate_enabled,
            egui::Button::new("Consolidate into active hold"),
        )
        .on_hover_text(
            "Move as much linked cargo as will fit onto the controlled hull; overflow stays aboard its carrier",
        )
        .clicked()
    {
        actions.push(FleetUiAction::ConsolidateToActive);
    }

    let search_id = Id::new("fleet-inventory-search");
    let mut search = ui
        .ctx()
        .data(|data| data.get_temp::<String>(search_id))
        .unwrap_or_default();
    ui.horizontal(|ui| {
        ui.label("Find");
        ui.add(
            egui::TextEdit::singleline(&mut search)
                .hint_text("item, category, or carrying hull")
                .desired_width(280.0),
        );
        if !search.is_empty() && ui.small_button("Clear").clicked() {
            search.clear();
        }
    });
    ui.ctx()
        .data_mut(|data| data.insert_temp(search_id, search.clone()));
    let needle = search.trim().to_lowercase();
    let visible = inventory
        .rows
        .iter()
        .filter(|row| {
            needle.is_empty()
                || row.name.to_lowercase().contains(&needle)
                || row.group.to_lowercase().contains(&needle)
                || row
                    .locations
                    .iter()
                    .any(|location| location.ship.to_lowercase().contains(&needle))
        })
        .collect::<Vec<_>>();

    ui.separator();
    egui::Grid::new("fleet-shared-inventory-grid")
        .striped(true)
        .min_col_width(70.0)
        .show(ui, |ui| {
            for heading in ["Item", "Category", "Quantity", "Mass", "Carriers"] {
                ui.strong(heading);
            }
            ui.end_row();
            if visible.is_empty() {
                ui.label(if inventory.rows.is_empty() {
                    "Cargo holds are empty"
                } else {
                    "No matching cargo"
                });
                ui.end_row();
            }
            for row in visible {
                ui.label(&row.name)
                    .on_hover_ui(|ui| draw_cargo_info(ui, row));
                ui.label(&row.group);
                ui.label(row.count.to_string());
                ui.label(&row.mass);
                ui.label(row.locations.len().to_string())
                    .on_hover_ui(|ui| draw_cargo_locations(ui, row));
                ui.end_row();
            }
        });

    if !inventory.unavailable.is_empty() {
        ui.separator();
        egui::CollapsingHeader::new(format!(
            "Unavailable cargo · {} of {} hulls outside this pool",
            inventory.total_hulls.saturating_sub(inventory.linked_hulls),
            inventory.total_hulls
        ))
        .show(ui, |ui| {
            for line in &inventory.unavailable {
                ui.label(line);
            }
        });
    }
}

fn draw_cargo_locations(ui: &mut egui::Ui, row: &CargoRow) {
    ui.strong("Physical cargo locations");
    for location in &row.locations {
        ui.label(format!(
            "{} · {} · {}",
            location.ship, location.count, location.mass
        ));
    }
}

fn draw_cargo_info(ui: &mut egui::Ui, row: &CargoRow) {
    ui.set_max_width(340.0);
    ui.strong(&row.name);
    ui.small(&row.item_id);
    ui.label(format!(
        "In cargo: {} · Stack mass: {}",
        row.count, row.mass
    ));
    draw_cargo_locations(ui, row);
    for (heading, lines, empty) in [
        (
            "Produced by (discovered)",
            &row.produced_by,
            "No discovered source recipes",
        ),
        (
            "Equipment compatibility",
            &row.equipment_links,
            "No equipment links",
        ),
        (
            "Could make (discovered)",
            &row.could_make,
            "No discovered follow-on recipes",
        ),
    ] {
        ui.separator();
        ui.strong(heading);
        if lines.is_empty() {
            ui.weak(empty);
        } else {
            for line in lines {
                ui.label(format!("• {line}"));
            }
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
    fn auto_transfer_action_preserves_checkbox_state() {
        assert_ne!(
            FleetUiAction::SetMiningDeliverToShip(true),
            FleetUiAction::SetMiningDeliverToShip(false)
        );
    }

    #[test]
    fn control_panel_pages_and_upgrade_actions_remain_distinct() {
        assert_ne!(FleetPage::Stats, FleetPage::Upgrades);
        assert_ne!(FleetPage::Upgrades, FleetPage::Inventory);
        assert_ne!(FleetPage::Inventory, FleetPage::Production);
        assert_ne!(
            FleetUiAction::SelectUpgrade(2),
            FleetUiAction::PurchaseUpgrade(2)
        );
        assert_ne!(
            FleetUiAction::QueueProductionRecipe(2),
            FleetUiAction::QueueDroneSmelt(2)
        );
    }

    #[test]
    fn stats_hero_scales_to_three_times_the_previous_preview_width() {
        assert_eq!(stats_hero_width(1_400.0), 840.0);
        assert_eq!(stats_hero_width(900.0), 584.0);
        assert_eq!(stats_hero_width(500.0), 280.0);
    }

    #[test]
    fn inventory_page_does_not_force_a_narrow_window_back_open() {
        let ctx = egui::Context::default();
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                Vec2::new(480.0, 500.0),
            )),
            ..Default::default()
        };
        let view = FleetPaneView {
            pinned: false,
            ships: vec![ShipRow {
                callsign: "Test ship".to_string(),
                hull_name: "Test hull".to_string(),
                system: "frontier".to_string(),
                condition: "Operational".to_string(),
                active: false,
                resolved: true,
            }],
            selected: Some(0),
            selected_ship: Some(SelectedShip {
                callsign: "Test ship".to_string(),
                hull_name: "Test hull".to_string(),
                artwork: None,
                system: "frontier".to_string(),
                distance: "20 units".to_string(),
                hull: "100/100".to_string(),
                shields: "100/100".to_string(),
                energy: "100/100".to_string(),
                weapon_energy: "100/100".to_string(),
                cargo: "0 / 10 t".to_string(),
                turrets: 0,
                active: false,
                switch_enabled: true,
                share_inventory: true,
                linked_inventory_status: "Linked".to_string(),
                mining_drone: None,
                systems: None,
            }),
            page: FleetPage::Inventory,
            upgrades: Vec::new(),
            selected_upgrade: None,
            upgrade_purchase_count: 1,
            production: None,
            inventory: SharedInventoryView {
                rows: vec![CargoRow {
                    item_id: "test:very_long_inventory_item".to_string(),
                    name: "An intentionally very long inventory item name".to_string(),
                    group: "Special cargo".to_string(),
                    count: 999,
                    mass: "999 kg".to_string(),
                    produced_by: Vec::new(),
                    equipment_links: Vec::new(),
                    could_make: Vec::new(),
                    locations: vec![CargoLocation {
                        ship: "Test ship".to_string(),
                        count: 999,
                        mass: "999 kg".to_string(),
                    }],
                }],
                linked_hulls: 1,
                total_hulls: 1,
                total_mass: "999 kg".to_string(),
                total_capacity: "1 t".to_string(),
                unavailable: Vec::new(),
                consolidate_enabled: false,
            },
            credits: 0,
            latest_operation: None,
        };

        let _ = ctx.run(input, |ctx| {
            let _ = draw(ctx, [480.0, 500.0], &view);
        });
        let width = ctx
            .memory(|memory| {
                memory
                    .area_rect(Id::new("fleet-workspace-window"))
                    .map(|rect| rect.width())
            })
            .expect("fleet window should render");
        assert!(width <= 460.0, "inventory content forced width to {width}");
    }
}
