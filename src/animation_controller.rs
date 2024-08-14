use crate::stickman::animation::Animation;
use crate::stickman::pose::Point;
use crate::stickman::position::Position;
use crate::stickman::skeleton::Skeleton;

#[derive(Debug)]
pub struct AnimationController<T: Position> {
    animation: Animation<T>,
}

impl<T: Position> AnimationController<T> {
    pub fn new(init_animation: Animation<T>) -> Self {
        AnimationController {
            animation: init_animation,
        }
    }

    pub fn set_animation(&mut self, animation: Animation<T>) {
        self.animation = animation;
    }

    pub fn animate(&mut self, obj: &mut T::Positionable, point: &Point) {
        let pose = self.animation.next_frame();
        pose.apply_to(obj, point);
    }

    pub fn stop_animation(&self) -> bool {
        // self.animation.reset();
        true
    }
}
