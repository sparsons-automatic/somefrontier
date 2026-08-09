use egui_macroquad::egui::{self, Id, Vec2};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResearchState {
    Completed,
    Researching,
    Affordable,
    Available,
    Locked,
}

impl ResearchState {
    pub fn label(self) -> &'static str {
        match self {
            Self::Completed => "Completed",
            Self::Researching => "Researching",
            Self::Affordable => "Affordable",
            Self::Available => "Available",
            Self::Locked => "Locked",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResearchNode {
    pub id: String,
    pub name: String,
    pub tier: u32,
    pub column: i32,
    pub row: i32,
    pub state: ResearchState,
    pub progress: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResearchEdge {
    pub from: String,
    pub to: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResearchDetail {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub state: ResearchState,
    pub price: u32,
    pub duration: String,
    pub requirements: Vec<String>,
    pub rewards: Vec<String>,
    pub progress: Option<f32>,
    pub remaining: Option<String>,
    pub start_enabled: bool,
    pub action_label: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResearchView {
    pub pinned: bool,
    pub credits: u32,
    pub nodes: Vec<ResearchNode>,
    pub edges: Vec<ResearchEdge>,
    pub selected: Option<String>,
    pub detail: Option<ResearchDetail>,
    pub latest_operation: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResearchUiAction {
    TogglePin,
    Select(String),
    Start(String),
}

pub fn draw(ctx: &egui::Context, screen: [f32; 2], view: &ResearchView) -> Vec<ResearchUiAction> {
    ctx.set_style(egui::Style::default());
    let mut actions = Vec::new();
    let height = 640.0_f32.min((screen[1] - 96.0).max(360.0));
    let min_size = Vec2::new(
        680.0_f32.min((screen[0] - 40.0).max(320.0)),
        360.0_f32.min((screen[1] - 40.0).max(280.0)),
    );

    egui::Window::new("Research")
        .id(Id::new("research-workspace-window"))
        .default_pos(egui::pos2(36.0, 36.0))
        .default_size(Vec2::new(1080.0_f32.min(screen[0] - 72.0), height))
        .min_size(min_size)
        .resizable(true)
        .collapsible(true)
        .show(ctx, |ui| draw_contents(ui, view, &mut actions));

    actions
}

fn draw_contents(ui: &mut egui::Ui, view: &ResearchView, actions: &mut Vec<ResearchUiAction>) {
    if crate::egui_pin::toggle(ui, view.pinned) {
        actions.push(ResearchUiAction::TogglePin);
    }
    ui.horizontal(|ui| {
        ui.heading("Research");
        ui.separator();
        ui.label(format!("Credits {}", view.credits));
        ui.separator();
        ui.label(format!("{} modules", view.nodes.len()));
    });
    ui.small("Tiers and their order come from loaded content packs. Scroll horizontally for additional tiers.");
    ui.separator();

    let pane_height = ui.available_height().max(240.0);
    let available_width = ui.available_width();
    let (tier_width, divider_gap, detail_width) = research_pane_widths(available_width);
    let (body_rect, _) = ui.allocate_exact_size(
        Vec2::new(available_width, pane_height),
        egui::Sense::hover(),
    );
    let tier_rect = egui::Rect::from_min_size(body_rect.min, Vec2::new(tier_width, pane_height));
    let detail_rect = egui::Rect::from_min_size(
        egui::pos2(tier_rect.right() + divider_gap, body_rect.top()),
        Vec2::new(detail_width, pane_height),
    );

    let mut tier_ui = ui.new_child(egui::UiBuilder::new().max_rect(tier_rect));
    tier_ui.set_clip_rect(tier_rect);
    egui::ScrollArea::horizontal()
        .id_salt("research-tier-browser")
        .auto_shrink([false, true])
        .show(&mut tier_ui, |ui| {
            ui.horizontal_top(|ui| {
                for tier in research_tiers(&view.nodes) {
                    draw_tier(ui, view, tier, pane_height, actions);
                    ui.separator();
                }
            });
        });

    ui.painter().vline(
        tier_rect.right() + divider_gap * 0.5,
        body_rect.y_range(),
        ui.visuals().widgets.noninteractive.bg_stroke,
    );
    let mut detail_ui = ui.new_child(egui::UiBuilder::new().max_rect(detail_rect));
    detail_ui.set_clip_rect(detail_rect);
    draw_detail(&mut detail_ui, view, pane_height, detail_width, actions);
}

fn research_pane_widths(available_width: f32) -> (f32, f32, f32) {
    let available_width = available_width.max(0.0);
    let divider_gap = 12.0_f32.min(available_width);
    let detail_width = (available_width * 0.42)
        .clamp(280.0, 360.0)
        .min(available_width - divider_gap);
    let tier_width = available_width - divider_gap - detail_width;
    (tier_width, divider_gap, detail_width)
}

fn research_tiers(nodes: &[ResearchNode]) -> Vec<u32> {
    let mut tiers = nodes.iter().map(|node| node.tier).collect::<Vec<_>>();
    tiers.sort_unstable();
    tiers.dedup();
    tiers
}

fn draw_tier(
    ui: &mut egui::Ui,
    view: &ResearchView,
    tier: u32,
    height: f32,
    actions: &mut Vec<ResearchUiAction>,
) {
    let mut nodes = view
        .nodes
        .iter()
        .filter(|node| node.tier == tier)
        .collect::<Vec<_>>();
    nodes.sort_by(|left, right| {
        left.column
            .cmp(&right.column)
            .then(left.row.cmp(&right.row))
            .then(left.name.cmp(&right.name))
    });

    ui.vertical(|ui| {
        ui.set_width(230.0);
        ui.heading(format!("Tier {tier}"));
        ui.small(format!("{} module(s)", nodes.len()));
        ui.separator();
        egui::ScrollArea::vertical()
            .id_salt(("research-tier-list", tier))
            .max_height(height - 58.0)
            .show(ui, |ui| {
                for node in nodes {
                    let selected = view.selected.as_deref() == Some(node.id.as_str());
                    let response = ui.add_sized(
                        [ui.available_width(), 64.0],
                        egui::Button::new(format!("{}\n{}", node.name, node.state.label()))
                            .selected(selected),
                    );
                    if response.clicked() {
                        actions.push(ResearchUiAction::Select(node.id.clone()));
                    }
                    if let Some(progress) = node.progress {
                        ui.add(
                            egui::ProgressBar::new(progress)
                                .desired_width(ui.available_width())
                                .text("Researching"),
                        );
                    }
                    ui.add_space(4.0);
                }
            });
    });
}

fn draw_detail(
    ui: &mut egui::Ui,
    view: &ResearchView,
    height: f32,
    width: f32,
    actions: &mut Vec<ResearchUiAction>,
) {
    ui.vertical(|ui| {
        ui.set_width(width);
        ui.heading("Module details");
        ui.separator();
        egui::ScrollArea::vertical()
            .id_salt("research-module-detail")
            .max_height(height - 34.0)
            .show(ui, |ui| {
                let Some(detail) = &view.detail else {
                    ui.label("Select a research module from any tier.");
                    if let Some(operation) = &view.latest_operation {
                        ui.separator();
                        ui.small(operation);
                    }
                    return;
                };

                ui.heading(&detail.name);
                ui.horizontal_wrapped(|ui| {
                    ui.strong(detail.state.label());
                    ui.separator();
                    ui.label(format!("{} cr", detail.price));
                    ui.separator();
                    ui.label(&detail.duration);
                });
                ui.label(&detail.summary);
                ui.add_space(6.0);
                ui.strong("Requirements");
                if detail.requirements.is_empty() {
                    ui.label("None");
                } else {
                    for requirement in &detail.requirements {
                        ui.label(format!("• {requirement}"));
                    }
                }
                ui.add_space(6.0);
                ui.strong("Rewards");
                if detail.rewards.is_empty() {
                    ui.label("None");
                } else {
                    for reward in &detail.rewards {
                        ui.label(format!("• {reward}"));
                    }
                }
                if let Some(progress) = detail.progress {
                    ui.add_space(8.0);
                    ui.add(
                        egui::ProgressBar::new(progress).text(
                            detail
                                .remaining
                                .clone()
                                .unwrap_or_else(|| "Researching".to_string()),
                        ),
                    );
                }
                ui.add_space(8.0);
                if ui
                    .add_enabled(
                        detail.start_enabled,
                        egui::Button::new(&detail.action_label),
                    )
                    .clicked()
                {
                    actions.push(ResearchUiAction::Start(detail.id.clone()));
                }
                if let Some(operation) = &view.latest_operation {
                    ui.separator();
                    ui.small(operation);
                }
            });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn research_actions_preserve_selection_and_start_identity() {
        assert_ne!(
            ResearchUiAction::Select("core:test".to_string()),
            ResearchUiAction::Start("core:test".to_string())
        );
    }

    #[test]
    fn research_tiers_are_sorted_deduplicated_and_data_driven() {
        let node = |tier| ResearchNode {
            id: format!("test:{tier}"),
            name: format!("Tier {tier}"),
            tier,
            column: tier as i32,
            row: 0,
            state: ResearchState::Locked,
            progress: None,
        };
        assert_eq!(
            research_tiers(&[node(12), node(2), node(12), node(5)]),
            vec![2, 5, 12]
        );
    }

    #[test]
    fn research_panes_never_request_more_than_the_available_width() {
        for available_width in [0.0, 240.0, 320.0, 680.0, 1080.0] {
            let (tier_width, divider_gap, detail_width) = research_pane_widths(available_width);
            assert!(tier_width >= 0.0);
            assert!(divider_gap >= 0.0);
            assert!(detail_width >= 0.0);
            assert!(
                (tier_width + divider_gap + detail_width - available_width).abs() < f32::EPSILON
            );
        }
    }
}
