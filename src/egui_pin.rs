use egui_macroquad::egui;

pub fn toggle(ui: &mut egui::Ui, pinned: bool) -> bool {
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.selectable_label(pinned, if pinned { "Pinned" } else { "Pin" })
                .on_hover_text("Keep this window open when its parent workspace closes")
                .clicked()
        })
        .inner
    })
    .inner
}
