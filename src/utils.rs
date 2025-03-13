use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

pub fn get_cache_dir() -> Result<PathBuf> {
    let cache_dir = dirs::cache_dir()
        .ok_or_else(|| anyhow!("Could not determine cache directory"))?
        .join(env!("CARGO_PKG_NAME"));
    Ok(cache_dir)
}

pub fn set_wallpaper(path: &Path) -> Result<()> {
    wallpaper::set_from_path(path.to_str().unwrap())
        .map_err(|e| anyhow!("Failed to set wallpaper: {}", e))?;

    Ok(())
}
