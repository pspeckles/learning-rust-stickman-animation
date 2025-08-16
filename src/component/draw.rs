use super::position::PositionData;

pub trait DrawComponent {
    fn get_drawables(&self) -> Vec<Drawable>;
}

pub struct Drawable {
    pub position: PositionData,
    pub shape: Shape,
}

pub enum Shape {
    Square,
}
