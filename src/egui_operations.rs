use egui_macroquad::egui::{self, Color32, Id, RichText, Vec2};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OperationCategory {
    Mining,
    Production,
    Travel,
    Trade,
    Combat,
    Research,
    System,
    General,
}

impl OperationCategory {
    pub fn label(self) -> &'static str {
        match self {
            Self::Mining => "Mining",
            Self::Production => "Production",
            Self::Travel => "Travel",
            Self::Trade => "Trade",
            Self::Combat => "Combat",
            Self::Research => "Research",
            Self::System => "System",
            Self::General => "General",
        }
    }

    fn marker(self) -> &'static str {
        match self {
            Self::Mining => "[MINE]",
            Self::Production => "[MAKE]",
            Self::Travel => "[NAV]",
            Self::Trade => "[TRADE]",
            Self::Combat => "[COMBAT]",
            Self::Research => "[R&D]",
            Self::System => "[SYS]",
            Self::General => "[INFO]",
        }
    }

    fn color(self) -> Color32 {
        match self {
            Self::Mining => Color32::from_rgb(213, 174, 91),
            Self::Production => Color32::from_rgb(130, 202, 216),
            Self::Travel => Color32::from_rgb(102, 189, 225),
            Self::Trade => Color32::from_rgb(125, 213, 153),
            Self::Combat => Color32::from_rgb(232, 112, 102),
            Self::Research => Color32::from_rgb(184, 139, 232),
            Self::System => Color32::from_rgb(232, 185, 100),
            Self::General => Color32::from_rgb(174, 194, 200),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum OperationsTab {
    #[default]
    All,
    Mining,
    Production,
    Travel,
    Trade,
    Combat,
    Research,
    System,
    General,
}

impl OperationsTab {
    pub const ALL: [Self; 9] = [
        Self::All,
        Self::Mining,
        Self::Production,
        Self::Travel,
        Self::Trade,
        Self::Combat,
        Self::Research,
        Self::System,
        Self::General,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Mining => "Mining",
            Self::Production => "Production",
            Self::Travel => "Travel",
            Self::Trade => "Trade",
            Self::Combat => "Combat",
            Self::Research => "Research",
            Self::System => "System",
            Self::General => "General",
        }
    }

    pub fn includes(self, category: OperationCategory) -> bool {
        self == Self::All || self.category() == Some(category)
    }

    fn category(self) -> Option<OperationCategory> {
        match self {
            Self::All => None,
            Self::Mining => Some(OperationCategory::Mining),
            Self::Production => Some(OperationCategory::Production),
            Self::Travel => Some(OperationCategory::Travel),
            Self::Trade => Some(OperationCategory::Trade),
            Self::Combat => Some(OperationCategory::Combat),
            Self::Research => Some(OperationCategory::Research),
            Self::System => Some(OperationCategory::System),
            Self::General => Some(OperationCategory::General),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OperationSeverity {
    Info,
    Success,
    Warning,
    Error,
}

impl OperationSeverity {
    pub const ALL: [Self; 4] = [Self::Info, Self::Success, Self::Warning, Self::Error];

    pub fn label(self) -> &'static str {
        match self {
            Self::Info => "Info",
            Self::Success => "Success",
            Self::Warning => "Warning",
            Self::Error => "Error",
        }
    }

    fn marker(self) -> &'static str {
        match self {
            Self::Info => "i",
            Self::Success => "+",
            Self::Warning => "!",
            Self::Error => "X",
        }
    }

    fn color(self) -> Color32 {
        match self {
            Self::Info => Color32::from_rgb(174, 194, 200),
            Self::Success => Color32::from_rgb(133, 213, 153),
            Self::Warning => Color32::from_rgb(235, 190, 99),
            Self::Error => Color32::from_rgb(240, 112, 105),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperationRow {
    pub timestamp: String,
    pub category: OperationCategory,
    pub source: String,
    pub severity: OperationSeverity,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperationsView {
    pub pinned: bool,
    pub rows: Vec<OperationRow>,
    pub selected_tab: OperationsTab,
    pub hidden_severities: Vec<OperationSeverity>,
    pub history_count: usize,
    pub history_limit: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperationsUiAction {
    TogglePin,
    SelectTab(OperationsTab),
    ToggleSeverity(OperationSeverity),
    ResetFilters,
    Clear,
}

pub fn draw(
    ctx: &egui::Context,
    screen: [f32; 2],
    view: &OperationsView,
) -> Vec<OperationsUiAction> {
    ctx.set_style(egui::Style::default());
    let mut actions = Vec::new();
    let width = 860.0_f32.min((screen[0] - 64.0).max(460.0));
    let height = 560.0_f32.min((screen[1] - 88.0).max(300.0));

    egui::Window::new("Operations Log")
        .id(Id::new("operations-log-window"))
        .default_pos(egui::pos2(72.0, 56.0))
        .default_size(Vec2::new(width, height))
        .min_size(Vec2::new(460.0_f32.min(width), 300.0_f32.min(height)))
        .resizable(true)
        .collapsible(true)
        .show(ctx, |ui| draw_contents(ui, view, &mut actions));

    actions
}

fn draw_contents(ui: &mut egui::Ui, view: &OperationsView, actions: &mut Vec<OperationsUiAction>) {
    if crate::egui_pin::toggle(ui, view.pinned) {
        actions.push(OperationsUiAction::TogglePin);
    }
    ui.horizontal(|ui| {
        ui.heading("Operations history");
        ui.separator();
        ui.label(format!(
            "{} / {} events",
            view.history_count, view.history_limit
        ));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("Clear history").clicked() {
                actions.push(OperationsUiAction::Clear);
            }
            if ui.button("Reset filters").clicked() {
                actions.push(OperationsUiAction::ResetFilters);
            }
        });
    });
    ui.small("Session-only history. Each tab keeps its own scroll position.");
    ui.separator();

    ui.horizontal_wrapped(|ui| {
        for tab in OperationsTab::ALL {
            if ui
                .selectable_label(view.selected_tab == tab, tab.label())
                .clicked()
            {
                actions.push(OperationsUiAction::SelectTab(tab));
            }
        }
    });
    ui.separator();
    ui.horizontal_wrapped(|ui| {
        ui.strong("Severity");
        for severity in OperationSeverity::ALL {
            let enabled = !view.hidden_severities.contains(&severity);
            if ui.selectable_label(enabled, severity.label()).clicked() {
                actions.push(OperationsUiAction::ToggleSeverity(severity));
            }
        }
    });
    ui.separator();

    let list_height = ui.available_height().max(120.0);
    egui::ScrollArea::vertical()
        .id_salt(("operations-log-history", view.selected_tab))
        .stick_to_bottom(true)
        .max_height(list_height)
        .show(ui, |ui| {
            ui.set_min_height(list_height);
            if view.rows.is_empty() {
                ui.weak(if view.history_count == 0 {
                    "No operations recorded in this play session."
                } else {
                    "No operations match this tab and severity filter."
                });
                return;
            }

            for row in &view.rows {
                ui.horizontal_top(|ui| {
                    ui.monospace(RichText::new(&row.timestamp).color(Color32::GRAY));
                    ui.label(
                        RichText::new(row.category.marker())
                            .color(row.category.color())
                            .strong(),
                    )
                    .on_hover_text(row.category.label());
                    ui.label(
                        RichText::new(row.severity.marker())
                            .color(row.severity.color())
                            .strong(),
                    )
                    .on_hover_text(row.severity.label());
                    ui.vertical(|ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.label(
                                RichText::new(&row.source)
                                    .color(row.category.color())
                                    .strong(),
                            );
                            ui.label(&row.message);
                        });
                    });
                });
                ui.separator();
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn category_and_severity_filters_cover_every_visible_state() {
        assert_eq!(OperationsTab::ALL.len(), 9);
        assert_eq!(OperationSeverity::ALL.len(), 4);
        assert!(OperationSeverity::ALL
            .iter()
            .all(|value| !value.label().is_empty()));
        assert!(OperationsTab::ALL
            .iter()
            .all(|value| !value.label().is_empty()));
        assert!(OperationsTab::ALL
            .iter()
            .filter_map(|tab| tab.category())
            .all(|category| OperationsTab::All.includes(category)));
    }

    #[test]
    fn clear_and_filter_actions_remain_distinct() {
        assert_ne!(
            OperationsUiAction::Clear,
            OperationsUiAction::ToggleSeverity(OperationSeverity::Warning)
        );
        assert_ne!(
            OperationsUiAction::Clear,
            OperationsUiAction::SelectTab(OperationsTab::Combat)
        );
        assert_ne!(OperationsUiAction::Clear, OperationsUiAction::ResetFilters);
    }
}
