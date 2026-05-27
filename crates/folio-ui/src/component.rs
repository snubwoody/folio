use cascada::{Layout, VerticalLayout};
use tiny_skia::Pixmap;
use crate::rect::Rect;
use crate::color::{Color, IntoColor, Rgba};

pub trait Component{
    fn id(&self) -> u32;

    /// Draws the component to the screen.
    fn draw(&self, pixmap: &mut Pixmap, layout: &dyn Layout);
    fn tick(&mut self, ){
        
    }
}

// TODO: use messages like SelectNode, DeselectNode
pub struct Column {
    id: u32,
    background_color: Color
}

impl Column {
    pub fn new() -> Column{
        Column{
            id: 0,
            background_color: Color::TRANSPARENT
        }
    }

    /// Sets the widget id.
    pub fn with_id(mut self, id: u32) -> Column{
        self.id = id;
        self
    }

    /// Sets the background [`Color`].
    pub fn background_color(mut self, color: impl IntoColor<Rgba>) -> Column{
        self.background_color = color.into_color();
        self
    }
    
    pub fn layout(&mut self) -> Box<dyn Layout>{
        let layout = VerticalLayout::new();
        Box::new(layout)
    }
}

impl Component for Column{
    fn id(&self) -> u32 {
        self.id
    }

    fn draw(&self, pixmap: &mut Pixmap, layout: &dyn Layout){
        let mut rect = Rect::new();
        rect.x = layout.position().x;
        rect.y = layout.position().y;
        rect.color = self.background_color.clone();
        rect.width = layout.size().width;
        rect.height = layout.size().height;
        rect.draw(pixmap);
    }
}
