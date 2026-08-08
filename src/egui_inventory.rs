use egui_extras::{Column, TableBuilder};
use egui_macroquad::egui::{
    self, Align, Color32, FontData, FontDefinitions, FontFamily, FontId, Frame, Id, Layout, Margin,
    RichText, Stroke, TextStyle, Vec2,
};

const WIDE_LAYOUT_MIN_WIDTH: f32 = 990.0;
const WIDE_LAYOUT_MIN_HEIGHT: f32 = 540.0;
const ROW_HEIGHT: f32 = 26.0;
const CONTENT_MARGIN_X: i8 = 40;
const CONTENT_MARGIN_Y: i8 = 24;

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
    pub cargo: String,
    pub hull: String,
    pub shield: String,
    pub energy: String,
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
    pub production_tab: ProductionTab,
    pub production: Vec<ProductionRow>,
    pub inventory: Vec<InventoryRow>,
    pub ship: ShipPane,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InventoryUiAction {
    SetProductionTab(ProductionTab),
    AdjustKeep { row: usize, direction: i32 },
    QueueRecipe { row: usize },
    InstallShield { slot: usize, shield_id: String },
    InstallWeapon { slot: usize, weapon_id: String },
    OpenUpgrades,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CompactPane {
    Ship,
    Production,
    Inventory,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OverlayRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub fn overlay_rect(screen: [f32; 2]) -> OverlayRect {
    let margin_x = (screen[0] * 0.045).clamp(12.0, 76.0);
    let margin_y = (screen[1] * 0.055).clamp(12.0, 66.0);
    OverlayRect {
        x: margin_x,
        y: margin_y,
        width: (screen[0] - margin_x * 2.0).max(320.0),
        height: (screen[1] - margin_y * 2.0).max(260.0),
    }
}

fn uses_wide_layout(rect: OverlayRect) -> bool {
    rect.width >= WIDE_LAYOUT_MIN_WIDTH && rect.height >= WIDE_LAYOUT_MIN_HEIGHT
}

pub fn draw(
    ctx: &egui::Context,
    screen: [f32; 2],
    view: &InventoryOverlayView,
) -> Vec<InventoryUiAction> {
    configure_context(ctx);
    let rect = overlay_rect(screen);
    let mut actions = Vec::new();

    egui::Area::new(Id::new("inventory-production-overlay"))
        .fixed_pos(egui::pos2(rect.x, rect.y))
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            ui.set_min_size(Vec2::new(rect.width, rect.height));
            ui.set_max_size(Vec2::new(rect.width, rect.height));
            Frame::NONE
                .inner_margin(Margin::symmetric(CONTENT_MARGIN_X, CONTENT_MARGIN_Y))
                .show(ui, |ui| {
                    ui.set_height(rect.height - f32::from(CONTENT_MARGIN_Y) * 2.0);
                    if uses_wide_layout(rect) {
                        draw_wide(ui, view, &mut actions);
                    } else {
                        draw_compact(ctx, ui, view, &mut actions);
                    }
                });
        });

    actions
}

