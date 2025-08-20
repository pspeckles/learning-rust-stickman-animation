use super::{
    geometry::figure_end_x,
    graph::Node,
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

    pub fn with_angle(self, angle: Rotation) -> Self {
        Self { angle, ..self }
    }

    pub fn with_point(self, point: Point) -> Self {
        Self { point, ..self }
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

pub trait PositionNode {
    fn set_point(&mut self, point: Point);
    fn set_angle(&mut self, angle: Rotation);
}

impl PositionNode for Node<PositionData> {
    fn set_point(&mut self, point: Point) {
        let v = *self.get();
        self.set(v.with_point(point));
    }

    fn set_angle(&mut self, angle: Rotation) {
        let v = *self.get();
        self.set(v.with_angle(angle));
    }
}
