use image::{imageops::FilterType};
use std::fs;
use std::path::{Path};

/// Changes the banners on the Corkscrew track.
/// If banner_path is empty, it resets the banners to their original state.
#[tauri::command]
pub fn change_banners(banner_path: &str) {
    // get the corkscrew path
    let exe_dir = std::env::current_exe()
        .expect("can't get exe path")
        .parent()
        .expect("exe has no parent")
        .to_path_buf();
    let corkscrew_path = exe_dir.join("torcs").join("tracks").join("road").join("corkscrew");
    
    // if empty string, reset all the banners
    if banner_path.is_empty() {
        let files = ["kilo", "TRUCK07", "64PASS1", "64PASS6", "treeRNS2", "64PASS9"];
        for file in files {
            let copy_path = corkscrew_path.join(format!("{}_copy.png", file));
            let original_path = corkscrew_path.join(format!("{}.png", file));
            if copy_path.exists() {
                let _ = fs::copy(&copy_path, &original_path);
            }
        }
    } else {
        let banner_img_path = Path::new(banner_path);
        if !banner_img_path.exists() {
            return;
        }

        // Handle truck (treeRNS2.png)
        let truck_file = "treeRNS2.png";
        let truck_outline = "treeRNS2_outline.png";
        let truck_res = [(176, 54), (36, 47)];
        let truck_coords: Vec<Vec<(u32, u32)>> = vec![
            vec![(0, 6), (0, 67), (0, 134), (0, 195)],
            vec![(182, 10), (182, 138)],
        ];

        let mut loop_count = 0;
        for (i, coords_list) in truck_coords.iter().enumerate() {
            let res = truck_res[i];
            let base_path = corkscrew_path.join(truck_file);

            let mut base = match image::open(&base_path) {
                Ok(img) => img.to_rgba8(),
                Err(_) => continue,
            };

            let mut banner_img = match image::open(banner_img_path) {
                    Ok(img) => img.to_rgba8(),
                    Err(_) => continue,
                };
            for &pos in coords_list {
                if loop_count == 0 {
                    banner_img = image::imageops::flip_horizontal(&banner_img);
                    banner_img = image::imageops::resize(&banner_img, res.0, res.1, FilterType::Nearest);
                    image::imageops::overlay(&mut base, &banner_img, pos.0 as i64, pos.1 as i64);
                } else {
                    // Rotate 270 and resize
                    let rotated = image::imageops::rotate270(&banner_img);
                    let resized = image::imageops::resize(&rotated, res.0, res.1, FilterType::Nearest);
                    image::imageops::overlay(&mut base, &resized, pos.0 as i64, pos.1 as i64);
                }
            }

            let outline_path = corkscrew_path.join(truck_outline);
            if let Ok(outline_img) = image::open(&outline_path) {
                image::imageops::overlay(&mut base, &outline_img.to_rgba8(), 0, 0);
            }

            let _ = base.save(base_path);
            loop_count+=1;
        }

        // Handle other files
        let files = ["kilo.png", "TRUCK07.png", "64PASS1.png", "64PASS1.png", "64PASS1.png", "64PASS6.png", "64PASS9.png"];
        let target_res = [(512, 256), (128, 64), (512, 123), (492, 71), (211, 124), (512, 158), (211, 119)];
        let start_coords = [(0, 0), (0, 64), (0, 228), (10, 133), (216, 376), (0, 0), (0, 198)];

        for i in 0..files.len() {
            let base_path = corkscrew_path.join(files[i]);
            let mut base = match image::open(&base_path) {
                Ok(img) => img.to_rgba8(),
                Err(_) => continue,
            };

            if let Ok(banner) = image::open(banner_img_path) {
                let mut resized = image::imageops::resize(&banner.to_rgba8(), target_res[i].0, target_res[i].1, FilterType::Nearest);

                // special case with TRUCK07
                if files[i]=="TRUCK07.png" {
                    resized=image::imageops::flip_horizontal(&resized);
                }
                image::imageops::overlay(&mut base, &resized, start_coords[i].0 as i64, start_coords[i].1 as i64);
                let _ = base.save(base_path);
            }
        }
    }
}
