use super::{
    geometry::figure_end_x,
    primitives::{Point, Rotation},
};

#[derive(Clone, Copy, Debug)]
pub struct PositionData {
    pub point: Point,
    pub angle: Rotation,
    pub width: u16,
    pub height: u16,
    pub z: i32,
}

impl PositionData {
    pub fn new(point: Point, angle: Rotation, width: u16, height: u16, z: i32) -> Self {
        PositionData {
            point,
            angle,
            width,
            height,
            z,
        }
    }

    /// calculates the "end" of the current position,
    /// based on inner state.
    /// The result, simplified, the other, from point view line coordinate.
    pub fn middle_x_end(&self) -> Point {
        figure_end_x(
            self.height as f32,
            (self.point.x(), self.point.y()),
            self.angle.r,
        )
        .into()
    }
}