fn configure_context(ctx: &egui::Context) {
    let configured_id = Id::new("some-frontier-egui-configured");
    if !ctx.data(|data| data.get_temp::<bool>(configured_id).unwrap_or(false)) {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "junicode".to_owned(),
            FontData::from_static(include_bytes!("../assets/fonts/Junicode.ttf")).into(),
        );
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, "junicode".to_owned());
        ctx.set_fonts(fonts);
        ctx.data_mut(|data| data.insert_temp(configured_id, true));
    }

    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(
        TextStyle::Heading,
        FontId::new(25.0, FontFamily::Proportional),
    );
    style
        .text_styles
        .insert(TextStyle::Body, FontId::new(17.0, FontFamily::Proportional));
    style.text_styles.insert(
        TextStyle::Button,
        FontId::new(16.0, FontFamily::Proportional),
    );
    style.spacing.item_spacing = Vec2::new(8.0, 6.0);
    style.spacing.interact_size.y = 28.0;
    style.visuals.panel_fill = Color32::TRANSPARENT;
    style.visuals.window_fill = Color32::TRANSPARENT;
    style.visuals.faint_bg_color = Color32::from_rgba_unmultiplied(10, 18, 24, 110);
    style.visuals.extreme_bg_color = Color32::from_rgba_unmultiplied(3, 9, 14, 225);
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgba_unmultiplied(8, 18, 24, 170);
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, cyan(105));
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgba_unmultiplied(18, 43, 50, 225);
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, cyan(210));
    style.visuals.widgets.active.bg_fill = Color32::from_rgba_unmultiplied(22, 54, 58, 235);
    style.visuals.widgets.active.bg_stroke = Stroke::new(1.0, cyan(245));
    style.visuals.selection.bg_fill = Color32::from_rgba_unmultiplied(28, 70, 74, 205);
    style.visuals.selection.stroke = Stroke::new(1.0, cyan(230));
    style.visuals.override_text_color = Some(text_color());
    ctx.set_style(style);
}

fn draw_wide(ui: &mut egui::Ui, view: &InventoryOverlayView, actions: &mut Vec<InventoryUiAction>) {
    let available = ui.available_width();
    let gap = 18.0;
    let detail_width = (available * 0.29).max(260.0);
    let production_width = (available * 0.36).max(340.0);
    let inventory_width = (available - detail_width - production_width - gap * 2.0).max(300.0);
    let height = ui.available_height();

    ui.horizontal_top(|ui| {
        ui.allocate_ui_with_layout(
            Vec2::new(detail_width, height),
            Layout::top_down(Align::Min),
            |ui| draw_ship_pane(ui, &view.ship, actions),
        );
        separator(ui, height);
        ui.allocate_ui_with_layout(
            Vec2::new(production_width, height),
            Layout::top_down(Align::Min),
            |ui| draw_production_pane(ui, view, actions),
        );
        separator(ui, height);
        ui.allocate_ui_with_layout(
            Vec2::new(inventory_width, height),
            Layout::top_down(Align::Min),
            |ui| draw_inventory_pane(ui, view),
        );
    });
}

fn draw_compact(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    view: &InventoryOverlayView,
    actions: &mut Vec<InventoryUiAction>,
) {
    let state_id = Id::new("inventory-compact-pane");
    let mut pane = ctx
        .data(|data| data.get_temp::<CompactPane>(state_id))
        .unwrap_or(CompactPane::Production);

    ui.horizontal(|ui| {
        if ui
            .selectable_label(pane == CompactPane::Ship, "Ship Pane")
            .clicked()
        {
            pane = CompactPane::Ship;
        }
        if ui
            .selectable_label(pane == CompactPane::Production, "Production")
            .clicked()
        {
            pane = CompactPane::Production;
        }
        if ui
            .selectable_label(pane == CompactPane::Inventory, "Inventory")
            .clicked()
        {
            pane = CompactPane::Inventory;
        }
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(RichText::new(format!("{} cr", view.credits)).color(muted_color()));
        });
    });
    ui.separator();
    match pane {
        CompactPane::Ship => draw_ship_pane(ui, &view.ship, actions),
        CompactPane::Production => draw_production_pane(ui, view, actions),
        CompactPane::Inventory => draw_inventory_pane(ui, view),
    }
    ctx.data_mut(|data| data.insert_temp(state_id, pane));
}

