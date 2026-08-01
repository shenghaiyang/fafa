use anyhow::{Context, Result};
use gpui::{App, AssetSource, SharedString};
use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "assets"]
#[include = "fonts/**/*"]
#[include = "icons/**/*"]
#[include = "images/**/*"]
#[exclude = "*.DS_Store"]
pub struct Assets;

impl Assets {
    pub fn load_fonts(&self, cx: &App) {
        let font_paths = self.list("fonts").expect("Font paths not found");
        let mut embedded_fonts = Vec::new();
        for font_path in font_paths {
            if font_path.ends_with(".ttf") {
                let font_bytes = cx
                    .asset_source()
                    .load(&font_path)
                    .expect("Error loading font file")
                    .expect("Assets should never return None");
                embedded_fonts.push(font_bytes);
            }
        }
        cx.text_system()
            .add_fonts(embedded_fonts)
            .expect("Failed to add fonts");
    }
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }
        Self::get(path)
            .map(|f| Some(f.data))
            .with_context(|| format!("loading asset at path {path:?}"))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}
