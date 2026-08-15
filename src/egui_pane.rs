use egui_macroquad::egui::{self, Id, InnerResponse, Pos2};
use std::hash::Hash;

fn position_id(window_id: Id) -> Id {
    window_id.with("content-independent-position")
}

/// Shows a movable window while keeping its position attached to the pane type.
///
/// Egui also stores area geometry, but that state can be reset or constrained as
/// window contents change. This separate position survives those area changes and
/// is updated only when the player actually drags the pane.
pub fn show_window<R>(
    ctx: &egui::Context,
    window_id: Id,
    window: egui::Window<'_>,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> Option<InnerResponse<Option<R>>> {
    let saved_position = ctx.data_mut(|data| data.get_persisted::<Pos2>(position_id(window_id)));
    let initial_position = saved_position
        .or_else(|| ctx.memory(|memory| memory.area_rect(window_id).map(|rect| rect.left_top())));
    let window = match initial_position {
        Some(position) => window.current_pos(position),
        None => window,
    };
    let response = window.show(ctx, add_contents);

    if let Some(response) = &response {
        if saved_position.is_none() || ctx.is_being_dragged(window_id.with("move")) {
            ctx.data_mut(|data| {
                data.insert_persisted(position_id(window_id), response.response.rect.left_top());
            });
        }
    }

    response
}

pub fn forget_position(ctx: &egui::Context, window_id: Id) {
    ctx.data_mut(|data| data.remove::<Pos2>(position_id(window_id)));
}

pub fn vertical_scroll<R>(
    ui: &mut egui::Ui,
    id_salt: impl Hash,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    let height = ui.available_height().max(1.0);
    egui::ScrollArea::vertical()
        .id_salt(id_salt)
        .auto_shrink([false, false])
        .min_scrolled_height(1.0)
        .max_height(height)
        .show(ui, add_contents)
        .inner
}

