use std::cell::Cell;

use crate::actor::{actor_pose::ActorPose, animation::AnimationFrames, position::Position};

#[derive(Debug, Clone)]
pub struct AnimationController {
    animation: AnimationFrames,
    frame_num: Cell<usize>,
}

impl<'a> AnimationController {
    pub fn new(init_animation: AnimationFrames) -> Self {
        AnimationController {
            animation: init_animation,
            frame_num: Cell::new(0),
        }
    }

    pub fn animate(&self) -> &ActorPose {
        let (frame, next_frame_num) = self.animation.next_frame(&self.frame_num.get());
        self.frame_num.set(next_frame_num);
        frame
    }

    pub fn stop_animation(&mut self) {
        self.frame_num.set(0);
    }
}
