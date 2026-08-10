use egui_macroquad::egui::{self, Color32, Id, RichText, Stroke, Vec2};
use std::collections::{BTreeSet, HashMap};

const GRAPH_PADDING: f32 = 14.0;
const GRAPH_HEADER_HEIGHT: f32 = 48.0;
const NODE_WIDTH: f32 = 218.0;
const NODE_HEIGHT: f32 = 76.0;
const COLUMN_GAP: f32 = 86.0;
const ROW_GAP: f32 = 28.0;

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
            Self::Available => "Need credits",
            Self::Locked => "Locked",
        }
    }

    fn marker(self) -> &'static str {
        match self {
            Self::Completed => "DONE",
            Self::Researching => "ACTIVE",
            Self::Affordable => "READY",
            Self::Available => "FUNDS",
            Self::Locked => "LOCKED",
        }
    }

    fn tooltip(self) -> &'static str {
        match self {
            Self::Completed => "Completed. This module's rewards are active.",
            Self::Researching => "Active research. Progress continues during play.",
            Self::Affordable => "Prerequisites and credit cost are satisfied. Ready to start.",
            Self::Available => "Prerequisites are complete, but more credits are required.",
            Self::Locked => {
                "Locked until its prerequisites are complete and no other research is active."
            }
        }
    }

    fn colors(self) -> (Color32, Color32, Color32) {
        match self {
            Self::Completed => (
                Color32::from_rgb(25, 58, 39),
                Color32::from_rgb(116, 205, 139),
                Color32::from_rgb(199, 239, 207),
            ),
            Self::Researching => (
                Color32::from_rgb(20, 52, 63),
                Color32::from_rgb(90, 202, 224),
                Color32::from_rgb(202, 241, 246),
            ),
            Self::Affordable => (
                Color32::from_rgb(53, 53, 27),
                Color32::from_rgb(222, 193, 93),
                Color32::from_rgb(244, 228, 169),
            ),
            Self::Available => (
                Color32::from_rgb(42, 43, 46),
                Color32::from_rgb(154, 164, 168),
                Color32::from_rgb(212, 218, 220),
            ),
            Self::Locked => (
                Color32::from_rgb(56, 29, 33),
                Color32::from_rgb(184, 91, 99),
                Color32::from_rgb(232, 181, 185),
            ),
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
    pub dependents: Vec<String>,
    pub rewards: Vec<String>,
    pub progress: Option<f32>,
    pub remaining: Option<String>,
    pub start_enabled: bool,
    pub action_label: String,
}

#[derive(Clone, Debug, PartialEq)]
struct ResearchNodeLayout {
    id: String,
    rect: egui::Rect,
}

#[derive(Clone, Debug, PartialEq)]
struct ResearchColumnLayout {
    authored_column: i32,
    tiers: Vec<u32>,
    x: f32,
    node_count: usize,
}

#[derive(Clone, Debug, PartialEq)]
struct ResearchGraphLayout {
    size: Vec2,
    nodes: Vec<ResearchNodeLayout>,
    columns: Vec<ResearchColumnLayout>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ResearchEdgePath {
    points: [egui::Pos2; 4],
    arrow: [egui::Pos2; 3],
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
    ui.small("Authored columns and rows come from loaded content packs. Scroll the tree in either direction; select a module to highlight its dependencies.");
    ui.separator();

    let pane_height = ui.available_height().max(240.0);
    let available_width = ui.available_width();
    if research_uses_stacked_layout(available_width) {
        let divider_gap = 10.0;
        let graph_height = (pane_height * 0.52).clamp(110.0, (pane_height - 120.0).max(110.0));
        let detail_height = (pane_height - graph_height - divider_gap).max(110.0);
        let (graph_rect, _) = ui.allocate_exact_size(
            Vec2::new(available_width, graph_height),
            egui::Sense::hover(),
        );
        let mut graph_ui = ui.new_child(egui::UiBuilder::new().max_rect(graph_rect));
        graph_ui.set_clip_rect(graph_rect);
        draw_graph(&mut graph_ui, view, actions);
        ui.add_space(divider_gap);
        let (detail_rect, _) = ui.allocate_exact_size(
            Vec2::new(available_width, detail_height),
            egui::Sense::hover(),
        );
        let mut detail_ui = ui.new_child(egui::UiBuilder::new().max_rect(detail_rect));
        detail_ui.set_clip_rect(detail_rect);
        draw_detail(
            &mut detail_ui,
            view,
            detail_height,
            available_width,
            actions,
        );
        return;
    }

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

    let mut graph_ui = ui.new_child(egui::UiBuilder::new().max_rect(tier_rect));
    graph_ui.set_clip_rect(tier_rect);
    draw_graph(&mut graph_ui, view, actions);

    ui.painter().vline(
        tier_rect.right() + divider_gap * 0.5,
        body_rect.y_range(),
        ui.visuals().widgets.noninteractive.bg_stroke,
    );
    let mut detail_ui = ui.new_child(egui::UiBuilder::new().max_rect(detail_rect));
    detail_ui.set_clip_rect(detail_rect);
    draw_detail(&mut detail_ui, view, pane_height, detail_width, actions);
}

fn research_uses_stacked_layout(available_width: f32) -> bool {
    available_width < 620.0
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

fn research_graph_layout(nodes: &[ResearchNode]) -> ResearchGraphLayout {
    if nodes.is_empty() {
        return ResearchGraphLayout {
            size: Vec2::new(1.0, 1.0),
            nodes: Vec::new(),
            columns: Vec::new(),
        };
    }

    let authored_columns = nodes
        .iter()
        .map(|node| node.column)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let authored_rows = nodes
        .iter()
        .map(|node| node.row)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let row_rank = authored_rows
        .iter()
        .enumerate()
        .map(|(index, row)| (*row, index))
        .collect::<HashMap<_, _>>();

    let mut layouts = Vec::with_capacity(nodes.len());
    let mut columns = Vec::with_capacity(authored_columns.len());
    let mut maximum_lane = 0;
    for (column_index, authored_column) in authored_columns.iter().enumerate() {
        let x = GRAPH_PADDING + column_index as f32 * (NODE_WIDTH + COLUMN_GAP);
        let mut column_nodes = nodes
            .iter()
            .filter(|node| node.column == *authored_column)
            .collect::<Vec<_>>();
        column_nodes.sort_by(|left, right| {
            left.row
                .cmp(&right.row)
                .then(left.tier.cmp(&right.tier))
                .then(left.name.cmp(&right.name))
                .then(left.id.cmp(&right.id))
        });
        let tiers = column_nodes
            .iter()
            .map(|node| node.tier)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let mut next_free_lane = 0;
        for node in &column_nodes {
            let desired_lane = row_rank.get(&node.row).copied().unwrap_or_default();
            let lane = desired_lane.max(next_free_lane);
            next_free_lane = lane + 1;
            maximum_lane = maximum_lane.max(lane);
            let y = GRAPH_PADDING + GRAPH_HEADER_HEIGHT + lane as f32 * (NODE_HEIGHT + ROW_GAP);
            layouts.push(ResearchNodeLayout {
                id: node.id.clone(),
                rect: egui::Rect::from_min_size(
                    egui::pos2(x, y),
                    Vec2::new(NODE_WIDTH, NODE_HEIGHT),
                ),
            });
        }
        columns.push(ResearchColumnLayout {
            authored_column: *authored_column,
            tiers,
            x,
            node_count: column_nodes.len(),
        });
    }

    layouts.sort_by(|left, right| {
        left.rect
            .min
            .x
            .total_cmp(&right.rect.min.x)
            .then(left.rect.min.y.total_cmp(&right.rect.min.y))
            .then(left.id.cmp(&right.id))
    });
    let width = GRAPH_PADDING * 2.0
        + authored_columns.len() as f32 * NODE_WIDTH
        + authored_columns.len().saturating_sub(1) as f32 * COLUMN_GAP
        + COLUMN_GAP * 0.5;
    let height = GRAPH_PADDING * 2.0
        + GRAPH_HEADER_HEIGHT
        + (maximum_lane + 1) as f32 * NODE_HEIGHT
        + maximum_lane as f32 * ROW_GAP;
    ResearchGraphLayout {
        size: Vec2::new(width, height),
        nodes: layouts,
        columns,
    }
}

fn draw_graph(ui: &mut egui::Ui, view: &ResearchView, actions: &mut Vec<ResearchUiAction>) {
    let layout = research_graph_layout(&view.nodes);
    egui::ScrollArea::both()
        .id_salt("research-graph-browser")
        .auto_shrink([false, false])
        .show_viewport(ui, |ui, _viewport| {
            ui.set_min_size(layout.size);
            let origin = ui.min_rect().min.to_vec2();
            draw_graph_columns(ui, &layout, origin);
            draw_graph_edges(ui, view, &layout, origin);
            draw_graph_nodes(ui, view, &layout, origin, actions);
        });
}

fn draw_graph_columns(ui: &egui::Ui, layout: &ResearchGraphLayout, origin: Vec2) {
    for column in &layout.columns {
        let tier_label = if column.tiers.len() == 1 {
            format!("Tier {}", column.tiers[0])
        } else {
            format!(
                "Column {} · Tiers {}",
                column.authored_column,
                column
                    .tiers
                    .iter()
                    .map(u32::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        let position = egui::pos2(
            origin.x + column.x,
            origin.y + GRAPH_PADDING + GRAPH_HEADER_HEIGHT * 0.35,
        );
        ui.painter().text(
            position,
            egui::Align2::LEFT_CENTER,
            format!("{tier_label}  ·  {} module(s)", column.node_count),
            egui::TextStyle::Heading.resolve(ui.style()),
            ui.visuals().strong_text_color(),
        );
    }
}

fn draw_graph_edges(
    ui: &egui::Ui,
    view: &ResearchView,
    layout: &ResearchGraphLayout,
    origin: Vec2,
) {
    let rects = layout
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node.rect.translate(origin)))
        .collect::<HashMap<_, _>>();
    let states = view
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node.state))
        .collect::<HashMap<_, _>>();
    let mut edges = view.edges.iter().collect::<Vec<_>>();
    edges.sort_by(|left, right| left.from.cmp(&right.from).then(left.to.cmp(&right.to)));

    for edge in edges {
        let (Some(from), Some(to)) = (rects.get(edge.from.as_str()), rects.get(edge.to.as_str()))
        else {
            continue;
        };
        let related = view
            .selected
            .as_deref()
            .is_some_and(|selected| selected == edge.from || selected == edge.to);
        let source_completed = states
            .get(edge.from.as_str())
            .is_some_and(|state| *state == ResearchState::Completed);
        let color = if related {
            Color32::from_rgb(104, 211, 229)
        } else if source_completed {
            Color32::from_rgba_premultiplied(105, 190, 128, 180)
        } else {
            Color32::from_rgba_premultiplied(112, 139, 147, 125)
        };
        let stroke = Stroke::new(if related { 3.0_f32 } else { 1.5_f32 }, color);
        let path = research_edge_path(*from, *to);
        for segment in path.points.windows(2) {
            ui.painter().line_segment([segment[0], segment[1]], stroke);
        }
        ui.painter().add(egui::Shape::convex_polygon(
            path.arrow.to_vec(),
            color,
            Stroke::NONE,
        ));
    }
}

fn research_edge_path(from: egui::Rect, to: egui::Rect) -> ResearchEdgePath {
    let horizontal_direction = (to.center().x - from.center().x).total_cmp(&0.0);
    let (start, end, mid_x, direction) = match horizontal_direction {
        std::cmp::Ordering::Greater => {
            let start = from.right_center();
            let end = to.left_center();
            (start, end, (start.x + end.x) * 0.5, 1.0)
        }
        std::cmp::Ordering::Less => {
            let start = from.left_center();
            let end = to.right_center();
            (start, end, (start.x + end.x) * 0.5, -1.0)
        }
        std::cmp::Ordering::Equal => {
            let start = from.right_center();
            let end = to.right_center();
            (
                start,
                end,
                from.right().max(to.right()) + COLUMN_GAP * 0.5,
                -1.0,
            )
        }
    };
    ResearchEdgePath {
        points: [
            start,
            egui::pos2(mid_x, start.y),
            egui::pos2(mid_x, end.y),
            end,
        ],
        arrow: [
            end,
            egui::pos2(end.x - direction * 8.0, end.y - 5.0),
            egui::pos2(end.x - direction * 8.0, end.y + 5.0),
        ],
    }
}

fn draw_graph_nodes(
    ui: &mut egui::Ui,
    view: &ResearchView,
    layout: &ResearchGraphLayout,
    origin: Vec2,
    actions: &mut Vec<ResearchUiAction>,
) {
    let nodes = view
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect::<HashMap<_, _>>();
    for node_layout in &layout.nodes {
        let Some(node) = nodes.get(node_layout.id.as_str()).copied() else {
            continue;
        };
        let rect = node_layout.rect.translate(origin);
        let selected = view.selected.as_deref() == Some(node.id.as_str());
        let (fill, state_border, text_color) = node.state.colors();
        let stroke = if selected {
            Stroke::new(3.0_f32, Color32::from_rgb(225, 245, 248))
        } else {
            Stroke::new(1.5_f32, state_border)
        };
        let label = RichText::new(format!(
            "{}  {}\n{}",
            node.state.marker(),
            node.name,
            node.state.label()
        ))
        .color(text_color);
        let response = ui.put(
            rect,
            egui::Button::new(label)
                .fill(fill)
                .stroke(stroke)
                .corner_radius(5.0)
                .wrap(),
        );
        if response.clicked() {
            actions.push(ResearchUiAction::Select(node.id.clone()));
        }
        response.on_hover_ui(|ui| {
            ui.strong(&node.name);
            ui.strong(format!("{} · {}", node.state.marker(), node.state.label()));
            ui.label(node.state.tooltip());
            ui.small(format!(
                "Tier {} · authored position {},{}",
                node.tier, node.column, node.row
            ));
        });
        if let Some(progress) = node.progress {
            let track = egui::Rect::from_min_max(
                egui::pos2(rect.left() + 7.0, rect.bottom() - 9.0),
                egui::pos2(rect.right() - 7.0, rect.bottom() - 5.0),
            );
            ui.painter().rect_filled(track, 2.0, Color32::from_gray(40));
            let filled = egui::Rect::from_min_max(
                track.min,
                egui::pos2(
                    track.left() + track.width() * progress.clamp(0.0, 1.0),
                    track.bottom(),
                ),
            );
            ui.painter()
                .rect_filled(filled, 2.0, Color32::from_rgb(104, 211, 229));
        }
    }
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
                ui.strong("Leads to");
                if detail.dependents.is_empty() {
                    ui.label("No dependent modules");
                } else {
                    for dependent in &detail.dependents {
                        ui.label(format!("→ {dependent}"));
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

    fn node(id: &str, tier: u32, column: i32, row: i32) -> ResearchNode {
        ResearchNode {
            id: id.to_string(),
            name: id.replace('_', " "),
            tier,
            column,
            row,
            state: ResearchState::Locked,
            progress: None,
        }
    }

    #[test]
    fn research_layout_is_deterministic_for_shuffled_content() {
        let ordered = vec![
            node("root", 0, 0, 0),
            node("mining", 1, 1, 0),
            node("fabrication", 1, 1, 2),
            node("remote", 5, 5, 1),
        ];
        let mut shuffled = ordered.clone();
        shuffled.reverse();

        assert_eq!(
            research_graph_layout(&ordered),
            research_graph_layout(&shuffled)
        );
        assert_eq!(
            research_graph_layout(&ordered)
                .columns
                .iter()
                .map(|column| column.authored_column)
                .collect::<Vec<_>>(),
            vec![0, 1, 5]
        );
    }

    #[test]
    fn duplicate_authored_positions_receive_non_overlapping_lanes() {
        let layout = research_graph_layout(&[
            node("pack_a", 1, 1, 0),
            node("pack_b", 1, 1, 0),
            node("next_row", 1, 1, 1),
        ]);

        assert!(layout.nodes.iter().enumerate().all(|(index, node)| {
            layout
                .nodes
                .iter()
                .skip(index + 1)
                .all(|other| !node.rect.intersects(other.rect))
        }));
        assert!(layout.size.x >= NODE_WIDTH + GRAPH_PADDING * 2.0);
        assert!(layout.size.y >= NODE_HEIGHT + GRAPH_HEADER_HEIGHT);
        let first = layout.nodes[0].rect;
        let second = layout.nodes[1].rect;
        let path = research_edge_path(first, second);
        assert!(path.points.iter().all(|point| point.x <= layout.size.x));
    }

    #[test]
    fn dependency_paths_end_at_the_dependent_card_with_direction_marker() {
        let from = egui::Rect::from_min_size(egui::pos2(10.0, 20.0), Vec2::new(100.0, 60.0));
        let to = egui::Rect::from_min_size(egui::pos2(220.0, 100.0), Vec2::new(100.0, 60.0));
        let path = research_edge_path(from, to);

        assert_eq!(path.points[0], from.right_center());
        assert_eq!(path.points[3], to.left_center());
        assert_eq!(path.arrow[0], to.left_center());
        assert!(path.arrow[1].x < path.arrow[0].x);
        assert_eq!(path.points[1].x, path.points[2].x);

        let below = to.translate(Vec2::new(0.0, 100.0));
        let same_column = research_edge_path(to, below);
        assert!(same_column.points[1].x > to.right());
        assert_eq!(same_column.points[3], below.right_center());
        assert!(same_column.arrow[1].x > same_column.arrow[0].x);
    }

    #[test]
    fn every_research_state_has_distinct_text_and_visual_treatment() {
        let states = [
            ResearchState::Completed,
            ResearchState::Researching,
            ResearchState::Affordable,
            ResearchState::Available,
            ResearchState::Locked,
        ];
        let labels = states
            .iter()
            .map(|state| state.label())
            .collect::<BTreeSet<_>>();
        let markers = states
            .iter()
            .map(|state| state.marker())
            .collect::<BTreeSet<_>>();
        let colors = states
            .iter()
            .map(|state| state.colors())
            .collect::<Vec<_>>();

        assert_eq!(labels.len(), states.len());
        assert_eq!(markers.len(), states.len());
        assert!(colors
            .iter()
            .enumerate()
            .all(|(index, color)| { colors.iter().skip(index + 1).all(|other| color != other) }));
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

    #[test]
    fn narrow_research_windows_stack_graph_above_details() {
        assert!(research_uses_stacked_layout(320.0));
        assert!(research_uses_stacked_layout(619.0));
        assert!(!research_uses_stacked_layout(620.0));
        assert!(!research_uses_stacked_layout(1080.0));
    }
}
