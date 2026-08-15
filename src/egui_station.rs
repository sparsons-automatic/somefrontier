use egui_macroquad::egui::{self, Id, Vec2};

#[derive(Clone, Debug, PartialEq)]
pub struct ServiceRow {
    pub name: String,
    pub context: String,
    pub description: String,
    pub artwork: Option<String>,
    pub available: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TradeRow {
    pub item: String,
    pub stock: String,
    pub cargo: u32,
    pub buy_price: u32,
    pub sell_price: u32,
    pub can_buy: bool,
    pub can_sell: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContractRow {
    pub name: String,
    pub description: String,
    pub objective: String,
    pub reward: u32,
    pub duration_days: f32,
    pub status: String,
    pub actionable: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecipeRow {
    pub name: String,
    pub status: String,
    pub purchasable: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResearchRow {
    pub name: String,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShipOffer {
    pub ship_id: String,
    pub name: String,
    pub price: u32,
    pub purchasable: bool,
    pub status: String,
    pub cargo_capacity: String,
    pub mass: String,
    pub forward_acceleration: f32,
    pub turn_acceleration: f32,
    pub mining_profile: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StationPaneView {
    pub pinned: bool,
    pub name: String,
    pub artwork: Option<String>,
    pub content_id: String,
    pub summary: String,
    pub ownership: String,
    pub proximity: String,
    pub credits: u32,
    pub services: Vec<ServiceRow>,
    pub selected_service: Option<usize>,
    pub service_description: Option<String>,
    pub service_status: Option<String>,
    pub trade: Vec<TradeRow>,
    pub garage: bool,
    pub repair_label: String,
    pub repair_enabled: bool,
    pub contracts: Vec<ContractRow>,
    pub recipes: Vec<RecipeRow>,
    pub research: Vec<ResearchRow>,
    pub shipyard: Vec<ShipOffer>,
    pub latest_operation: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StationUiAction {
    TogglePin,
    SelectService(usize),
    Buy(usize),
    Sell(usize),
    Repair,
    Contract(usize),
    PurchaseRecipe(usize),
    BuyShip(String),
}

pub fn draw(ctx: &egui::Context, screen: [f32; 2], view: &StationPaneView) -> Vec<StationUiAction> {
    ctx.set_style(egui::Style::default());
    let mut actions = Vec::new();
    let height = 620.0_f32.min((screen[1] - 96.0).max(300.0));
    let width = 900.0_f32.min((screen[0] - 32.0).max(360.0));
    let has_selected_service = selected_service(&view.services, view.selected_service).is_some();

    let window_id = Id::new("station-workspace-window");
    crate::egui_pane::show_window(
        ctx,
        window_id,
        egui::Window::new("Station")
            .id(window_id)
            .default_pos(egui::pos2(16.0, 48.0))
            .default_size(Vec2::new(width, height))
            .min_width(if has_selected_service {
                680.0_f32.min(width)
            } else {
                360.0
            })
            .resizable(true)
            .collapsible(true),
        |ui| {
            crate::egui_pane::vertical_scroll(ui, "station-content-scroll", |ui| {
                draw_contents(ui, view, &mut actions);
            });
        },
    );

    actions
}

fn draw_contents(ui: &mut egui::Ui, view: &StationPaneView, actions: &mut Vec<StationUiAction>) {
    if crate::egui_pin::toggle(ui, view.pinned) {
        actions.push(StationUiAction::TogglePin);
    }
    crate::egui_art::preview(ui, view.artwork.as_deref(), Vec2::new(260.0, 160.0));
    ui.heading(&view.name);
    ui.small(&view.content_id);
    ui.label(&view.summary);
    egui::Grid::new("station-facts-grid")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            fact_row(ui, "Ownership", &view.ownership);
            fact_row(ui, "Proximity", &view.proximity);
            fact_row(ui, "Credits", &view.credits.to_string());
        });

    ui.separator();
    ui.heading("Services");
    if view.services.is_empty() {
        ui.label("No services declared");
        return;
    }

    if selected_service(&view.services, view.selected_service).is_some() {
        ui.columns(2, |columns| {
            draw_service_cards(&mut columns[0], view, actions);
            egui::Frame::group(columns[1].style())
                .inner_margin(10.0)
                .show(&mut columns[1], |ui| draw_service_rail(ui, view, actions));
        });
    } else {
        draw_service_cards(ui, view, actions);
    }
}

fn selected_service(services: &[ServiceRow], selected: Option<usize>) -> Option<&ServiceRow> {
    selected.and_then(|selected| services.get(selected))
}

fn draw_service_cards(
    ui: &mut egui::Ui,
    view: &StationPaneView,
    actions: &mut Vec<StationUiAction>,
) {
    for (index, service) in view.services.iter().enumerate() {
        let label = format!(
            "{}\n{}\n{}",
            service.name, service.description, service.context
        );
        let response = crate::egui_art::card_button(
            ui,
            service.artwork.as_deref(),
            label,
            view.selected_service == Some(index),
            service.available,
        );
        if response.clicked() {
            actions.push(StationUiAction::SelectService(index));
        }
    }
}

fn draw_service_rail(
    ui: &mut egui::Ui,
    view: &StationPaneView,
    actions: &mut Vec<StationUiAction>,
) {
    let Some(service) = selected_service(&view.services, view.selected_service) else {
        return;
    };
    ui.heading(&service.name);
    ui.label(&service.context);
    if let Some(status) = &view.service_status {
        ui.label(status);
    }
    if let Some(description) = &view.service_description {
        ui.label(description);
    }

    if view.garage {
        ui.separator();
        ui.heading("Maintenance");
        if ui
            .add_enabled(view.repair_enabled, egui::Button::new(&view.repair_label))
            .clicked()
        {
            actions.push(StationUiAction::Repair);
        }
    }
    draw_trade(ui, view, actions);
    draw_contracts(ui, view, actions);
    draw_recipes(ui, view, actions);
    draw_research(ui, view);
    draw_shipyard(ui, view, actions);

    if let Some(latest) = &view.latest_operation {
        ui.separator();
        ui.label(format!("Latest: {latest}"));
    }
}

fn draw_shipyard(ui: &mut egui::Ui, view: &StationPaneView, actions: &mut Vec<StationUiAction>) {
    if view.shipyard.is_empty() {
        return;
    }
    ui.separator();
    ui.heading("Available hulls");
    for offer in &view.shipyard {
        ui.group(|ui| {
            ui.strong(&offer.name);
            ui.small(format!(
                "Cargo {} · Mass {} · Thrust {:.0} · Turn {:.1}",
                offer.cargo_capacity,
                offer.mass,
                offer.forward_acceleration,
                offer.turn_acceleration
            ));
            if let Some(profile) = &offer.mining_profile {
                ui.small(profile);
            }
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(
                        offer.purchasable,
                        egui::Button::new(format!("Buy · {} cr", offer.price)),
                    )
                    .clicked()
                {
                    actions.push(StationUiAction::BuyShip(offer.ship_id.clone()));
                }
                ui.small(&offer.status);
            });
        });
    }
}

fn draw_trade(ui: &mut egui::Ui, view: &StationPaneView, actions: &mut Vec<StationUiAction>) {
    if view.trade.is_empty() {
        return;
    }
    ui.separator();
    ui.heading("Market");
    egui::Grid::new("station-trade-grid")
        .striped(true)
        .show(ui, |ui| {
            for heading in ["Item", "Stock", "Cargo", "Buy", "Sell"] {
                ui.strong(heading);
            }
            ui.end_row();
            for (index, row) in view.trade.iter().enumerate() {
                ui.label(&row.item);
                ui.label(&row.stock);
                ui.label(row.cargo.to_string());
                if ui
                    .add_enabled(
                        row.can_buy,
                        egui::Button::new(format!("{} cr", row.buy_price)),
                    )
                    .clicked()
                {
                    actions.push(StationUiAction::Buy(index));
                }
                if ui
                    .add_enabled(
                        row.can_sell,
                        egui::Button::new(format!("{} cr", row.sell_price)),
                    )
                    .clicked()
                {
                    actions.push(StationUiAction::Sell(index));
                }
                ui.end_row();
            }
        });
}

fn draw_contracts(ui: &mut egui::Ui, view: &StationPaneView, actions: &mut Vec<StationUiAction>) {
    if view.contracts.is_empty() {
        return;
    }
    ui.separator();
    ui.heading("Contracts");
    for (index, contract) in view.contracts.iter().enumerate() {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.strong(&contract.name);
                ui.label(format!(
                    "{} cr · {:.1}d",
                    contract.reward, contract.duration_days
                ));
                if ui
                    .add_enabled(contract.actionable, egui::Button::new(&contract.status))
                    .clicked()
                {
                    actions.push(StationUiAction::Contract(index));
                }
            });
            ui.label(&contract.description);
            ui.small(&contract.objective);
        });
    }
}

