use cascada::{solve_layout, BoxSizing, EmptyLayout, HorizontalLayout, IntrinsicSize, Layout, Padding, Size, VerticalLayout};
use pixels::Pixels;
use tiny_skia::Pixmap;
use crate::component::{Column, Component};
use crate::canvas::Canvas;

pub struct Ui {
    pixmap: Pixmap,
    canvas: Canvas,
    layout: Box<dyn Layout>,
    components: Vec<Box<dyn Component>>
}

impl Ui {
    pub fn new() -> Ui {
        let pixmap = Pixmap::new(250, 250).unwrap();
        let renderer = Canvas::new();

        let node_layout = EmptyLayout::new()
            .intrinsic_size(
                IntrinsicSize{
                    width: BoxSizing::Flex(1),
                    height: BoxSizing::Fixed(50.0)
                }
            );

        let node = Column::new()
            .with_id(node_layout.id().inner())
            .background_color(35);

        let node_panel_layout = VerticalLayout::new()
            .padding(Padding::all(12.0))
            .intrinsic_size(IntrinsicSize::fill())
            .add_child(node_layout)
            .max_width(350.0);

        let node_panel = Column::new()
            .with_id(node_panel_layout.id().inner())
            .background_color(30);


        let canvas_layout = EmptyLayout::new()
            .intrinsic_size(IntrinsicSize::fill());

        let canvas = Canvas::new()
            .with_id(canvas_layout.id().inner());

        let root_layout = HorizontalLayout::new()
            .intrinsic_size(IntrinsicSize::fill())
            .add_child(node_panel_layout)
            .add_child(canvas_layout);

        let components: Vec<Box<dyn Component>> = vec![
            Box::new(node_panel),
            Box::new(node),
            Box::new(canvas)
        ];

        Ui {
            pixmap,
            canvas: renderer,
            layout: Box::new(root_layout),
            components
        }
    }

    fn add_component(&mut self, f: impl FnOnce(EmptyLayout,Column) -> (EmptyLayout,Column)){
        let (layout,column) = f(EmptyLayout::new(),Column::new());
        let column = column.with_id(layout.id().inner());
    }

    pub fn column(&mut self){
        let node_layout = EmptyLayout::new()
            .intrinsic_size(
                IntrinsicSize{
                    width: BoxSizing::Flex(1),
                    height: BoxSizing::Fixed(50.0)
                }
            );

        let node = Column::new()
            .with_id(node_layout.id().inner())
            .background_color(35);

        let node_panel_layout = VerticalLayout::new()
            .padding(Padding::all(12.0))
            .intrinsic_size(IntrinsicSize::fill())
            .add_child(node_layout)
            .max_width(350.0);

        let node_panel = Column::new()
            .with_id(node_panel_layout.id().inner())
            .background_color(30);


        let root_layout = HorizontalLayout::new()
            .intrinsic_size(IntrinsicSize::fill())
            .add_child(node_panel_layout);

        let components: Vec<Box<dyn Component>> = vec![
            Box::new(node_panel),
            Box::new(node),
        ];
    }

    /// Resizes the `Pixmap`. Does nothing if the `width` or `height` is 0.
    pub fn resize(&mut self, width: u32, height: u32){
        if width > 0 && height > 0 {
            self.pixmap = Pixmap::new(width, height).unwrap();
        }
        // self.renderer.resize(width, height);
    }

    // The tick function represents a single frame
    pub fn tick(&mut self){
        for component in &mut self.components{
            component.tick();
        }
    }

    /// Draw the UI to the screen
    pub fn draw(&mut self, buffer: &mut Pixels,width: u32, height: u32){
        self.pixmap.fill(tiny_skia::Color::WHITE);
        // self.canvas.draw(&mut self.pixmap, &EmptyLayout::new());

        solve_layout(self.layout.as_mut(),Size::new(width as f32, height as f32));

        for component in &self.components{
            let layout = self.layout.iter().find(|l|l.id().inner() == component.id());

            if let Some(component_layout) = layout{
                component.draw(&mut self.pixmap,component_layout);
            }
        }

        buffer
            .frame_mut()
            .copy_from_slice(&self.pixmap.data());
        buffer.render().unwrap();
    }
}


use crate::app::{App};
use tracing::info;
use tracing_subscriber::EnvFilter;

mod rect;
mod app;
mod color;
mod canvas;
mod component;
mod ui;

/// Map value from one range to another. Any overflow or underflow is clipped to the min or max
///
/// # Example
/// ```
/// use agape_core::map;
/// let mapped_half = map(5.0,[0.0,10.0],[10.0,20.0]);
/// assert_eq!(mapped_half,15.0);
/// ```
pub fn map(mut value: f32, input_range: [f32; 2], output_range: [f32; 2]) -> f32 {
    if value > input_range[1] {
        value = input_range[1]
    } else if value < input_range[0] {
        value = input_range[0]
    }

    let scale = (output_range[1] - output_range[0]) / (input_range[1] - input_range[0]);
    let offset = input_range[0] * (scale) + output_range[0];

    value * scale + offset
}

fn main() {
    unsafe {
        std::env::set_var("RUST_LOG","warn,motion_graphics=trace")
    }
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let app = App::new();
    app.run();
    info!("Exiting app");
}

