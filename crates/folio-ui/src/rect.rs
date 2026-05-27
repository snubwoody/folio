use crate::color::{Color, Rgba};
use crate::map;
use std::sync::atomic::{AtomicU32, Ordering};
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};

// TODO:
// - Animate between:
//   - Colors
static NODE_ID_COUNT: AtomicU32 = AtomicU32::new(0);

pub fn create_node_id() -> u32 {
    NODE_ID_COUNT.fetch_add(1, Ordering::Relaxed)
}

fn lerp(a: f32, b: f32, f: f32) -> f32 {
    a * (1.0 - f) + (b * f)
}

/// Animatable properties
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Property {
    Position(f32, f32),
    Size(f32, f32),
    CornerRadius(f32),
}

#[derive(Debug, Clone, Copy)]
pub struct Keyframe {
    node_id: u32,
    start_frame: u32,
    end_frame: u32,
    /// The property to animate
    property: Property,
}

impl Keyframe {
    pub fn new(node_id: u32, start_frame: u32, end_frame: u32, property: Property) -> Keyframe {
        Keyframe {
            node_id,
            start_frame,
            end_frame,
            property,
        }
    }

    pub fn node_id(&self) -> u32 {
        self.node_id
    }

    pub fn end_frame(&self) -> u32 {
        self.end_frame
    }

    pub fn start_frame(&self) -> u32 {
        self.start_frame
    }
}

#[derive(Default, Debug)]
#[derive(Clone)]
pub struct Rect {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub corner_radius: f32,
    pub color: Color<Rgba>,
}

impl Rect {
    pub fn new() -> Rect {
        Self {
            id: create_node_id(),
            ..Default::default()
        }
    }

    pub fn interpolate(&mut self, current_frame: u32, keyframe: &Keyframe) {
        // FIXME: this is kind of off, store original value
        let frame_offset = (current_frame - keyframe.start_frame) as f32;
        let frame_diff = (frame_offset / keyframe.end_frame as f32).min(1.0);
        match &keyframe.property {
            Property::Size(width, height) => {
                self.width = lerp(self.width, *width, frame_diff);
                self.height = lerp(self.height, *height, frame_diff);
            }
            Property::Position(x, y) => {
                self.x = lerp(self.x, *x, frame_diff);
                self.y = lerp(self.y, *y, frame_diff);
            }
            _ => {}
        }
    }

    pub fn draw(&self, pixmap: &mut Pixmap) {
        if self.width == 0.0 || self.height == 0.0 {
            return;
        }

        let (r, g, b, a) = self.color.inner();

        // Map the alpha since it's clipped to 100
        let a = map(a as f32, [0.0, 100.0], [0.0, 255.0]) as u8;
        let mut paint = Paint::default();
        paint.set_color_rgba8(r, g, b, a);

        let x = self.x;
        let y = self.y;
        let width = self.width;
        let height = self.height;

        // TODO: clamp radius
        let radius = self.corner_radius;

        // Construct a rounded rect going clockwise
        let mut pb = PathBuilder::new();

        // Top left corner
        pb.move_to(x + radius, y);
        // Top edge
        pb.line_to(x + width - radius, y);
        // Top right corner
        pb.quad_to(x + width, y, x + width, y + radius);
        // Right edge
        pb.line_to(x + width, y + height - radius);
        // Bottom right corner
        pb.quad_to(x + width, y + height, x + width - radius, y + height);
        // Bottom edge
        pb.line_to(x + radius, y + height);
        // Bottom left corner
        pb.quad_to(x, y + height, x, y + height - radius);
        // Left edge
        pb.line_to(x, y + radius);
        // Top left corner
        pb.quad_to(x, y, x + radius, y);


        let path = pb.finish().unwrap();
        pixmap.fill_path(
            &path,
            &paint,
            FillRule::Winding,
            Transform::identity(),
            None,
        );

        // Will implement borders soon enough

        // if let Some(border) = &self.border {
        //     // TODO turn this into a function
        //     let (r, g, b, a) = border.color.inner();
        //     let a = map(a as f32, [0.0, 100.0], [0.0, 255.0]) as u8;

        //     let mut border_paint = Paint::default();
        //     border_paint.set_color_rgba8(r, g, b, a);
        //     let mut path_builder = PathBuilder::new();
        //     path_builder.push_rect(rect);
        //     let path = path_builder.finish().unwrap();

        //     let stroke = Stroke {
        //         width: border.width,
        //         ..Default::default()
        //     };

        //     pixmap.stroke_path(&path, &border_paint, &stroke, Transform::identity(), None);
        // }
    }
}

#[cfg(test)]
mod test{
    use crate::color::IntoColor;
    use super::*;

    #[test]
    fn size() {
        let mut pixmap = Pixmap::new(100, 100).unwrap();
        pixmap.fill(tiny_skia::Color::WHITE);

        let mut rect = Rect::new();
        rect.width = 50.0;
        rect.height = 20.0;
        rect.color = Color::rgb(12,144,240);
        rect.draw(&mut pixmap);

        for x in 0..100 {
            for y in 0..100 {
                let pixel = pixmap.pixel(x, y).unwrap();
                let r = pixel.red();
                let g = pixel.green();
                let b = pixel.blue();

                if x < 50 && y < 20 {
                    assert_eq!((r, g, b), (12, 144, 240));
                } else {
                    assert_eq!((r, g, b), (255, 255, 255));
                }
            }
        }
    }

    #[test]
    fn background_color() {
        let mut pixmap = Pixmap::new(100, 100).unwrap();
        pixmap.fill(tiny_skia::Color::WHITE);
        let mut rect = Rect::new();
        rect.width = 100.0;
        rect.height = 100.0;
        rect.color = 150.into_color();
        rect.draw(&mut pixmap);
        for pixel in pixmap.pixels() {
            assert_eq!(pixel.red(), 150);
            assert_eq!(pixel.green(), 150);
            assert_eq!(pixel.blue(), 150);
        }
    }

    #[test]
    fn rect_interpolate_size_halfway(){
        let mut rect = Rect::new();
        let keyframe = Keyframe::new(rect.id,0,10,Property::Size(100.0,50.0));
        rect.interpolate(5,&keyframe);
        assert_eq!(rect.width,50.0);
        assert_eq!(rect.height,25.0);
    }

    #[test]
    fn rect_interpolate_position_halfway(){
        let mut rect = Rect::new();
        let keyframe = Keyframe::new(rect.id,0,10,Property::Position(100.0,50.0));
        rect.interpolate(5,&keyframe);
        assert_eq!(rect.x,50.0);
        assert_eq!(rect.y,25.0);
    }
}
