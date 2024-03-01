use image::{ImageBuffer, imageops, Rgba, RgbaImage};

pub struct Renderer {
    pub m_image_data: RgbaImage,
    pub m_width: u32,
    pub m_height: u32,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Renderer {
        return Renderer {
            m_image_data: ImageBuffer::new(width, height),
            m_width: width,
            m_height: height,
        };
    }
    pub fn render(&mut self) -> RgbaImage {
        for (x, y, pixel) in self.m_image_data.enumerate_pixels_mut() {
            *pixel = Self::frag(
                x as f32 / self.m_width as f32,
                y as f32 / self.m_height as f32,
            );
        }

        /* correct uv coordinates */
        let flipped = imageops::flip_vertical(&self.m_image_data.clone());
        return flipped;
    }

    /* this is basically the fragment shader */
    pub fn frag(width: f32, height: f32) -> Rgba<u8> {
        let r: u8 = (width * 255.0) as u8;
        let g: u8 = (height * 255.0) as u8;
        return image::Rgba([r, g, 0, 255]);
    }
}
