use image::{self, ImageBuffer, Rgba};

const DEF_IMAGE_SIZE: u32 = 256;

const MONOSPACE: &[u8] = include_bytes!("../assets/monospace.png");

pub struct Ass {
    pub font: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Ass {
    pub fn load() -> Ass {
        let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(DEF_IMAGE_SIZE, DEF_IMAGE_SIZE);

        for pixel in img.enumerate_pixels_mut() {
            *pixel.2 = image::Rgba([255,255,255,255]);
        }

        let image_result = image::load_from_memory_with_format(MONOSPACE, image::ImageFormat::Png);
        let image = match image_result {
            Ok(image_result) => image_result.to_rgba8(),
            Err(_image_result) => img.clone()
        };

        Ass {
            font: image,
        }
    }
}