fn draw_recipes(ui: &mut egui::Ui, view: &StationPaneView, actions: &mut Vec<StationUiAction>) {
    if view.recipes.is_empty() {
        return;
    }
    ui.separator();
    ui.heading("Recipe unlocks");
    egui::Grid::new("station-recipes-grid")
        .striped(true)
        .show(ui, |ui| {
            for (index, recipe) in view.recipes.iter().enumerate() {
                ui.label(&recipe.name);
                if ui
                    .add_enabled(recipe.purchasable, egui::Button::new(&recipe.status))
                    .clicked()
                {
                    actions.push(StationUiAction::PurchaseRecipe(index));
                }
                ui.end_row();
            }
        });
}

fn draw_research(ui: &mut egui::Ui, view: &StationPaneView) {
    if view.research.is_empty() {
        return;
    }
    ui.separator();
    ui.heading("Research leads");
    egui::Grid::new("station-research-grid")
        .striped(true)
        .show(ui, |ui| {
            for lead in &view.research {
                ui.label(&lead.name);
                ui.label(&lead.status);
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

    fn service_row(name: &str) -> ServiceRow {
        ServiceRow {
            name: name.to_string(),
            context: "Dock range reached".to_string(),
            description: "Service description".to_string(),
            artwork: None,
            available: true,
        }
    }

    #[test]
    fn station_actions_are_typed_by_row() {
        assert_eq!(StationUiAction::Buy(2), StationUiAction::Buy(2));
        assert_ne!(StationUiAction::Buy(2), StationUiAction::Sell(2));
    }

    #[test]
    fn service_detail_rail_requires_a_valid_selection() {
        let services = vec![service_row("Supply Counter")];

        assert!(selected_service(&services, None).is_none());
        assert!(selected_service(&services, Some(1)).is_none());
        assert_eq!(
            selected_service(&services, Some(0)).map(|service| service.name.as_str()),
            Some("Supply Counter")
        );
    }
}