fn draw_ship_pane(ui: &mut egui::Ui, ship: &ShipPane, actions: &mut Vec<InventoryUiAction>) {
    heading(ui, "Ship Pane");
    ui.label(RichText::new(&ship.ship_name).size(20.0).color(cyan(255)));
    ui.label(RichText::new(&ship.cargo).color(cyan(220)));
    ui.separator();
    egui::Grid::new("ship-vitals")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            ship_stat(ui, "Hull", &ship.hull);
            ship_stat(ui, "Shield", &ship.shield);
            ship_stat(ui, "Energy", &ship.energy);
            ship_stat(ui, "Mass", &ship.mass);
            ship_stat(ui, "Engine", &ship.engine);
            ship_stat(ui, "Turn", &ship.turn);
        });

    egui::ScrollArea::vertical()
        .id_salt("ship-pane-scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            section_label(ui, "Power");
            for line in &ship.power {
                truncated_label(ui, line, muted_color());
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
                truncated_label(ui, line, muted_color());
            }
        });
}

fn draw_production_pane(
    ui: &mut egui::Ui,
    view: &InventoryOverlayView,
    actions: &mut Vec<InventoryUiAction>,
) {
    ui.horizontal(|ui| {
        heading(ui, "Production");
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            for (label, tab) in [
                ("Process", ProductionTab::Process),
                ("Craft", ProductionTab::Craft),
                ("Smelt", ProductionTab::Smelt),
            ] {
                if ui
                    .selectable_label(view.production_tab == tab, label)
                    .clicked()
                {
                    actions.push(InventoryUiAction::SetProductionTab(tab));
                }
            }
        });
    });

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

    TableBuilder::new(ui)
        .id_salt("production-table")
        .striped(true)
        .resizable(false)
        .column(Column::remainder().at_least(90.0).clip(true))
        .column(Column::exact(72.0))
        .column(Column::exact(54.0))
        .column(Column::exact(34.0))
        .column(Column::exact(22.0))
        .header(24.0, |mut header| {
            for label in ["Item", "Keep", "Status", "%", ""] {
                header.col(|ui| {
                    ui.label(RichText::new(label).color(header_color()));
                });
            }
        })
        .body(|body| {
            body.rows(ROW_HEIGHT, view.production.len(), |mut table_row| {
                let row_index = table_row.index();
                let row = &view.production[row_index];
                table_row.set_selected(row_index == selected);
                table_row.col(|ui| {
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
                });
                table_row.col(|ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 2.0;
                        if keep_chip(
                            ui,
                            "−",
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
                        if keep_chip(
                            ui,
                            "+",
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
                });
                table_row.col(|ui| {
                    truncated_label(ui, &row.status, value_color(row.enabled));
                });
                table_row.col(|ui| {
                    truncated_label(ui, &row.percent, value_color(row.enabled));
                });
                table_row.col(|ui| {
                    if row.active {
                        ui.label(RichText::new("◆").color(cyan(255)));
                    }
                });
            });
        });
    ui.ctx()
        .data_mut(|data| data.insert_temp(selected_id, selected));
}

