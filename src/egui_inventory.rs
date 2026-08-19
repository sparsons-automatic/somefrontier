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
    pub output: String,
    pub keep: u32,
    pub status: String,
    pub percent: String,
    pub active: bool,
    pub activity: ProductionActivity,
    pub enabled: bool,
    pub unlocked: bool,
    pub requirements: Vec<String>,
    pub sub_requirements: Vec<String>,
    pub equipment_links: Vec<String>,
    pub could_make: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProductionActivity {
    Active(String),
    Blocked(String),
    Waiting(String),
    Idle(String),
}

impl ProductionActivity {
    fn label(&self) -> String {
        match self {
            Self::Active(detail) => format!("Active · {detail}"),
            Self::Blocked(detail) => format!("Blocked · {detail}"),
            Self::Waiting(detail) => format!("Waiting · {detail}"),
            Self::Idle(detail) => format!("Idle · {detail}"),
        }
    }

    fn color(&self, ui: &egui::Ui) -> egui::Color32 {
        match self {
            Self::Active(_) => production_active_color(),
            Self::Blocked(_) => ui.visuals().warn_fg_color,
            Self::Waiting(_) => egui::Color32::from_rgb(225, 184, 92),
            Self::Idle(_) => ui.visuals().weak_text_color(),
        }
    }
}

