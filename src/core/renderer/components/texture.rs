use gl::{self, types::*};
use image;

#[derive(Debug)]
pub struct Texture {
    handle: u32,
}

impl Texture {
    pub fn new(image: image::DynamicImage) -> Self {
        let image = match image {
            image::DynamicImage::ImageRgb8(img) => img,
            x => x.to_rgb8(),
        };

        let handle = unsafe {
            let mut handle = 0u32;

            gl::ActiveTexture(gl::TEXTURE0);
            gl::GenTextures(1, &mut handle);
            gl::BindTexture(gl::TEXTURE_2D, handle);

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::MIRRORED_REPEAT as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::MIRRORED_REPEAT as i32,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                image.width() as i32,
                image.height() as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                (&image as &[u8]).as_ptr() as *const GLvoid,
            );

            handle
        };

        Texture { handle }
    }

    pub fn load_from(path: &str) -> Result<Texture, image::ImageError> {
        let image = image::open(path)?;
        Ok(Self::new(image))
    }
}