fn draw_inventory_pane(ui: &mut egui::Ui, view: &InventoryOverlayView) {
    ui.horizontal(|ui| {
        heading(ui, "Inventory");
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(RichText::new(format!("Credits {}", view.credits)).color(muted_color()));
        });
    });
    TableBuilder::new(ui)
        .id_salt("inventory-table")
        .striped(true)
        .resizable(false)
        .column(Column::remainder().at_least(90.0).clip(true))
        .column(Column::exact(42.0))
        .column(Column::exact(70.0))
        .header(24.0, |mut header| {
            for label in ["Item", "Qty", "Mass"] {
                header.col(|ui| {
                    ui.label(RichText::new(label).color(header_color()));
                });
            }
        })
        .body(|mut body| {
            if view.inventory.is_empty() {
                body.row(ROW_HEIGHT, |mut row| {
                    row.col(|ui| {
                        ui.label(RichText::new("empty").color(muted_color()));
                    });
                    row.col(|ui| {
                        ui.label("0");
                    });
                    row.col(|ui| {
                        ui.label("0 kg");
                    });
                });
            } else {
                body.rows(ROW_HEIGHT, view.inventory.len(), |mut table_row| {
                    let row = &view.inventory[table_row.index()];
                    table_row.col(|ui| {
                        truncated_label(ui, &row.item, text_color());
                    });
                    table_row.col(|ui| {
                        ui.label(RichText::new(row.quantity.to_string()).color(cyan(255)));
                    });
                    table_row.col(|ui| {
                        truncated_label(ui, &row.mass, cyan(255));
                    });
                });
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
    ui.label(RichText::new(&slot.label).color(header_color()));
    egui::ComboBox::from_id_salt((id_prefix, index))
        .selected_text(&slot.installed_name)
        .width(ui.available_width())
        .show_ui(ui, |ui| {
            if slot.choices.is_empty() {
                ui.label(RichText::new("No compatible equipment in cargo").color(muted_color()));
            }
            for choice in &slot.choices {
                let selected = slot.installed_id.as_deref() == Some(choice.id.as_str());
                if ui.selectable_label(selected, &choice.name).clicked() && !selected {
                    install(choice.id.clone());
                }
            }
        });
}

fn keep_chip(ui: &mut egui::Ui, icon: &str, tooltip: &str, enabled: bool) -> egui::Response {
    ui.scope(|ui| {
        ui.spacing_mut().button_padding = Vec2::ZERO;
        ui.add_enabled(
            enabled,
            egui::Button::new(RichText::new(icon).size(13.0))
                .min_size(Vec2::splat(18.0))
                .corner_radius(9.0),
        )
        .on_hover_text(tooltip)
    })
    .inner
}

fn heading(ui: &mut egui::Ui, text: &str) {
    ui.label(
        RichText::new(text)
            .heading()
            .color(Color32::from_rgb(235, 242, 226)),
    );
}

fn section_label(ui: &mut egui::Ui, text: &str) {
    ui.add_space(6.0);
    ui.separator();
    ui.label(RichText::new(text).color(header_color()));
}

fn ship_stat(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.label(RichText::new(label).color(muted_color()));
    truncated_label(ui, value, text_color());
    ui.end_row();
}

fn truncated_label(ui: &mut egui::Ui, text: &str, color: Color32) {
    ui.add(egui::Label::new(RichText::new(text).color(color)).truncate())
        .on_hover_text(text);
}

fn separator(ui: &mut egui::Ui, height: f32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(1.0, height), egui::Sense::hover());
    ui.painter()
        .vline(rect.center().x, rect.y_range(), Stroke::new(1.0, cyan(105)));
}

fn cyan(alpha: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(150, 221, 226, alpha)
}

fn text_color() -> Color32 {
    Color32::from_rgb(205, 226, 230)
}

fn header_color() -> Color32 {
    Color32::from_rgb(168, 204, 210)
}

fn muted_color() -> Color32 {
    Color32::from_rgb(126, 156, 164)
}

fn value_color(enabled: bool) -> Color32 {
    if enabled {
        cyan(255)
    } else {
        Color32::from_rgb(126, 143, 148)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlay_rect_stays_inside_supported_window_sizes() {
        for screen in [
            [640.0, 420.0],
            [1100.0, 760.0],
            [1280.0, 720.0],
            [1600.0, 900.0],
        ] {
            let rect = overlay_rect(screen);
            assert!(rect.x >= 0.0 && rect.y >= 0.0);
            assert!(rect.x + rect.width <= screen[0]);
            assert!(rect.y + rect.height <= screen[1]);
        }
    }

    #[test]
    fn compact_layout_is_used_before_three_panes_can_fit() {
        assert!(!uses_wide_layout(overlay_rect([960.0, 500.0])));
        assert!(uses_wide_layout(overlay_rect([1100.0, 760.0])));
    }

    #[test]
    fn inset_leaves_a_usable_compact_content_area() {
        let rect = overlay_rect([640.0, 420.0]);

        assert!(rect.width - f32::from(CONTENT_MARGIN_X) * 2.0 >= 500.0);
        assert!(rect.height - f32::from(CONTENT_MARGIN_Y) * 2.0 >= 320.0);
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
}
