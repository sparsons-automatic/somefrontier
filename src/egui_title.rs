use egui_macroquad::egui::{self, Align, Id, Layout, TextureId, Vec2};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TitleMainView {
    pub logo: Option<TextureId>,
    pub logo_aspect_ratio: f32,
    pub load_enabled: bool,
    pub fast_start: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TitleMainAction {
    NewGame,
    LoadGame,
    ContentPacks,
    Settings,
    Quit,
}

pub fn draw_main(
    ctx: &egui::Context,
    screen: [f32; 2],
    view: TitleMainView,
) -> Vec<TitleMainAction> {
    ctx.set_style(egui::Style::default());
    let mut actions = Vec::new();
    let size = Vec2::new(
        520.0_f32.min((screen[0] - 48.0).max(320.0)),
        600.0_f32.min((screen[1] - 48.0).max(420.0)),
    );
    let position = egui::pos2((screen[0] - size.x) * 0.5, (screen[1] - size.y) * 0.5);

    egui::Window::new("Some Frontier")
        .id(Id::new("title-main-window"))
        .default_pos(position)
        .fixed_size(size)
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .vscroll(true)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                if let Some(texture) = view.logo {
                    let width = (ui.available_width() - 44.0).min(420.0);
                    let height = (width / view.logo_aspect_ratio.max(0.1)).min(245.0);
                    ui.add(egui::Image::new((texture, Vec2::new(width, height))));
                } else {
                    ui.add_space(42.0);
                    ui.heading("Some Frontier");
                    ui.add_space(42.0);
                }
                ui.label("Local systems, rough ore, quiet engines");
                ui.add_space(18.0);
            });

            title_button(
                ui,
                "New Game",
                "N",
                true,
                TitleMainAction::NewGame,
                &mut actions,
            );
            title_button(
                ui,
                "Load Game",
                "L",
                view.load_enabled,
                TitleMainAction::LoadGame,
                &mut actions,
            );
            title_button(
                ui,
                "Content Packs",
                "C",
                true,
                TitleMainAction::ContentPacks,
                &mut actions,
            );
            title_button(
                ui,
                "Settings",
                "S",
                true,
                TitleMainAction::Settings,
                &mut actions,
            );
            ui.add_space(8.0);
            title_button(ui, "Quit", "Q", true, TitleMainAction::Quit, &mut actions);

            if view.fast_start {
                ui.add_space(10.0);
                ui.colored_label(
                    ui.visuals().warn_fg_color,
                    "Fast start enabled by SOME_FRONTIER_FAST_START",
                );
            }
        });

    actions
}

