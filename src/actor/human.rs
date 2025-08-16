use crate::component::animation::{AnimationComponent, AnimationState};
use crate::component::draw::{DrawComponent, Drawable, Shape};
use crate::component::position::Position;

use super::t_pose::t_pose;

#[derive(Debug)]
pub struct Human {
    pub position: Position,
    pub animation: AnimationState,
}

impl Human {
    pub fn new(position: Position) -> Self {
        Human {
            position,
            animation: AnimationState::new(Box::new(t_pose())),
        }
    }
}

impl AnimationComponent for Human {
    fn get_animation_state(&self) -> &AnimationState {
        &self.animation
    }

    fn get_animation_state_mut(&mut self) -> &mut AnimationState {
        &mut self.animation
    }
}

impl DrawComponent for Human {
    fn get_drawables(&self) -> Vec<Drawable> {
        let mut drawables = vec![];
        for pose_entry in self.animation.current().pose.entries() {
            drawables.push(Drawable {
                position: *pose_entry.get(),
                shape: Shape::Square,
            })
        }

        drawables
    }
}
