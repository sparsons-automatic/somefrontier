use egui_macroquad::egui::{self, Id, Vec2};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackRow {
    pub name: String,
    pub detail: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackDetail {
    pub name: String,
    pub id: String,
    pub version: String,
    pub description: String,
    pub path: String,
    pub dependencies: Vec<String>,
    pub optional_dependencies: Vec<String>,
    pub options: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ObjectKind {
    Item,
    Recipe,
    Ship,
    NpcShip,
    Shield,
    Weapon,
    PowerModule,
    Research,
    Faction,
    Universe,
    GalaxyGroup,
    GalaxyCluster,
    Galaxy,
    Region,
    System,
    Star,
    Planet,
    Station,
    Vendor,
    Upgrade,
}

impl ObjectKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Item => "Item",
            Self::Recipe => "Recipe",
            Self::Ship => "Ship",
            Self::NpcShip => "NPC Ship",
            Self::Shield => "Shield",
            Self::Weapon => "Weapon",
            Self::PowerModule => "Power Module",
            Self::Research => "Research",
            Self::Faction => "Faction",
            Self::Universe => "Universe",
            Self::GalaxyGroup => "Galaxy Group",
            Self::GalaxyCluster => "Galaxy Cluster",
            Self::Galaxy => "Galaxy",
            Self::Region => "Region",
            Self::System => "System",
            Self::Star => "Star",
            Self::Planet => "Planet",
            Self::Station => "Station",
            Self::Vendor => "Vendor",
            Self::Upgrade => "Upgrade",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ObjectKey {
    pub kind: ObjectKind,
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObjectRow {
    pub key: ObjectKey,
    pub name: String,
    pub detail: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObjectGroup {
    pub title: String,
    pub rows: Vec<ObjectRow>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DetailField {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DetailReference {
    pub label: String,
    pub target: ObjectKey,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObjectDetail {
    pub key: ObjectKey,
    pub name: String,
    pub artwork: Option<String>,
    pub summary: Option<String>,
    pub fields: Vec<DetailField>,
    pub references: Vec<DetailReference>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContentBrowserView {
    pub pinned: bool,
    pub summary: String,
    pub transition_summary: Option<String>,
    pub filter_label: String,
    pub packs: Vec<PackRow>,
    pub selected_pack: Option<usize>,
    pub selected_detail: Option<PackDetail>,
    pub object_groups: Vec<ObjectGroup>,
    pub selected_object: Option<ObjectKey>,
    pub object_detail: Option<ObjectDetail>,
    pub can_go_back: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContentUiAction {
    TogglePin,
    SelectPack(Option<usize>),
    SelectObject(ObjectKey),
    BackObject,
}

pub fn draw(
    ctx: &egui::Context,
    screen: [f32; 2],
    view: &ContentBrowserView,
) -> Vec<ContentUiAction> {
    ctx.set_style(egui::Style::default());
    let mut actions = Vec::new();
    let height = 680.0_f32.min((screen[1] - 96.0).max(320.0));

    egui::Window::new("Content Browser")
        .id(Id::new("content-browser-window"))
        .default_pos(egui::pos2(32.0, 40.0))
        .default_size(Vec2::new(1180.0_f32.min(screen[0] - 64.0), height))
        .min_width(720.0)
        .resizable(true)
        .collapsible(true)
        .show(ctx, |ui| draw_contents(ui, view, &mut actions));

    actions
}

fn draw_contents(ui: &mut egui::Ui, view: &ContentBrowserView, actions: &mut Vec<ContentUiAction>) {
    if crate::egui_pin::toggle(ui, view.pinned) {
        actions.push(ContentUiAction::TogglePin);
    }
    let pane_height = ui.available_height().max(220.0);
    ui.horizontal_top(|ui| {
        pack_pane(ui, view, pane_height, actions);
        ui.separator();
        object_pane(ui, view, pane_height, actions);
        ui.separator();
        detail_pane(ui, view, pane_height, actions);
    });
}

fn pack_pane(
    ui: &mut egui::Ui,
    view: &ContentBrowserView,
    height: f32,
    actions: &mut Vec<ContentUiAction>,
) {
    ui.vertical(|ui| {
        ui.set_width(190.0);
        ui.heading("Content packs");
        ui.small(format!("{} loaded", view.packs.len()));
        ui.separator();
        egui::ScrollArea::vertical()
            .id_salt("content-pack-list")
            .max_height(height - 54.0)
            .show(ui, |ui| {
                if ui
                    .add_sized(
                        [ui.available_width(), 34.0],
                        egui::Button::new("All packs").selected(view.selected_pack.is_none()),
                    )
                    .clicked()
                {
                    actions.push(ContentUiAction::SelectPack(None));
                }
                for (index, pack) in view.packs.iter().enumerate() {
                    if ui
                        .add_sized(
                            [ui.available_width(), 48.0],
                            egui::Button::new(format!("{}\n{}", pack.name, pack.detail))
                                .selected(view.selected_pack == Some(index)),
                        )
                        .clicked()
                    {
                        actions.push(ContentUiAction::SelectPack(Some(index)));
                    }
                }
            });
    });
}

fn object_pane(
    ui: &mut egui::Ui,
    view: &ContentBrowserView,
    height: f32,
    actions: &mut Vec<ContentUiAction>,
) {
    ui.vertical(|ui| {
        ui.set_width(260.0);
        ui.heading("Objects");
        ui.small(&view.filter_label);
        ui.separator();
        egui::ScrollArea::vertical()
            .id_salt("content-object-list")
            .max_height(height - 54.0)
            .show(ui, |ui| {
                if view.object_groups.is_empty() {
                    ui.label("No matching content");
                }
                for group in &view.object_groups {
                    egui::CollapsingHeader::new(format!("{} ({})", group.title, group.rows.len()))
                        .default_open(true)
                        .show(ui, |ui| {
                            for row in &group.rows {
                                let selected = view.selected_object.as_ref() == Some(&row.key);
                                let response = ui.add_sized(
                                    [ui.available_width(), 42.0],
                                    egui::Button::new(format!("{}\n{}", row.name, row.detail))
                                        .selected(selected),
                                );
                                if response.clicked() {
                                    actions.push(ContentUiAction::SelectObject(row.key.clone()));
                                }
                            }
                        });
                }
            });
    });
}

fn detail_pane(
    ui: &mut egui::Ui,
    view: &ContentBrowserView,
    height: f32,
    actions: &mut Vec<ContentUiAction>,
) {
    ui.vertical(|ui| {
        ui.set_min_width(300.0);
        ui.horizontal(|ui| {
            if ui
                .add_enabled(view.can_go_back, egui::Button::new("Back"))
                .clicked()
            {
                actions.push(ContentUiAction::BackObject);
            }
            ui.heading("Details");
        });
        ui.separator();
        egui::ScrollArea::vertical()
            .id_salt("content-pack-detail")
            .max_height(height - 38.0)
            .show(ui, |ui| {
                if let Some(detail) = &view.object_detail {
                    crate::egui_art::preview(
                        ui,
                        detail.artwork.as_deref(),
                        Vec2::new(260.0, 170.0),
                    );
                    ui.heading(&detail.name);
                    ui.small(format!("{} · {}", detail.key.kind.label(), detail.key.id));
                    if let Some(summary) = &detail.summary {
                        ui.label(summary);
                    }
                    ui.add_space(4.0);
                    egui::Grid::new("content-object-facts")
                        .num_columns(2)
                        .striped(true)
                        .show(ui, |ui| {
                            for field in &detail.fields {
                                fact_row(ui, &field.label, &field.value);
                            }
                        });
                    if !detail.references.is_empty() {
                        ui.add_space(8.0);
                        ui.strong("References");
                        for reference in &detail.references {
                            if ui
                                .link(format!("{} → {}", reference.label, reference.target.id))
                                .clicked()
                            {
                                actions
                                    .push(ContentUiAction::SelectObject(reference.target.clone()));
                            }
                        }
                    }
                } else {
                    if let Some(detail) = &view.selected_detail {
                        ui.heading(&detail.name);
                        ui.label(&detail.description);
                        egui::Grid::new("content-pack-facts")
                            .num_columns(2)
                            .striped(true)
                            .show(ui, |ui| {
                                fact_row(ui, "ID", &detail.id);
                                fact_row(ui, "Version", &detail.version);
                                fact_row(ui, "Path", &detail.path);
                                fact_row(ui, "Dependencies", &list_or_none(&detail.dependencies));
                                fact_row(
                                    ui,
                                    "Optional dependencies",
                                    &list_or_none(&detail.optional_dependencies),
                                );
                            });
                        if !detail.options.is_empty() {
                            ui.strong("Configuration options");
                            for option in &detail.options {
                                ui.label(option);
                            }
                        }
                    } else {
                        ui.label(&view.summary);
                        if let Some(transition) = &view.transition_summary {
                            ui.small(transition);
                        }
                    }
                    ui.separator();
                    ui.label("Select an object to inspect its authored data and references.");
                }
            });
    });
}

fn fact_row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.label(label);
    ui.label(value);
    ui.end_row();
}

fn list_or_none(values: &[String]) -> String {
    if values.is_empty() {
        "None".to_string()
    } else {
        values.join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_filter_actions_distinguish_all_and_specific_packs() {
        assert_ne!(
            ContentUiAction::SelectPack(None),
            ContentUiAction::SelectPack(Some(0))
        );
    }

    #[test]
    fn empty_metadata_lists_have_an_explicit_value() {
        assert_eq!(list_or_none(&[]), "None");
        assert_eq!(list_or_none(&["core".to_string()]), "core");
    }
}