fn title_button(
    ui: &mut egui::Ui,
    label: &str,
    shortcut: &str,
    enabled: bool,
    action: TitleMainAction,
    actions: &mut Vec<TitleMainAction>,
) {
    if ui
        .add_enabled(
            enabled,
            egui::Button::new(format!("{label}    [{shortcut}]"))
                .min_size(Vec2::new(ui.available_width(), 38.0)),
        )
        .clicked()
    {
        actions.push(action);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubmenuKind {
    NewGame,
    LoadGame,
    ContentPacks,
    Settings,
}

#[derive(Clone, Debug)]
pub struct ShipChoice {
    pub name: String,
    pub pack: String,
    pub hull: f32,
    pub shields: f32,
    pub weapon_banks: usize,
    pub cargo_capacity: f32,
    pub mass: f32,
    pub forward_acceleration: f32,
    pub turn_acceleration: f32,
    pub texture: Option<TextureId>,
}

#[derive(Clone, Debug)]
pub struct SaveChoice {
    pub label: String,
    pub system: String,
    pub last_played: String,
    pub seed: u64,
    pub elapsed_days: f32,
    pub legacy: bool,
}

#[derive(Clone, Debug)]
pub struct PackOptionView {
    pub label: String,
    pub description: String,
    pub value: String,
    pub interactive: bool,
}

#[derive(Clone, Debug)]
pub struct PackChoice {
    pub name: String,
    pub id: String,
    pub version: String,
    pub description: String,
    pub options: Vec<PackOptionView>,
}

#[derive(Clone, Debug)]
pub struct SettingsChoice {
    pub category: String,
    pub label: String,
    pub value: String,
    pub note: String,
}

#[derive(Clone, Debug)]
pub struct TitleSubmenuView {
    pub kind: SubmenuKind,
    pub seed: String,
    pub seed_valid: bool,
    pub ships: Vec<ShipChoice>,
    pub selected_ship: usize,
    pub saves: Vec<SaveChoice>,
    pub selected_save: usize,
    pub confirming_delete: bool,
    pub delete_error: Option<String>,
    pub packs: Vec<PackChoice>,
    pub selected_pack: usize,
    pub settings: Vec<SettingsChoice>,
    pub selected_setting: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TitleSubmenuAction {
    Back,
    SetSeed(String),
    RandomizeSeed,
    SelectShip(usize),
    StartGame,
    SelectSave(usize),
    LoadSave,
    DeleteSave,
    SelectPack(usize),
    CyclePackOption(usize),
    SelectSetting(usize),
    AdjustSetting(i32),
}

pub fn draw(
    ctx: &egui::Context,
    screen: [f32; 2],
    view: &TitleSubmenuView,
) -> Vec<TitleSubmenuAction> {
    ctx.set_style(egui::Style::default());
    let mut actions = Vec::new();
    let size = submenu_size(screen, view.kind);
    let position = egui::pos2((screen[0] - size.x) * 0.5, (screen[1] - size.y) * 0.5);
    let title = match view.kind {
        SubmenuKind::NewGame => "New Game",
        SubmenuKind::LoadGame => "Load Game",
        SubmenuKind::ContentPacks => "Content Packs",
        SubmenuKind::Settings => "Settings",
    };

    egui::Window::new(title)
        .id(Id::new(("title-submenu", title)))
        .default_pos(position)
        .default_size(size)
        .min_size(Vec2::new(560.0_f32.min(size.x), 380.0_f32.min(size.y)))
        .resizable(true)
        .collapsible(false)
        .show(ctx, |ui| match view.kind {
            SubmenuKind::NewGame => draw_new_game(ui, view, &mut actions),
            SubmenuKind::LoadGame => {
                draw_load_game(ui, view, &mut actions, (size.y - 120.0).clamp(220.0, 430.0))
            }
            SubmenuKind::ContentPacks => {
                draw_content_packs(ui, view, &mut actions, (size.y - 120.0).clamp(220.0, 430.0))
            }
            SubmenuKind::Settings => draw_settings(ui, view, &mut actions),
        });

    actions
}

fn submenu_size(screen: [f32; 2], kind: SubmenuKind) -> Vec2 {
    let desired = match kind {
        SubmenuKind::NewGame => Vec2::new(1040.0, 700.0),
        SubmenuKind::LoadGame => Vec2::new(900.0, 580.0),
        SubmenuKind::ContentPacks => Vec2::new(900.0, 580.0),
        SubmenuKind::Settings => Vec2::new(760.0, 500.0),
    };
    Vec2::new(
        desired.x.min((screen[0] - 48.0).max(320.0)),
        desired.y.min((screen[1] - 48.0).max(280.0)),
    )
}

fn draw_new_game(
    ui: &mut egui::Ui,
    view: &TitleSubmenuView,
    actions: &mut Vec<TitleSubmenuAction>,
) {
    ui.label("Choose a ship and world seed for your fresh run.");
    ui.separator();
    ui.horizontal(|ui| {
        ui.label("World seed");
        let mut seed = view.seed.clone();
        let response = ui.add_sized([300.0, 24.0], egui::TextEdit::singleline(&mut seed));
        if response.changed() {
            seed.retain(|character| character.is_ascii_digit());
            seed.truncate(20);
            actions.push(TitleSubmenuAction::SetSeed(seed));
        }
        if !view.seed_valid {
            ui.colored_label(ui.visuals().warn_fg_color, "Enter a valid u64 seed");
        }
        if ui.button("Randomize").clicked() {
            actions.push(TitleSubmenuAction::RandomizeSeed);
        }
    });
    ui.add_space(6.0);
    ui.strong(format!("Select ship · {} loaded", view.ships.len()));

    let footer_height = 42.0;
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .max_height((ui.available_height() - footer_height).max(180.0))
        .show(ui, |ui| {
            let available = ui.available_width();
            let columns = if available >= 820.0 {
                3
            } else if available >= 540.0 {
                2
            } else {
                1
            };
            let gap = 10.0;
            let card_width = ((available - gap * (columns - 1) as f32) / columns as f32).max(220.0);
            egui::Grid::new("new-game-ship-grid")
                .num_columns(columns)
                .spacing([gap, gap])
                .show(ui, |ui| {
                    for (index, ship) in view.ships.iter().enumerate() {
                        let selected = index == view.selected_ship;
                        let fill = if selected {
                            ui.visuals().selection.bg_fill
                        } else {
                            ui.visuals().faint_bg_color
                        };
                        let response = egui::Frame::group(ui.style())
                            .fill(fill)
                            .show(ui, |ui| {
                                ui.set_width(card_width - 18.0);
                                ui.vertical(|ui| {
                                    ui.strong(&ship.name);
                                    ui.small(&ship.pack);
                                    if let Some(texture) = ship.texture {
                                        ui.add_sized(
                                            [card_width - 22.0, 120.0],
                                            egui::Image::new((texture, Vec2::new(240.0, 120.0)))
                                                .fit_to_exact_size(Vec2::new(
                                                    card_width - 22.0,
                                                    120.0,
                                                )),
                                        );
                                    } else {
                                        ui.add_sized(
                                            [card_width - 22.0, 120.0],
                                            egui::Label::new("Ship image unavailable"),
                                        );
                                    }
                                    ui.label(format!(
                                        "Hull {:.0}  ·  Shield {:.0}",
                                        ship.hull, ship.shields
                                    ));
                                    ui.label(format!(
                                        "Cargo {:.0} t  ·  Mass {:.0} t",
                                        ship.cargo_capacity / 1_000.0,
                                        ship.mass / 1_000.0
                                    ));
                                    ui.small(format!(
                                        "Thrust {:.0}  ·  Turn {:.1}",
                                        ship.forward_acceleration, ship.turn_acceleration
                                    ));
                                    ui.horizontal(|ui| {
                                        ui.label(format!(
                                            "{} turret {}",
                                            ship.weapon_banks,
                                            if ship.weapon_banks == 1 {
                                                "bank"
                                            } else {
                                                "banks"
                                            }
                                        ));
                                        if selected {
                                            ui.with_layout(
                                                Layout::right_to_left(Align::Center),
                                                |ui| {
                                                    ui.strong("SELECTED");
                                                },
                                            );
                                        }
                                    });
                                });
                            })
                            .response
                            .interact(egui::Sense::click());
                        if response.clicked() {
                            actions.push(TitleSubmenuAction::SelectShip(index));
                        }
                        if (index + 1) % columns == 0 {
                            ui.end_row();
                        }
                    }
                });
        });

    ui.separator();
    ui.horizontal(|ui| {
        if ui.button("Back").clicked() {
            actions.push(TitleSubmenuAction::Back);
        }
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui
                .add_enabled(
                    view.seed_valid && !view.ships.is_empty(),
                    egui::Button::new("Start Game"),
                )
                .clicked()
            {
                actions.push(TitleSubmenuAction::StartGame);
            }
        });
    });
}

fn draw_load_game(
    ui: &mut egui::Ui,
    view: &TitleSubmenuView,
    actions: &mut Vec<TitleSubmenuAction>,
    body_height: f32,
) {
    ui.label("Last played saves appear first.");
    ui.separator();
    let list_width = (ui.available_width() * 0.44).clamp(220.0, 340.0);
    ui.allocate_ui_with_layout(
        Vec2::new(ui.available_width(), body_height),
        Layout::left_to_right(Align::Min),
        |ui| {
            ui.allocate_ui_with_layout(
                Vec2::new(list_width, body_height),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.strong("Saved games");
                    ui.separator();
                    egui::ScrollArea::vertical()
                        .id_salt("title-save-list")
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            if view.saves.is_empty() {
                                ui.label("No saved games found.");
                            }
                            for (index, save) in view.saves.iter().enumerate() {
                                let response = ui.selectable_label(
                                    index == view.selected_save,
                                    format!(
                                        "{}\n{} · {}",
                                        save.label, save.system, save.last_played
                                    ),
                                );
                                if response.clicked() {
                                    actions.push(TitleSubmenuAction::SelectSave(index));
                                }
                                if response.double_clicked() {
                                    actions.push(TitleSubmenuAction::SelectSave(index));
                                    actions.push(TitleSubmenuAction::LoadSave);
                                }
                            }
                        });
                },
            );
            ui.separator();
            ui.allocate_ui_with_layout(
                Vec2::new(ui.available_width(), body_height),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.strong("Save details");
                    ui.separator();
                    if let Some(save) = view.saves.get(view.selected_save) {
                        ui.heading(&save.label);
                        ui.label(if save.legacy {
                            "Legacy save file"
                        } else {
                            "Save slot"
                        });
                        egui::Grid::new("title-save-details")
                            .num_columns(2)
                            .striped(true)
                            .show(ui, |ui| {
                                fact_row(ui, "Seed", &save.seed.to_string());
                                fact_row(ui, "System", &save.system);
                                fact_row(ui, "Elapsed", &format!("{:.1} days", save.elapsed_days));
                                fact_row(ui, "Played", &save.last_played);
                            });
                        if let Some(error) = &view.delete_error {
                            ui.colored_label(ui.visuals().error_fg_color, error);
                        } else if view.confirming_delete {
                            ui.colored_label(
                                ui.visuals().warn_fg_color,
                                "Click Confirm Delete again to permanently remove this save.",
                            );
                        }
                    }
                },
            );
        },
    );
    ui.separator();
    ui.horizontal(|ui| {
        if ui.button("Back").clicked() {
            actions.push(TitleSubmenuAction::Back);
        }
        if ui
            .add_enabled(
                !view.saves.is_empty(),
                egui::Button::new(if view.confirming_delete {
                    "Confirm Delete"
                } else {
                    "Delete"
                }),
            )
            .clicked()
        {
            actions.push(TitleSubmenuAction::DeleteSave);
        }
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui
                .add_enabled(!view.saves.is_empty(), egui::Button::new("Load Game"))
                .clicked()
            {
                actions.push(TitleSubmenuAction::LoadSave);
            }
        });
    });
}

