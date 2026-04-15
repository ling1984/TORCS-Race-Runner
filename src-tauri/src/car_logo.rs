use image::{ImageReader};
use image::imageops::FilterType;
use std::path::PathBuf;

pub fn overlay_car_logo(car_index: u32, image_path: &str, exe_dir: &PathBuf) -> Result<(), String> {
    let base_dir = exe_dir.join("torcs").join("drivers").join("scr_server").join(car_index.to_string());
    let copy_path = base_dir.join("car1-ow1 - Copy.rgb");
    let result_path = base_dir.join("car1-ow1.rgb");

    // Load base image
    let base = ImageReader::open(&copy_path)
        .map_err(|e| format!("Failed to open base image: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode base image: {}", e))?;
    // SgiDecoder::new(r)

    // Load overlay
    let overlay = ImageReader::open(image_path)
        .map_err(|e| format!("Failed to open overlay: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode overlay: {}", e))?;

    // Convert to RGBA8 for easier overlaying
    let base_rgba = base.to_rgba8();
    let overlay_rgba = overlay.to_rgba8();

    // Scale
    let overlay_61 = image::imageops::resize(&overlay_rgba, 61, 33, FilterType::Nearest);
    let overlay_37 = image::imageops::resize(&overlay_rgba, 37, 20, FilterType::Nearest);

    // Regions
    let mut base_img = base_rgba;
    let regions_61 = vec![(399, 472), (54, 402)];
    let regions_37 = vec![(93, 461), (387, 392)];

    for (x, y) in regions_61 {
        image::imageops::overlay(&mut base_img, &overlay_61, x as i64, y as i64);
    }

    for (x, y) in regions_37 {
        image::imageops::overlay(&mut base_img, &overlay_37, x as i64, y as i64);
    }

    base_img.save(result_path)
        .map_err(|e| format!("Failed to save image: {}", e))?;

    Ok(())
}
