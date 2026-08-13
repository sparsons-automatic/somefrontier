use egui_macroquad::egui::{self, ColorImage, TextureHandle, TextureId, Vec2};
use std::{cell::RefCell, collections::HashMap};

thread_local! {
    static ART_CACHE: RefCell<ArtCache> = RefCell::new(ArtCache::default());
}

#[derive(Default)]
struct ArtCache {
    sources: HashMap<String, ColorImage>,
    textures: HashMap<String, TextureHandle>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ArtTexture {
    id: TextureId,
    size: Vec2,
}

pub fn install_sources(sources: HashMap<String, ColorImage>) {
    ART_CACHE.with(|cache| {
        *cache.borrow_mut() = ArtCache {
            sources,
            textures: HashMap::new(),
        };
    });
}

pub fn preview(ui: &mut egui::Ui, artwork: Option<&str>, max_size: Vec2) -> bool {
    let Some(artwork) = artwork else {
        return false;
    };
    let Some(texture) = texture(ui.ctx(), artwork) else {
        return false;
    };
    let available = Vec2::new(ui.available_width().min(max_size.x), max_size.y);
    let size = contained_size(texture.size, available);
    if size.x <= 0.0 || size.y <= 0.0 {
        return false;
    }

    ui.allocate_ui_with_layout(
        Vec2::new(ui.available_width(), size.y + 8.0),
        egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
        |ui| {
            egui::Frame::group(ui.style())
                .inner_margin(4.0)
                .show(ui, |ui| {
                    ui.add(egui::Image::new((texture.id, size)));
                });
        },
    );
    true
}

pub fn card_button(
    ui: &mut egui::Ui,
    artwork: Option<&str>,
    text: impl Into<egui::WidgetText>,
    selected: bool,
    enabled: bool,
) -> egui::Response {
    let text = text.into();
    let button = artwork
        .and_then(|artwork| texture(ui.ctx(), artwork))
        .map(|texture| {
            let image = egui::Image::new((
                texture.id,
                contained_size(texture.size, Vec2::new(128.0, 78.0)),
            ));
            egui::Button::image_and_text(image, text.clone())
        })
        .unwrap_or_else(|| egui::Button::new(text));
    ui.add_enabled(
        enabled,
        button
            .selected(selected)
            .min_size(Vec2::new(ui.available_width(), 92.0)),
    )
}

fn texture(ctx: &egui::Context, key: &str) -> Option<ArtTexture> {
    ART_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if !cache.textures.contains_key(key) {
            let source = cache.sources.get(key)?.clone();
            let handle = ctx.load_texture(
                format!("some-frontier-art:{key}"),
                source,
                egui::TextureOptions::LINEAR,
            );
            cache.textures.insert(key.to_string(), handle);
        }
        let handle = cache.textures.get(key)?;
        Some(ArtTexture {
            id: handle.id(),
            size: Vec2::new(handle.size()[0] as f32, handle.size()[1] as f32),
        })
    })
}

fn contained_size(source: Vec2, bounds: Vec2) -> Vec2 {
    if source.x <= 0.0 || source.y <= 0.0 || bounds.x <= 0.0 || bounds.y <= 0.0 {
        return Vec2::ZERO;
    }
    let scale = (bounds.x / source.x).min(bounds.y / source.y);
    source * scale
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn previews_preserve_artwork_aspect_ratio() {
        let fitted = contained_size(Vec2::new(320.0, 160.0), Vec2::new(120.0, 120.0));

        assert_eq!(fitted, Vec2::new(120.0, 60.0));
        assert_eq!(contained_size(Vec2::ZERO, Vec2::splat(120.0)), Vec2::ZERO);
    }
}
