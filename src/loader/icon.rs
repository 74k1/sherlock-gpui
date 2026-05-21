use crate::loader::{
    assets::Assets,
    icon::{render::copy_png_to_cache, theme::resolve_icon_internal},
};

const ICON_SIZE: u16 = 48;

mod cache;
mod render;
mod theme;

pub use cache::{CustomIconTheme, IconThemeGuard, IconType};
use render::{render_svg_to_cache, render_to_png_cache};

pub fn resolve_icon_path(name: &str) -> Option<IconType> {
    // 1. Check in-memory HashMap cache
    if let Ok(Some(icon)) = IconThemeGuard::lookup_icon(name) {
        return icon;
    }

    let mut result: Option<IconType> = None;

    // Check embedded files
    if let Some(asset) = Assets::get(&format!("icons/{name}.svg")) {
        result = render_to_png_cache(name, &asset.data).map(IconType::Png);
    } else if let Some(asset) = Assets::get(&format!("icons/{name}.png")) {
        result = copy_png_to_cache(name, &asset.data).map(IconType::Png);
    }

    if result.is_none() {
        result = resolve_icon_internal(name).and_then(|p| render_svg_to_cache(name, p));
    }

    if result.is_none() {
        result = Assets::get("icons/400.svg")
            .and_then(|a| render_to_png_cache("placeholder", &a.data))
            .map(IconType::Png);
    }

    // Finalize: Write found result back to the Guard buffer
    if let Ok(mut cache) = IconThemeGuard::get_write() {
        cache.buf.insert(name.to_string(), result.clone());
    }

    result
}
