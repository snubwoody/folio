use cascada::{solve_layout, AxisAlignment, GlobalId, IntrinsicSize, Layout, Size, VerticalLayout};
use tiny_skia::{Pixmap, PixmapPaint, Transform};
use crate::rect::{Keyframe, Property, Rect};
use crate::color::{Color, IntoColor};
use crate::component::{Column, Component};

/// Animation renderer and timeline.
#[derive(Debug)]
pub struct Canvas {
    id: u32,
    shapes: Vec<Rect>,
    frame_time: u32,
    keyframes: Vec<Keyframe>,
    start_frame: u32,
    end_frame: u32,
}

// TODO: get time delta

impl Canvas {
    pub fn new() -> Self {
        // TODO: inner pixmap
        let mut shapes = vec![];
        let mut rect = Rect::new();
        let mut keyframes = vec![];

        rect.width = 200.0;
        rect.height = 200.0;
        rect.color = Color::rgb(224, 114, 224);

        // TODO: add keyframe method on rect

        keyframes.push(Keyframe::new(
            rect.id,
            0,
            100,
            Property::Position(50.0, 50.0),
        ));
        keyframes.push(Keyframe::new(
            rect.id,
            100,
            200,
            Property::Position(150.0, 0.0),
        ));
        keyframes.push(Keyframe::new(
            rect.id,
            200,
            300,
            Property::Position(250.0, 50.0),
        ));

        shapes.push(rect);

        Self {
            id: 0,
            shapes,
            frame_time: 0,
            keyframes,
            start_frame: 1,
            end_frame: 250,
        }
    }

    /// Sets the canvas id.
    pub fn with_id(mut self, id: u32) -> Self{
        self.id = id;
        self
    }


    /// Draw the timeline to the screen.
    fn draw_timeline(&mut self,) {
        // TODO: align bottom
        let mut timeline_layout = cascada::VerticalLayout::new()
            .with_label("Timeline")
            .intrinsic_size(cascada::IntrinsicSize{
                width: cascada::BoxSizing::Flex(1),
                height: cascada::BoxSizing::Fixed(50.0),
            });

        let mut root_layout = VerticalLayout::new()
            .with_label("root")
            .intrinsic_size(IntrinsicSize::fill())
            .main_axis_alignment(AxisAlignment::End)
            .add_child(timeline_layout);

        // solve_layout(&mut root_layout,Size::new(self.pixmap.width() as f32,self.pixmap.height() as f32));

        let timeline_layout = &root_layout.children()[0];

        // TODO: controllable cursor
        let mut timeline = Rect::new();
        timeline.id = timeline_layout.id().inner();
        timeline.width = timeline_layout.size().width;
        timeline.height = timeline_layout.size().height;
        timeline.color = 50.into_color();
        timeline.x = timeline_layout.position().x;
        // FIXME: layout bug
        timeline.y = timeline_layout.position().y - 50.0;

        let mut cursor = Rect::new();
        cursor.height = timeline.height;
        cursor.width = 5.0;
        cursor.x = (timeline.width * (self.frame_time as f32 / self.end_frame as f32))
            .min(timeline.width - cursor.width);
        cursor.y = timeline.y;
        cursor.color = 100.into_color();

        // timeline.draw(&mut self.pixmap);
        // cursor.draw(&mut self.pixmap);
    }

    pub fn animate(&mut self) {
        if self.frame_time > self.end_frame {
            return;
        }

        for shape in &mut self.shapes {
            let shape_id = shape.id;
            let keyframes = self
                .keyframes
                .iter()
                .filter(|k| k.start_frame() < self.frame_time)
                .filter(|k| k.end_frame() > self.frame_time) // Filter completed keyframes
                .filter(|k| k.node_id() == shape_id);

            for keyframe in keyframes {
                shape.interpolate(self.frame_time, keyframe);
                // keyframe.interpolate(self.frame_time,&mut *shape);
            }
            // shape.animate(self.frame_time);
        }
        self.frame_time += 1;
    }
}

impl Component for Canvas{
    fn id(&self) -> u32 {
        self.id
    }

    fn tick(&mut self) {
        self.animate();
    }

    fn draw(&self, pixmap: &mut Pixmap, layout: &dyn Layout) {
        // TODO: remove dbg! in cascada and fixed layout bug
        for shape in &self.shapes {
            let mut shape = shape.clone();
            shape.x += layout.position().x;
            shape.y += layout.position().y;
            shape.draw(pixmap);
        }
    }
}