use egui_macroquad::egui::{self, Id, Vec2};

#[derive(Clone, Debug, PartialEq)]
pub struct InteractionRow {
    pub label: String,
    pub status: String,
    pub enabled: bool,
    pub complete: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TradeRow {
    pub item: String,
    pub stock: u32,
    pub cargo: u32,
    pub buy_price: u32,
    pub sell_price: u32,
    pub buy_amount: u32,
    pub sell_amount: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NpcPaneView {
    pub pinned: bool,
    pub name: String,
    pub artwork: Option<String>,
    pub content_id: String,
    pub proximity: String,
    pub identified: bool,
    pub hostile: bool,
    pub intel_level: String,
    pub scan_stage: String,
    pub scan_progress: Option<f32>,
    pub scan_status: String,
    pub identity: String,
    pub disposition: String,
    pub systems: String,
    pub loadout: String,
    pub defenses: Vec<String>,
    pub cargo_summary: String,
    pub summary: String,
    pub interactions: Vec<InteractionRow>,
    pub trade_open: bool,
    pub trade_available: bool,
    pub trade_quantity: u32,
    pub quantity_step: u32,
    pub credits: u32,
    pub trade: Vec<TradeRow>,
    pub latest_operation: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NpcUiAction {
    TogglePin,
    Interaction(usize),
    CloseTrade,
    AdjustTradeQuantity(i32),
    Buy(usize),
    Sell(usize),
}

pub fn draw(ctx: &egui::Context, screen: [f32; 2], view: &NpcPaneView) -> Vec<NpcUiAction> {
    ctx.set_style(egui::Style::default());
    let mut actions = Vec::new();
    let height = 620.0_f32.min((screen[1] - 96.0).max(300.0));

    egui::Window::new("Contact")
        .id(Id::new("npc-contact-window"))
        .default_pos(egui::pos2(16.0, 48.0))
        .default_size(Vec2::new(480.0, height))
        .min_width(340.0)
        .resizable(true)
        .collapsible(true)
        .show(ctx, |ui| {
            crate::egui_pane::vertical_scroll(ui, "contact-content-scroll", |ui| {
                draw_contents(ui, view, &mut actions);
            });
        });

    actions
}

fn draw_contents(ui: &mut egui::Ui, view: &NpcPaneView, actions: &mut Vec<NpcUiAction>) {
    if crate::egui_pin::toggle(ui, view.pinned) {
        actions.push(NpcUiAction::TogglePin);
    }
    crate::egui_art::preview(ui, view.artwork.as_deref(), Vec2::new(240.0, 150.0));
    ui.heading(&view.name);
    ui.small(&view.content_id);
    ui.horizontal(|ui| {
        ui.label(&view.proximity);
        if view.hostile {
            ui.colored_label(ui.visuals().error_fg_color, "Hostile");
        }
    });
    ui.horizontal(|ui| {
        ui.strong(format!("Intel: {}", view.intel_level));
        ui.label(&view.scan_stage);
    });
    if let Some(progress) = view.scan_progress {
        ui.add(
            egui::ProgressBar::new(progress.clamp(0.0, 1.0))
                .show_percentage()
                .text(&view.scan_status),
        );
    } else {
        ui.small(&view.scan_status);
    }

    egui::Grid::new("npc-contact-facts")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            fact_row(ui, "Identity", &view.identity);
            fact_row(ui, "Disposition", &view.disposition);
            fact_row(ui, "Systems", &view.systems);
            fact_row(ui, "Loadout", &view.loadout);
            fact_row(ui, "Cargo", &view.cargo_summary);
        });
    for defense in &view.defenses {
        ui.small(defense);
    }
    ui.label(&view.summary);

    ui.separator();
    if view.trade_open {
        draw_trade(ui, view, actions);
    } else {
        draw_interactions(ui, view, actions);
    }

    if let Some(latest) = &view.latest_operation {
        ui.separator();
        ui.label(format!("Latest: {latest}"));
    }
}

fn draw_interactions(ui: &mut egui::Ui, view: &NpcPaneView, actions: &mut Vec<NpcUiAction>) {
    ui.heading("Interactions");
    egui::Grid::new("npc-contact-interactions")
        .striped(true)
        .show(ui, |ui| {
            for (index, row) in view.interactions.iter().enumerate() {
                if ui
                    .add_enabled(
                        row.enabled,
                        egui::Button::new(&row.label).selected(row.complete),
                    )
                    .clicked()
                {
                    actions.push(NpcUiAction::Interaction(index));
                }
                ui.label(&row.status);
                ui.end_row();
            }
        });

    if !view.identified {
        ui.small("Keep the selected contact in scanner range to reveal its identity and services.");
    }
}

fn draw_trade(ui: &mut egui::Ui, view: &NpcPaneView, actions: &mut Vec<NpcUiAction>) {
    ui.horizontal(|ui| {
        ui.heading("Cargo market");
        if ui.button("Back to contact").clicked() {
            actions.push(NpcUiAction::CloseTrade);
        }
    });
    ui.horizontal(|ui| {
        ui.label("Quantity");
        if ui
            .add_enabled(view.trade_available, egui::Button::new("−").small())
            .on_hover_text(format!("Decrease by {}", view.quantity_step))
            .clicked()
        {
            actions.push(NpcUiAction::AdjustTradeQuantity(-1));
        }
        ui.label(view.trade_quantity.to_string());
        if ui
            .add_enabled(view.trade_available, egui::Button::new("+").small())
            .on_hover_text(format!("Increase by {}", view.quantity_step))
            .clicked()
        {
            actions.push(NpcUiAction::AdjustTradeQuantity(1));
        }
        ui.separator();
        ui.label(format!("Credits {}", view.credits));
    });
    ui.small(format!(
        "Quantity step {} · hold Shift for ±10 or Ctrl for ±20",
        view.quantity_step
    ));

    if !view.trade_available {
        ui.label("Approach this identified, non-hostile trader to exchange cargo.");
    }
    if view.trade.is_empty() {
        ui.label("No cargo available");
        return;
    }

    egui::Grid::new("npc-contact-trade")
        .striped(true)
        .show(ui, |ui| {
            for heading in ["Item", "Stock", "Cargo", "Buy", "Sell"] {
                ui.strong(heading);
            }
            ui.end_row();
            for (index, row) in view.trade.iter().enumerate() {
                ui.label(&row.item);
                ui.label(row.stock.to_string());
                ui.label(row.cargo.to_string());
                let buy_label = if row.buy_amount > 0 {
                    format!(
                        "x{} · {} cr",
                        row.buy_amount,
                        row.buy_price.saturating_mul(row.buy_amount)
                    )
                } else {
                    "Unavailable".to_string()
                };
                if ui
                    .add_enabled(row.buy_amount > 0, egui::Button::new(buy_label))
                    .clicked()
                {
                    actions.push(NpcUiAction::Buy(index));
                }
                let sell_label = if row.sell_amount > 0 {
                    format!(
                        "x{} · {} cr",
                        row.sell_amount,
                        row.sell_price.saturating_mul(row.sell_amount)
                    )
                } else {
                    "No cargo".to_string()
                };
                if ui
                    .add_enabled(row.sell_amount > 0, egui::Button::new(sell_label))
                    .clicked()
                {
                    actions.push(NpcUiAction::Sell(index));
                }
                ui.end_row();
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
    fn trade_actions_keep_buy_and_sell_rows_distinct() {
        assert_ne!(NpcUiAction::Buy(1), NpcUiAction::Sell(1));
        assert_eq!(NpcUiAction::Interaction(2), NpcUiAction::Interaction(2));
    }
}
