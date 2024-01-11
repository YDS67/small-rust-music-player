use image::{self, ImageBuffer, Rgba};
use std::path::Path;

use crate::settings;

const DEF_IMAGE_SIZE: u32 = settings::MAPSIZE as u32;

pub struct Ass {
    pub font: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Ass {
    pub fn load() -> Ass {
        let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(DEF_IMAGE_SIZE, DEF_IMAGE_SIZE);

        for pixel in img.enumerate_pixels_mut() {
            *pixel.2 = image::Rgba([255,255,255,255]);
        }

        let mut images: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> = Vec::new();

        let paths: Vec<&str> = vec![
            "assets/monospace.png",
        ];

        for path in paths {
            let image_result = image::open(Path::new(path));
            let image = match image_result {
                Ok(image_result) => image_result.to_rgba8(),
                Err(_image_result) => img.clone()
            };
            images.push(image)
        }

        Ass {
            font: images[0].clone(),
        }
    }
}
