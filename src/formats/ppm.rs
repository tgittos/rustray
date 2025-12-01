use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

pub struct PpmImage {
    pub width: usize,
    pub height: usize,
    pub max_color_value: u8,
    pub data: Vec<u8>,
}

pub fn new_ppm_image(width: usize, height: usize, max_color_value: Option<u8>) -> PpmImage {
    let data = vec![0; width * height * 3]; // Initialize with black pixels
    PpmImage {
        width,
        height,
        max_color_value: max_color_value.unwrap_or(255),
        data,
    }
}

pub fn write_ppm(file_path: &str, ppm_image: PpmImage) -> Result<()> {
    let mut file = File::create(file_path)?;
    let header = format!(
        "P3\n{} {}\n{}\n",
        ppm_image.width, ppm_image.height, ppm_image.max_color_value
    );
    let mut contents = header;
    for y in (0..ppm_image.height).rev() {
        for x in 0..ppm_image.width {
            let offset = (y * ppm_image.width + x) * 3;
            let ir = ppm_image.data[offset];
            let ig = ppm_image.data[offset + 1];
            let ib = ppm_image.data[offset + 2];
            contents.push_str(&format!("{} {} {} ", ir, ig, ib));
        }
        contents.push('\n');
    }
    file.write_all(contents.as_bytes())
}