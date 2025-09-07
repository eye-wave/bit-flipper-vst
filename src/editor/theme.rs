use std::borrow::Cow;
use std::fs;
use std::path::PathBuf;

const DEFAULT_PALETTE: &[u8] = include_bytes!("../../assets/textures/__palette__.png");
const DEFAULT_ATLAS: &[u8] = include_bytes!("../../assets/textures/__texture_atlas__.png");

fn get_or_write(tex_path: &PathBuf, fallback: &'static [u8]) -> Cow<'static, [u8]> {
    if !tex_path.exists() {
        fs::write(tex_path, fallback).ok();

        return Cow::Borrowed(fallback);
    }

    match fs::read(tex_path) {
        Ok(data) => Cow::Owned(data),
        Err(_) => Cow::Borrowed(fallback),
    }
}

pub fn load_textures() -> (Cow<'static, [u8]>, Cow<'static, [u8]>) {
    let config_dir = dirs::config_dir();
    if let Some(tex_path) = config_dir.map(|p| p.join(env!("CARGO_PKG_NAME"))) {
        let atlas_path = tex_path.join("__texture_atlas__.png");
        let palette_path = tex_path.join("__palette__.png");

        let atlas = get_or_write(&atlas_path, DEFAULT_ATLAS);
        let palette = get_or_write(&palette_path, DEFAULT_PALETTE);

        (palette, atlas)
    } else {
        (Cow::Borrowed(DEFAULT_PALETTE), Cow::Borrowed(DEFAULT_ATLAS))
    }
}

pub fn open_theme_dir() -> std::io::Result<()> {
    let config_dir = dirs::config_dir();
    if let Some(tex_path) = config_dir.map(|p| p.join(env!("CARGO_PKG_NAME"))) {
        if tex_path.exists() {
            open::that(tex_path)?;
        }
    }

    Ok(())
}
