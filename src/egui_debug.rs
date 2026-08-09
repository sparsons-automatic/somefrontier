use egui_macroquad::egui::{self, Id, Key, Vec2};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugConsoleView {
    pub input: String,
    pub history: Vec<String>,
    pub help: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DebugConsoleUiAction {
    SetInput(String),
    Execute(String),
    Close,
}

pub fn draw(
    ctx: &egui::Context,
    screen: [f32; 2],
    view: &DebugConsoleView,
) -> Vec<DebugConsoleUiAction> {
    ctx.set_style(egui::Style::default());
    let mut actions = Vec::new();
    let mut open = true;
    let width = 820.0_f32.min((screen[0] - 48.0).max(420.0));
    let height = 440.0_f32.min((screen[1] - 72.0).max(260.0));

    egui::Window::new("Debug Console")
        .id(Id::new("debug-console-window"))
        .default_pos(egui::pos2(28.0, (screen[1] - height - 28.0).max(24.0)))
        .default_size(Vec2::new(width, height))
        .min_size(Vec2::new(420.0_f32.min(width), 260.0_f32.min(height)))
        .resizable(true)
        .collapsible(true)
        .open(&mut open)
        .show(ctx, |ui| draw_contents(ui, view, &mut actions));

    if !open {
        actions.push(DebugConsoleUiAction::Close);
    }
    actions
}

fn draw_contents(
    ui: &mut egui::Ui,
    view: &DebugConsoleView,
    actions: &mut Vec<DebugConsoleUiAction>,
) {
    ui.small(&view.help);
    ui.separator();

    let input_height = 30.0;
    let history_height = (ui.available_height() - input_height - 18.0).max(100.0);
    egui::ScrollArea::vertical()
        .id_salt("debug-console-history")
        .stick_to_bottom(true)
        .max_height(history_height)
        .show(ui, |ui| {
            ui.set_min_height(history_height);
            if view.history.is_empty() {
                ui.weak("Command output will appear here.");
            } else {
                for line in view.history.iter().rev() {
                    ui.monospace(line);
                }
            }
        });

    ui.separator();
    ui.horizontal(|ui| {
        let mut input = view.input.clone();
        let response = ui.add_sized(
            [ui.available_width() - 76.0, input_height],
            egui::TextEdit::singleline(&mut input)
                .font(egui::TextStyle::Monospace)
                .hint_text("Enter command"),
        );
        if response.changed() {
            actions.push(DebugConsoleUiAction::SetInput(input.clone()));
        }
        let execute = ui.button("Run").clicked()
            || (response.has_focus() && ui.input(|input| input.key_pressed(Key::Enter)));
        if execute {
            let command = input.trim();
            if !command.is_empty() {
                actions.push(DebugConsoleUiAction::Execute(command.to_string()));
            }
            response.request_focus();
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_and_edit_actions_remain_distinct() {
        assert_ne!(
            DebugConsoleUiAction::SetInput("help".to_string()),
            DebugConsoleUiAction::Execute("help".to_string())
        );
    }
}
