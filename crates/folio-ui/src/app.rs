use std::sync::Arc;
use std::time::{Duration, Instant};
use cascada::{solve_layout, BoxSizing, EmptyLayout, IntrinsicSize, Layout, Padding, Size, VerticalLayout};
use pixels::{Pixels, SurfaceTexture,};
use tracing::{debug, info, trace};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

use crate::color::{Color, IntoColor, Rgba};
use crate::ui::Ui;

pub struct App<'a> {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'a>>,
    ui: Ui,
    frame_count: u64,
    last_frame: Instant,
    fps_timer: Instant
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        // This function is always safe to call on Windows
        #[cfg(windows)]
        unsafe {
            std::env::set_var("WGPU_BACKEND","gl");
        }

        App {
            window: None,
            pixels: None,
            ui: Ui::new(),
            frame_count: 0,
            last_frame: Instant::now(),
            fps_timer: Instant::now()
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        let width = width.max(1);
        let height = height.max(1);

        self.pixels
            .as_mut()
            .unwrap()
            .resize_surface(width, height)
            .expect("Failed to resize the pixel buffer");

        self.pixels
            .as_mut()
            .unwrap()
            .resize_buffer(width, height)
            .expect("Failed to resize the pixel buffer");

        self.ui.resize(width,height);
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self).unwrap();
    }

    fn render(&mut self){
        let pixels = self.pixels.as_mut().unwrap();
        let width = self.window.as_ref().unwrap().inner_size().width;
        let height = self.window.as_ref().unwrap().inner_size().height;
        self.ui.draw(pixels,width,height);
    }
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = WindowAttributes::default();
        let window = event_loop.create_window(attrs).unwrap();
        let window = Arc::new(window);

        let width = window.inner_size().width;
        let height = window.inner_size().height;

        let surface = SurfaceTexture::new(width, height, Arc::clone(&window));
        let pixels = Pixels::new(width, height, surface).unwrap();

        self.pixels = Some(pixels);
        self.window = Some(Arc::clone(&window));
        debug!("Created new window and pixel buffer");
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let now = Instant::now();
        // TODO: pass delta time to canvas
        let delta_time = now.duration_since(self.last_frame);
        self.last_frame = now;

        if self.fps_timer.elapsed() >= Duration::from_secs(1){

            let fps = self.frame_count;
            trace!("{fps} Fps");
            self.frame_count = 0;
            self.fps_timer = Instant::now();
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.render();
                self.window.as_mut().unwrap().request_redraw();
            }
            WindowEvent::Resized(size) => {
                self.resize(size.width, size.height);
            }
            _ => {}
        }

        self.ui.tick();
        self.frame_count += 1;
    }
}