pub fn both_scroll<R>(
    ui: &mut egui::Ui,
    id_salt: impl Hash,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    let viewport = ui.available_size().max(egui::Vec2::splat(1.0));
    egui::ScrollArea::both()
        .id_salt(id_salt)
        .auto_shrink([false, false])
        .min_scrolled_width(1.0)
        .min_scrolled_height(1.0)
        .max_width(viewport.x)
        .max_height(viewport.y)
        .show(ui, add_contents)
        .inner
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draw_vertical_scroll_test_frame(ctx: &egui::Context, events: Vec<egui::Event>) -> Id {
        let scroll_id = std::cell::Cell::new(None);
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(400.0, 240.0),
            )),
            events,
            ..Default::default()
        };
        let _ = ctx.run(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                scroll_id.set(Some(
                    ui.make_persistent_id(Id::new("bounded-vertical-scroll-test")),
                ));
                vertical_scroll(ui, "bounded-vertical-scroll-test", |ui| {
                    for row in 0..40 {
                        ui.label(format!("Overflow row {row}"));
                    }
                });
            });
        });
        scroll_id.get().expect("vertical scroll area should draw")
    }

    fn draw_resized_window_scroll_test_frame(
        ctx: &egui::Context,
        events: Vec<egui::Event>,
        both_axes: bool,
    ) -> Id {
        let scroll_id = std::cell::Cell::new(None);
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(600.0, 400.0),
            )),
            events,
            ..Default::default()
        };
        let _ = ctx.run(input, |ctx| {
            egui::Window::new("Overflow inventory")
                .id(Id::new("resized-overflow-window"))
                .fixed_pos(egui::pos2(20.0, 20.0))
                .fixed_size(egui::vec2(260.0, 45.0))
                .show(ctx, |ui| {
                    scroll_id.set(Some(
                        ui.make_persistent_id(Id::new("resized-overflow-scroll")),
                    ));
                    let rows = |ui: &mut egui::Ui| {
                        for row in 0..3 {
                            ui.label(format!("Inventory row {row} with a long material name"));
                        }
                    };
                    if both_axes {
                        both_scroll(ui, "resized-overflow-scroll", rows);
                    } else {
                        vertical_scroll(ui, "resized-overflow-scroll", rows);
                    }
                });
        });
        scroll_id
            .get()
            .expect("resized window scroll area should draw")
    }

    #[test]
    fn pane_position_survives_area_reset_and_content_changes() {
        let ctx = egui::Context::default();
        let window_id = Id::new("planet-workspace-window");

        ctx.begin_pass(egui::RawInput::default());
        show_window(
            &ctx,
            window_id,
            egui::Window::new("Planet")
                .id(window_id)
                .default_pos(egui::pos2(240.0, 180.0)),
            |ui| ui.label("First planet"),
        );
        let _ = ctx.end_pass();

        ctx.memory_mut(|memory| memory.reset_areas());

        ctx.begin_pass(egui::RawInput::default());
        show_window(
            &ctx,
            window_id,
            egui::Window::new("Planet")
                .id(window_id)
                .default_pos(egui::pos2(16.0, 48.0)),
            |ui| ui.label("A different planet with different contents"),
        );
        let _ = ctx.end_pass();

        let rect = ctx
            .memory(|memory| memory.area_rect(window_id))
            .expect("planet pane should retain an area");
        assert_eq!(rect.left_top(), egui::pos2(240.0, 180.0));
    }

    #[test]
    fn pane_types_keep_independent_positions() {
        let ctx = egui::Context::default();
        let planet_id = Id::new("planet-workspace-window");
        let station_id = Id::new("station-workspace-window");

        ctx.begin_pass(egui::RawInput::default());
        show_window(
            &ctx,
            planet_id,
            egui::Window::new("Planet")
                .id(planet_id)
                .default_pos(egui::pos2(120.0, 140.0)),
            |ui| ui.label("Planet"),
        );
        show_window(
            &ctx,
            station_id,
            egui::Window::new("Station")
                .id(station_id)
                .default_pos(egui::pos2(520.0, 260.0)),
            |ui| ui.label("Station"),
        );
        let _ = ctx.end_pass();

        assert_eq!(
            ctx.memory(|memory| memory.area_rect(planet_id).map(|rect| rect.left_top())),
            Some(egui::pos2(120.0, 140.0))
        );
        assert_eq!(
            ctx.memory(|memory| memory.area_rect(station_id).map(|rect| rect.left_top())),
            Some(egui::pos2(520.0, 260.0))
        );
    }

    #[test]
    fn bounded_vertical_scroll_consumes_wheel_input() {
        let ctx = egui::Context::default();
        let scroll_id = draw_vertical_scroll_test_frame(&ctx, Vec::new());
        let scroll_id_after = draw_vertical_scroll_test_frame(
            &ctx,
            vec![
                egui::Event::PointerMoved(egui::pos2(100.0, 100.0)),
                egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta: egui::vec2(0.0, -120.0),
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );

        assert_eq!(scroll_id_after, scroll_id);
        let offset = egui::scroll_area::State::load(&ctx, scroll_id)
            .expect("vertical scroll state should persist")
            .offset;
        assert!(offset.y > 0.0, "wheel input should scroll bounded content");
    }

    #[test]
    fn resized_window_with_overflowing_content_consumes_wheel_input() {
        let ctx = egui::Context::default();
        let scroll_id = draw_resized_window_scroll_test_frame(&ctx, Vec::new(), true);
        let scroll_id_after = draw_resized_window_scroll_test_frame(
            &ctx,
            vec![
                egui::Event::PointerMoved(egui::pos2(120.0, 70.0)),
                egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta: egui::vec2(0.0, -120.0),
                    modifiers: egui::Modifiers::NONE,
                },
            ],
            true,
        );

        assert_eq!(scroll_id_after, scroll_id);
        let offset = egui::scroll_area::State::load(&ctx, scroll_id)
            .expect("resized scroll state should persist")
            .offset;
        assert!(
            offset.y > 0.0,
            "wheel input should scroll resized window content"
        );
    }

    #[test]
    fn resized_vertical_window_with_short_overflow_consumes_wheel_input() {
        let ctx = egui::Context::default();
        let scroll_id = draw_resized_window_scroll_test_frame(&ctx, Vec::new(), false);
        draw_resized_window_scroll_test_frame(
            &ctx,
            vec![
                egui::Event::PointerMoved(egui::pos2(120.0, 70.0)),
                egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta: egui::vec2(0.0, -120.0),
                    modifiers: egui::Modifiers::NONE,
                },
            ],
            false,
        );

        let offset = egui::scroll_area::State::load(&ctx, scroll_id)
            .expect("vertical resized scroll state should persist")
            .offset;
        assert!(offset.y > 0.0, "wheel input should scroll short overflow");
    }
}
