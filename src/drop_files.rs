use once_cell::sync::Lazy;
use std::{fs, path::PathBuf};

static EMBEDDED_PALETTE: &[u8] = include_bytes!("../assets/textures/__palette__.png");
static EMBEDDED_ATLAS: &[u8] = include_bytes!("../assets/textures/__texture_atlas__.png");

fn texture_path() -> PathBuf {
    dirs::config_dir().unwrap().join(env!("CARGO_PKG_NAME"))
}

pub fn palette_path() -> PathBuf {
    texture_path().join("__palette__.png")
}

pub fn atlas_path() -> PathBuf {
    texture_path().join("__texture_atlas__.png")
}

fn ensure_files() {
    let dir = texture_path();
    if !dir.exists() {
        fs::create_dir_all(&dir).expect("failed to create config dir");
    }

    if !palette_path().exists() {
        fs::write(palette_path(), EMBEDDED_PALETTE).expect("failed to write palette");
    }

    if !atlas_path().exists() {
        fs::write(atlas_path(), EMBEDDED_ATLAS).expect("failed to write atlas");
    }
}

pub static PALETTE_BYTES: Lazy<&'static [u8]> = Lazy::new(|| {
    ensure_files();
    let bytes = fs::read(palette_path()).expect("palette missing even after embed");
    Box::leak(bytes.into_boxed_slice())
});

pub static ATLAS_BYTES: Lazy<&'static [u8]> = Lazy::new(|| {
    ensure_files();
    let bytes = fs::read(atlas_path()).expect("atlas missing even after embed");
    Box::leak(bytes.into_boxed_slice())
});