fn draw_content_packs(
    ui: &mut egui::Ui,
    view: &TitleSubmenuView,
    actions: &mut Vec<TitleSubmenuAction>,
    body_height: f32,
) {
    ui.label("Installed content and new-game configuration.");
    ui.separator();
    ui.allocate_ui_with_layout(
        Vec2::new(ui.available_width(), body_height),
        Layout::left_to_right(Align::Min),
        |ui| {
            ui.allocate_ui_with_layout(
                Vec2::new(220.0_f32.min(ui.available_width()), body_height),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.strong("Installed");
                    ui.separator();
                    egui::ScrollArea::vertical()
                        .id_salt("title-content-pack-list")
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            for (index, pack) in view.packs.iter().enumerate() {
                                if ui
                                    .selectable_label(index == view.selected_pack, &pack.name)
                                    .clicked()
                                {
                                    actions.push(TitleSubmenuAction::SelectPack(index));
                                }
                            }
                        });
                },
            );
            ui.separator();
            ui.allocate_ui_with_layout(
                Vec2::new(ui.available_width(), body_height),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.strong("Configuration");
                    ui.separator();
                    if let Some(pack) = view.packs.get(view.selected_pack) {
                        ui.heading(&pack.name);
                        ui.small(format!("{} · v{}", pack.id, pack.version));
                        ui.label(&pack.description);
                        ui.separator();
                        ui.strong("Options");
                        if pack.options.is_empty() {
                            ui.label("No configurable options declared.");
                        }
                        egui::ScrollArea::vertical()
                            .id_salt("title-content-pack-options")
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                for (index, option) in pack.options.iter().enumerate() {
                                    egui::Frame::group(ui.style()).show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.vertical(|ui| {
                                                ui.strong(&option.label);
                                                ui.small(&option.description);
                                            });
                                            ui.with_layout(
                                                Layout::right_to_left(Align::Center),
                                                |ui| {
                                                    if ui
                                                        .add_enabled(
                                                            option.interactive,
                                                            egui::Button::new(&option.value),
                                                        )
                                                        .clicked()
                                                    {
                                                        actions.push(
                                                            TitleSubmenuAction::CyclePackOption(
                                                                index,
                                                            ),
                                                        );
                                                    }
                                                },
                                            );
                                        });
                                    });
                                }
                            });
                    } else {
                        ui.label("No content packs found.");
                    }
                },
            );
        },
    );
    ui.separator();
    if ui.button("Back").clicked() {
        actions.push(TitleSubmenuAction::Back);
    }
}

