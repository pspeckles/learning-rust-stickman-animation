use sfml::graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::system::Vector2f;

use super::primitives::Rotation;

#[derive(Debug)]
pub struct Flag {
    pole: RectangleShape<'static>,
    cloth: RectangleShape<'static>,
    cloth_far: RectangleShape<'static>,
}

impl Flag {
    pub fn new(length: f32, width: f32) -> Self {
        let mut cloth = RectangleShape::with_size(Vector2f::new(length / 1.5, width / 2.0));
        cloth.set_fill_color(Color::YELLOW);
        Flag {
            pole: RectangleShape::with_size(Vector2f::new(length, 5.0)),
            cloth,
            cloth_far: RectangleShape::with_size(Vector2f::new(length / 1.6, width / 2.0)),
        }
    }

    pub fn draw(&self, window: &mut RenderWindow) {
        window.draw(&self.pole);
        window.draw(&self.cloth);
        window.draw(&self.cloth_far);
    }
}

pub struct FlagPosition {
    rotation: Rotation,
}

impl FlagPosition {
    pub fn new(rotation: f32) -> Self {
        FlagPosition {
            rotation: Rotation { r: rotation },
        }
    }
}

// impl Position for FlagPosition {
//     type Positionable = Flag;
//
//     fn apply_to(&self, object: &mut Self::Positionable, point: &Point) {
//         object.pole.set_position((point.x(), point.y()));
//         object.pole.set_rotation(self.rotation.r);
//         object.cloth.set_rotation(self.rotation.r);
//         let cloth_start = figure_end_y(
//             object.pole.size().x,
//             (object.pole.position().x, object.pole.position().y),
//             object.pole.rotation(),
//         );
//         object.cloth.set_position((cloth_start.0, cloth_start.1));
//         let cloth_far_start = figure_end_x(
//             object.cloth.size().y,
//             (object.cloth.position().x, object.cloth.position().y),
//             object.cloth.rotation(),
//         );
//         let shift = if object.cloth_far.position().y >= cloth_far_start.1 {
//             object.cloth_far.size().y * 0.05
//         } else {
//             object.cloth_far.size().y * -0.05
//         };
//         object
//             .cloth_far
//             .set_position((cloth_far_start.0, cloth_far_start.1 + shift));
//         object.cloth_far.set_rotation(self.rotation.r);
//     }
// }
