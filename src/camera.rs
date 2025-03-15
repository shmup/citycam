use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Camera {
    pub name: String,
    pub url: String,
}

pub fn get_embedded_cameras() -> Result<Vec<Camera>> {
    let content = include_str!("../resources/cams.json");
    let cameras: Vec<Camera> = serde_json::from_str(content)?;
    Ok(cameras)
}

pub fn load_cameras<P: AsRef<Path>>(path: P) -> Result<Vec<Camera>> {
    let content = fs::read_to_string(path)?;
    let cameras: Vec<Camera> = serde_json::from_str(&content)?;
    Ok(cameras)
}

pub fn find_camera(cameras: &[Camera], selector: &str) -> Result<Camera> {
    if let Ok(index) = selector.parse::<usize>() {
        if index > 0 && index <= cameras.len() {
            return Ok(cameras[index - 1].clone());
        } else {
            return Err(anyhow!("Camera index out of range: {}", index));
        }
    }

    let selector_lower = selector.to_lowercase();
    for camera in cameras {
        if camera.name.to_lowercase().contains(&selector_lower) {
            return Ok(camera.clone());
        }
    }

    if selector.is_empty() && !cameras.is_empty() {
        return Ok(cameras[0].clone());
    }

    Err(anyhow!("Camera not found: {}", selector))
}

pub fn list_cameras(cameras: &[Camera]) -> String {
    let mut result = String::from("Available cameras:\n");
    for (i, camera) in cameras.iter().enumerate() {
        result.push_str(&format!("  {}. {}\n", i + 1, camera.name));
    }
    result
}