impl ProductionRow {
    fn accepts_input(&self) -> bool {
        self.unlocked
    }
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
    pub auto_repair_status: String,
    pub auto_repair_toggle: Option<String>,
    pub power: Vec<String>,
    pub shields: Vec<EquipmentSlot>,
    pub weapons: Vec<EquipmentSlot>,
    pub feedback: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InventoryOverlayView {
    pub keep_click_step: i32,
    pub production_tab: ProductionTab,
    pub production: Vec<ProductionRow>,
    pub show_production: bool,
    pub production_pinned: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InventoryUiAction {
    SetProductionTab(ProductionTab),
    AdjustKeep { row: usize, direction: i32 },
    QueueRecipe { row: usize },
    ToggleProductionPin,
}

pub fn draw(
    ctx: &egui::Context,
    screen: [f32; 2],
    view: &InventoryOverlayView,
) -> Vec<InventoryUiAction> {
    draw_work_windows(ctx, screen, view)
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
            .show(ctx, |ui| {
                if crate::egui_pin::toggle(ui, view.production_pinned) {
                    actions.push(InventoryUiAction::ToggleProductionPin);
                }
                draw_production_pane(ui, view, &mut actions);
            });
    }

    actions
}

fn default_window_height(screen_height: f32, preferred_height: f32) -> f32 {
    preferred_height.min((screen_height - 96.0).max(240.0))
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

    crate::egui_pane::both_scroll(ui, "production-grid-scroll", |ui| {
        egui::Grid::new("production-grid")
            .striped(true)
            .min_col_width(44.0)
            .show(ui, |ui| {
                for header in ["Item", "Keep", "Status", "%", "Active"] {
                    ui.strong(header);
                }
                ui.end_row();

                for (row_index, row) in view.production.iter().enumerate() {
                    let item_label = production_item_label(row);
                    let response = ui.add(egui::Label::new(item_label).sense(egui::Sense::click()));
                    let mut row_rect = response.rect;
                    if response.clicked() {
                        selected = row_index;
                        if row.accepts_input() {
                            actions.push(InventoryUiAction::QueueRecipe { row: row_index });
                        }
                    }
                    response.on_hover_ui(|ui| draw_recipe_info_card(ui, row));

                    let keep_response = ui.horizontal(|ui| {
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
                    row_rect = row_rect.union(keep_response.response.rect);
                    if row.active {
                        let status_response = ui.colored_label(
                            production_active_color(),
                            egui::RichText::new(&row.status).strong(),
                        );
                        let status_response =
                            status_response.on_hover_ui(|ui| draw_recipe_info_card(ui, row));
                        row_rect = row_rect.union(status_response.rect);
                        let percent_response = ui.colored_label(
                            production_active_color(),
                            egui::RichText::new(&row.percent).strong(),
                        );
                        row_rect = row_rect.union(percent_response.rect);
                        let active_response = ui.colored_label(production_active_color(), "●");
                        row_rect = row_rect.union(active_response.rect);
                        ui.painter().rect_filled(
                            row_rect.expand2(egui::vec2(4.0, 2.0)),
                            2.0,
                            production_active_tint(),
                        );
                    } else {
                        ui.add_enabled(row.enabled, egui::Label::new(&row.status))
                            .on_hover_ui(|ui| draw_recipe_info_card(ui, row));
                        ui.add_enabled(row.enabled, egui::Label::new(&row.percent));
                        ui.weak("—");
                    }
                    ui.end_row();
                }
            });
    });
    ui.ctx()
        .data_mut(|data| data.insert_temp(selected_id, selected));
}

fn production_active_color() -> egui::Color32 {
    egui::Color32::from_rgb(104, 211, 153)
}

fn production_active_tint() -> egui::Color32 {
    egui::Color32::from_rgba_premultiplied(45, 92, 67, 22)
}

fn production_item_label(row: &ProductionRow) -> egui::RichText {
    if row.active {
        egui::RichText::new(format!("▶ {}", row.item))
            .strong()
            .color(production_active_color())
    } else {
        egui::RichText::new(&row.item)
    }
}

pub fn draw_production_page(
    ui: &mut egui::Ui,
    view: &InventoryOverlayView,
) -> Vec<InventoryUiAction> {
    let mut actions = Vec::new();
    draw_production_pane(ui, view, &mut actions);
    actions
}

fn draw_recipe_info_card(ui: &mut egui::Ui, row: &ProductionRow) {
    ui.set_max_width(340.0);
    ui.strong(&row.item);
    ui.label("Production recipe");
    ui.colored_label(
        row.activity.color(ui),
        egui::RichText::new(row.activity.label()).strong(),
    );
    info_card_section(ui, "Produces", std::slice::from_ref(&row.output));
    info_card_section(ui, "Requires", &row.requirements);
    if !row.sub_requirements.is_empty() {
        info_card_section(ui, "Sub-requirements", &row.sub_requirements);
    }
    if !row.equipment_links.is_empty() {
        info_card_section(ui, "Equipment compatibility", &row.equipment_links);
    }
    info_card_section_with_empty(
        ui,
        "Could make (discovered)",
        &row.could_make,
        "No discovered follow-on recipes",
    );
}

fn info_card_section(ui: &mut egui::Ui, heading: &str, lines: &[String]) {
    info_card_section_with_empty(ui, heading, lines, "None");
}

fn info_card_section_with_empty(ui: &mut egui::Ui, heading: &str, lines: &[String], empty: &str) {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn production_scroll_test_view() -> InventoryOverlayView {
        InventoryOverlayView {
            keep_click_step: 1,
            production_tab: ProductionTab::Smelt,
            production: (0..40)
                .map(|index| ProductionRow {
                    item: format!("Recipe {index}"),
                    output: format!("Recipe {index} x1"),
                    keep: 0,
                    status: "Ready".to_string(),
                    percent: "0%".to_string(),
                    active: false,
                    activity: ProductionActivity::Idle("Not queued".to_string()),
                    enabled: true,
                    unlocked: true,
                    requirements: Vec::new(),
                    sub_requirements: Vec::new(),
                    equipment_links: Vec::new(),
                    could_make: Vec::new(),
                })
                .collect(),
            show_production: true,
            production_pinned: false,
        }
    }

    fn draw_scroll_test_frame(
        ctx: &egui::Context,
        view: &InventoryOverlayView,
        events: Vec<egui::Event>,
    ) -> Id {
        let scroll_id = std::cell::Cell::new(None);
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                Vec2::new(600.0, 320.0),
            )),
            events,
            ..Default::default()
        };
        let _ = ctx.run(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                scroll_id.set(Some(
                    ui.make_persistent_id(Id::new("production-grid-scroll")),
                ));
                let mut actions = Vec::new();
                draw_production_pane(ui, view, &mut actions);
            });
        });
        scroll_id.get().expect("production scroll area should draw")
    }

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
            activity: ProductionActivity::Idle("Not queued".to_string()),
            enabled: false,
            unlocked: true,
            requirements: Vec::new(),
            output: "Test plate x1".to_string(),
            sub_requirements: Vec::new(),
            equipment_links: Vec::new(),
            could_make: Vec::new(),
        };

        assert!(row.accepts_input());
    }

    #[test]
    fn keep_chips_expand_to_show_the_active_modifier_step() {
        assert_eq!(keep_chip_label("+", 1), "+");
        assert_eq!(keep_chip_label("−", 10), "−10");
        assert_eq!(keep_chip_label("+", 20), "+20");
    }

    #[test]
    fn active_production_rows_receive_a_distinct_working_label() {
        let mut view = production_scroll_test_view();
        let idle = production_item_label(&view.production[0]);
        view.production[0].active = true;
        let active = production_item_label(&view.production[0]);

        assert_eq!(idle.text(), "Recipe 0");
        assert_eq!(active.text(), "▶ Recipe 0");
    }

    #[test]
    fn production_recipe_grid_consumes_wheel_input() {
        let ctx = egui::Context::default();
        let view = production_scroll_test_view();
        let scroll_id = draw_scroll_test_frame(&ctx, &view, Vec::new());

        let scroll_id_after = draw_scroll_test_frame(
            &ctx,
            &view,
            vec![
                egui::Event::PointerMoved(egui::pos2(120.0, 140.0)),
                egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta: egui::vec2(0.0, -120.0),
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );

        assert_eq!(scroll_id_after, scroll_id);
        let offset = egui::scroll_area::State::load(&ctx, scroll_id)
            .expect("production scroll state should persist")
            .offset;
        assert!(offset.y > 0.0, "wheel input should scroll the recipe grid");
    }
}
