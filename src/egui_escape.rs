use egui_macroquad::egui::{self, Id, Vec2};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EscapeView {
    pub save_dirty: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EscapeUiAction {
    Resume,
    SaveNow,
    ResetMenus,
    SaveToTitle,
    QuitDesktop,
}

pub fn draw(ctx: &egui::Context, screen: [f32; 2], view: EscapeView) -> Vec<EscapeUiAction> {
    ctx.set_style(egui::Style::default());
    let mut actions = Vec::new();
    let size = Vec2::new(420.0_f32.min((screen[0] - 32.0).max(280.0)), 360.0);

    egui::Window::new("Game Paused")
        .id(Id::new("escape-menu-window"))
        .default_pos(egui::pos2(
            ((screen[0] - size.x) * 0.5).max(8.0),
            ((screen[1] - size.y) * 0.5).max(8.0),
        ))
        .fixed_size(size)
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| draw_contents(ui, view, &mut actions));

    actions
}

fn draw_contents(ui: &mut egui::Ui, view: EscapeView, actions: &mut Vec<EscapeUiAction>) {
    ui.heading("Some Frontier");
    if view.save_dirty {
        ui.colored_label(
            ui.visuals().warn_fg_color,
            "Unsaved changes are queued for autosave.",
        );
    } else {
        ui.label("Current game state is saved.");
    }
    ui.separator();

    action_button(
        ui,
        "Resume",
        "Esc",
        "Close the pause menu and return to gameplay.",
        EscapeUiAction::Resume,
        actions,
    );
    action_button(
        ui,
        "Save Now",
        "S",
        "Write the current run to disk without leaving gameplay.",
        EscapeUiAction::SaveNow,
        actions,
    );
    action_button(
        ui,
        "Reset Menus",
        "R",
        "Restore every egui menu to its default geometry and clear all pins.",
        EscapeUiAction::ResetMenus,
        actions,
    );
    action_button(
        ui,
        "Save to Title Menu",
        "T",
        "Save the current run, leave gameplay, and return to the title menu.",
        EscapeUiAction::SaveToTitle,
        actions,
    );

    ui.add_space(8.0);
    ui.separator();
    action_button(
        ui,
        "Save and Quit Desktop",
        "Q",
        "Save the current run, then close Some Frontier.",
        EscapeUiAction::QuitDesktop,
        actions,
    );
}

fn action_button(
    ui: &mut egui::Ui,
    label: &str,
    shortcut: &str,
    help: &str,
    action: EscapeUiAction,
    actions: &mut Vec<EscapeUiAction>,
) {
    let button_label = format!("{label}    [{shortcut}]");
    if ui
        .add_sized(
            [ui.available_width(), 34.0],
            egui::Button::new(button_label),
        )
        .on_hover_text(help)
        .clicked()
    {
        actions.push(action);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_actions_remain_distinct() {
        assert_ne!(EscapeUiAction::Resume, EscapeUiAction::SaveNow);
        assert_ne!(EscapeUiAction::SaveToTitle, EscapeUiAction::QuitDesktop);
    }
}
