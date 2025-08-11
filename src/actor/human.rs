use crate::component::animation::AnimationState;
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
