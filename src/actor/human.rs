use std::vec;

use sfml::graphics::{Color, Drawable, RectangleShape, Shape, Transformable};
use sfml::system::Vector2;

use crate::component::animation::{AnimationComponent, AnimationState};
use crate::component::draw::DrawComponent;
use crate::component::position::PositionData;

use super::t_pose::t_pose;

pub struct Human {
    pub animation: AnimationState,
}

impl Human {
    pub fn new() -> Self {
        Human {
            animation: AnimationState::new(Box::new(t_pose())),
        }
    }
}

impl AnimationComponent for Human {
    fn get_animation_state_mut(&mut self) -> &mut AnimationState {
        &mut self.animation
    }
}

impl DrawComponent for Human {
    fn get_drawables(&self) -> Vec<Box<dyn Drawable>> {
        let mut drawables = vec![];
        let current_animation = &self.animation.current();
        let entries = current_animation.pose.entries();
        // Build stable-sorted order by z-index: left (-1), torso/head/neck (0), right (+1)
        let mut order: Vec<usize> = (0..entries.len()).collect();
        order.sort_by_key(|i| entries[*i].get().z);
        for i in order {
            let position = entries[i].get();
            let texture = to_texture(position);
            let bone = to_joint(position);
            drawables.push(texture);
            drawables.push(bone);
        }

        drawables
    }
}

fn to_texture(position: &PositionData) -> Box<dyn Drawable> {
    let mut rect = RectangleShape::new();
    rect.set_size(Vector2::new(position.width as f32, position.height as f32));
    rect.set_position(Vector2::new(
        position.point.x() - (position.width as f32 * (position.angle.r.to_radians()).cos()) / 2.0,
        position.point.y() - (position.width as f32 * (position.angle.r.to_radians()).sin()) / 2.0,
    ));
    rect.set_outline_color(Color::BLUE);
    rect.set_outline_thickness(1.0);
    rect.set_rotation(position.angle.r);
    Box::new(rect)
}
fn to_joint(position: &PositionData) -> Box<dyn Drawable> {
    let mut joint = RectangleShape::with_size(Vector2 {
        x: 2.0,
        y: position.height as f32,
    });
    joint.set_position(Vector2::new(position.point.x(), position.point.y()));
    joint.set_rotation(position.angle.r);
    joint.set_fill_color(Color::RED);
    Box::new(joint)
}
