pub mod renderer;
pub mod world;
pub mod input_manager;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use renderer::engine::GraphicsEngine;
use world::World;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("soft-render")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let g = GraphicsEngine::new(pixels.get_frame());
    let mut world = World::new(g);
    
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event: window_event, .. } => 
                match window_event {
                    WindowEvent::Resized(size) => {
                        pixels.resize_surface(size.width, size.height);
                    },
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    },
                    e => 
                        world.input_manager.process(&e)
                    
                }
            ,
            Event::MainEventsCleared => {
                world.update();

                world.render();
                pixels.render().unwrap();
            },
            _ => {}
        }
    });
}