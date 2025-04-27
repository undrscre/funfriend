use gl::types::*;
use image::DynamicImage;
use std::{collections::HashMap, path::Path};

const TEXTURE_WRAP_S: GLenum = gl::TEXTURE_WRAP_S;
const TEXTURE_WRAP_T: GLenum = gl::TEXTURE_WRAP_T;
const TEXTURE_MIN_FILTER: GLenum = gl::TEXTURE_MIN_FILTER;
const TEXTURE_MAG_FILTER: GLenum = gl::TEXTURE_MAG_FILTER;
const CLAMP_TO_BORDER: GLenum = gl::CLAMP_TO_BORDER;
const NEAREST: GLenum = gl::NEAREST;

#[derive(Debug, Clone, Copy)]
pub struct SizedTexture {
    pub tex: GLuint, // OpenGL texture ID
    pub width: i32,
    pub height: i32,
}

fn get_default_texture_params() -> HashMap<GLenum, GLenum> {
    let mut params = HashMap::new();
    params.insert(TEXTURE_WRAP_S, CLAMP_TO_BORDER);
    params.insert(TEXTURE_WRAP_T, CLAMP_TO_BORDER);
    params.insert(TEXTURE_MIN_FILTER, NEAREST);
    params.insert(TEXTURE_MAG_FILTER, NEAREST);
    params
}

pub fn load_texture(
    filepath: &str,
) -> SizedTexture {
    let mut texture_id: GLuint = 0;
    let img = image::open(&Path::new(filepath)).expect("unable to open file");
    let img_rgba = match img {
        DynamicImage::ImageRgba8(rgba_img) => rgba_img,
        _ => img.to_rgba8(),
    };
    let width = img_rgba.width() as i32;
    let height = img_rgba.height() as i32;
    let data = img_rgba.into_raw();

    let params = get_default_texture_params();

    unsafe {
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        for (&pname, &pval) in &params {
            gl::TexParameteri(gl::TEXTURE_2D, pname, pval as GLint);
        }
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as GLint,
            width,
            height,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const GLvoid,
        );
        gl::BindTexture(gl::TEXTURE_2D, 0);
    } 

    println!("loaded texture");
    SizedTexture {
        tex: texture_id,
        width,
        height,
    }
}

#[derive(Debug)]
pub struct TextureBasket {
    textures: Vec<SizedTexture>,
    fps: f64,
    t: f64,
}

impl TextureBasket {
    pub fn new(textures: Vec<SizedTexture>, fps: f64) -> Self {
        TextureBasket {
            textures,
            fps,
            t: 0.0,
        }
    }
    pub fn frame(&self) -> usize {
        if self.textures.is_empty() {
            return 0;
        }
        (self.t * self.fps).floor() as usize % self.textures.len()
    }
    pub fn texture(&self) -> Option<&SizedTexture> {
        if self.textures.is_empty() {
            None
        } else {
            Some(&self.textures[self.frame()])
        }
    }

    pub fn update(&mut self, delta: f64) {
        self.t += delta;
    }
    pub fn clean_up(&self) {
        unsafe {
            for texture in &self.textures {
                gl::DeleteTextures(1, &texture.tex);
            }
        }
    }
}
