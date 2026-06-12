use image::imageops::FilterType;
use std::path::Path;

use image::DynamicImage;
use image_extras::sgi::SgiDecoder;
use std::fs::{copy, File};
use std::io::BufReader;

use crate::sgi_encoder;

pub fn overlay_car_logo(car_index: u32, image_path: &str, exe_dir: &Path) -> Result<(), String> {
    let base_dir = exe_dir
        .join("torcs")
        .join("drivers")
        .join("scr_server")
        .join(car_index.to_string());
    let copy_path = base_dir.join("car1-ow1 - Copy.rgb");
    let result_path = base_dir.join("car1-ow1.rgb");

    // Load base image
    // - RGB format is not supported by image or image-extras directly.
    // - SGI format is equivalent and supported by image-extras.
    // We use file instead of image
    let file = File::open(&copy_path).map_err(|e| format!("Failed to open base image: {}", e))?;
    let decoder = SgiDecoder::new(BufReader::new(file))
        .map_err(|e| format!("Failed to decode base image: {}", e))?;
    let base = DynamicImage::from_decoder(decoder)
        .map_err(|e| format!("Failed to convert to DynamicImage: {}", e))?;

    // Load overlay
    let overlay = image::ImageReader::open(image_path)
        .map_err(|e| format!("Failed to open overlay: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode overlay: {}", e))?;

    // Convert to RGBA8 for easier overlaying
    let base_rgba: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = base.to_rgba8();
    let overlay_rgba: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = overlay.to_rgba8();

    // Scale
    let overlay_61 = image::imageops::resize(&overlay_rgba, 61, 33, FilterType::Nearest);
    let overlay_37 = image::imageops::resize(&overlay_rgba, 37, 20, FilterType::Nearest);

    // Regions
    let mut base_img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = base_rgba;
    let regions_61 = vec![(399, 472), (54, 402)];
    let regions_37 = vec![(93, 461), (387, 392)];

    for (x, y) in regions_61 {
        image::imageops::overlay(&mut base_img, &overlay_61, x as i64, y as i64);
    }

    for (x, y) in regions_37 {
        image::imageops::overlay(&mut base_img, &overlay_37, x as i64, y as i64);
    }

    // we need to convert rgba8 to sgi format
    match sgi_encoder::write_sgi(base_img, &result_path) {
        Ok(()) => println!("SGI write succeeded"),
        Err(e) => eprintln!("SGI write failed: {}", e),
    }

    Ok(())
}

pub fn reset_car_logo(car_index: u32, exe_dir: &Path) -> Result<(), String> {
    let base_dir = exe_dir
        .join("torcs")
        .join("drivers")
        .join("scr_server")
        .join(car_index.to_string());
    let copy_path = base_dir.join("car1-ow1 - Copy.rgb");
    let result_path = base_dir.join("car1-ow1.rgb");

    copy(&copy_path, &result_path).map_err(|e| format!("Failed to copy file: {}", e))?;

    Ok(())
}