fn draw_settings(
    ui: &mut egui::Ui,
    view: &TitleSubmenuView,
    actions: &mut Vec<TitleSubmenuAction>,
) {
    ui.label("Saved separately from game saves.");
    ui.separator();
    ui.columns(2, |columns| {
        columns[0].set_width(190.0);
        columns[0].strong("Category");
        for (index, setting) in view.settings.iter().enumerate() {
            if columns[0]
                .selectable_label(index == view.selected_setting, &setting.category)
                .clicked()
            {
                actions.push(TitleSubmenuAction::SelectSetting(index));
            }
        }

        columns[1].strong("Preference");
        if let Some(setting) = view.settings.get(view.selected_setting) {
            columns[1].heading(&setting.category);
            columns[1].label(&setting.label);
            columns[1].horizontal(|ui| {
                if ui.small_button("−").clicked() {
                    actions.push(TitleSubmenuAction::AdjustSetting(-1));
                }
                ui.monospace(&setting.value);
                if ui.small_button("+").clicked() {
                    actions.push(TitleSubmenuAction::AdjustSetting(1));
                }
            });
            columns[1].separator();
            columns[1].label(&setting.note);
        }
    });
    ui.separator();
    if ui.button("Back").clicked() {
        actions.push(TitleSubmenuAction::Back);
    }
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
    fn submenu_sizes_fit_the_available_screen() {
        for kind in [
            SubmenuKind::NewGame,
            SubmenuKind::LoadGame,
            SubmenuKind::ContentPacks,
            SubmenuKind::Settings,
        ] {
            let size = submenu_size([640.0, 420.0], kind);
            assert!(size.x <= 592.0);
            assert!(size.y <= 372.0);
        }
    }
}
