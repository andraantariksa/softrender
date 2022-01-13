use super::triangle::Triangle3D;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

pub struct GraphicsEngine<'a> {
    framebuffer: &'a mut [u8]
}

impl<'a> GraphicsEngine<'a> {
    pub fn new(framebuffer: &'a mut [u8]) -> Self {
        Self {
            framebuffer
        }
    }

    // fn set_pixel_and_depth(&self, color: Color, depth: f32) {

    // }

    fn rasterize_triangle(&mut self, triangle: Triangle3D<f32>) {

    }
}
