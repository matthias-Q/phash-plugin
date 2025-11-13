use image::DynamicImage;
use rustdct::DctPlanner;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process_image(bytes: &[u8]) -> String {
    if let Ok(image) = image::load_from_memory(bytes) {
        if let Ok(hash) = phash(&image, 8, 12) {
            hash
        } else {
            "error: failed to compute hash".to_string()
        }
    } else {
        "error: failed to load image".to_string()
    }
}

fn phash(image: &DynamicImage, hash_size: usize, highfreq_factor: usize) -> Result<String, String> {
    if hash_size < 2 {
        return Err("Hash size must be greater than or equal to 2".to_string());
    }

    let img_size = hash_size * highfreq_factor;

    // Convert to grayscale and resize
    let gray_image = image.to_luma8();
    let resized = image::imageops::resize(
        &gray_image,
        img_size as u32,
        img_size as u32,
        image::imageops::FilterType::Lanczos3,
    );

    // Convert to f32 matrix
    let mut pixels: Vec<Vec<f32>> = vec![vec![0.0; img_size]; img_size];
    for (y, row) in pixels.iter_mut().enumerate() {
        for (x, pixel) in row.iter_mut().enumerate() {
            *pixel = resized.get_pixel(x as u32, y as u32)[0] as f32;
        }
    }

    // Apply 2D DCT
    let dct = dct_2d(&pixels, img_size);

    // Extract low frequency components (excluding DC component at [0,0])
    let mut dct_lowfreq: Vec<Vec<f32>> = vec![vec![0.0; hash_size]; hash_size];
    for y in 0..hash_size {
        for x in 0..hash_size {
            dct_lowfreq[y][x] = dct[y][x];
        }
    }

    // Calculate mean (excluding DC component)
    let mut sum = 0.0;
    let mut count = 0;
    (0..hash_size).for_each(|y| {
        (0..hash_size).for_each(|x| {
            if x != 0 || y != 0 {
                sum += dct_lowfreq[y][x];
                count += 1;
            }
        });
    });
    let mean = sum / count as f32;

    // Generate hash
    let mut hash_bits = String::new();
    (0..hash_size).for_each(|y| {
        (0..hash_size).for_each(|x| {
            if dct_lowfreq[y][x] > mean {
                hash_bits.push('1');
            } else {
                hash_bits.push('0');
            }
        });
    });

    // Convert to hexadecimal
    let hex_hash = bits_to_hex(&hash_bits);
    Ok(hex_hash)
}

fn dct_2d(matrix: &[Vec<f32>], size: usize) -> Vec<Vec<f32>> {
    let mut planner = DctPlanner::new();
    let dct = planner.plan_dct2(size);

    // DCT on rows
    let mut temp: Vec<Vec<f32>> = vec![vec![0.0; size]; size];
    for y in 0..size {
        let mut row = matrix[y].clone();
        dct.process_dct2(&mut row);
        temp[y] = row;
    }

    // DCT on columns
    let mut result: Vec<Vec<f32>> = vec![vec![0.0; size]; size];
    for x in 0..size {
        let mut col: Vec<f32> = (0..size).map(|y| temp[y][x]).collect();
        dct.process_dct2(&mut col);
        for y in 0..size {
            result[y][x] = col[y];
        }
    }

    result
}
fn bits_to_hex(bits: &str) -> String {
    let mut hex = String::new();
    for chunk in bits.as_bytes().chunks(4) {
        let mut val = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            if bit == b'1' {
                val |= 1 << (3 - i);
            }
        }
        hex.push_str(&format!("{:x}", val));
    }
    hex
}
