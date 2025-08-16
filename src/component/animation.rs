use crate::actor::animation::AnimationFrame;

pub trait AnimationComponent {
    fn get_animation_state_mut(&mut self) -> &mut AnimationState;
}

#[derive(Clone, Debug)]
pub struct AnimationState {
    current_frame: Box<AnimationFrame>,
    current_frame_time: u128,
    next_key_frame: usize,
    current_key_frame: usize,
}

impl AnimationState {
    pub fn new(current_frame: Box<AnimationFrame>) -> Self {
        AnimationState {
            current_frame,
            current_frame_time: 0,
            next_key_frame: 0,
            current_key_frame: 0,
        }
    }

    pub fn current(&self) -> Box<AnimationFrame> {
        self.current_frame.clone()
    }

    pub fn set_current(&mut self, frame: Box<AnimationFrame>) {
        self.current_frame = frame.clone();
    }

    pub fn set_current_frame_time(&mut self, t: u128) {
        self.current_frame_time = t;
    }

    pub fn current_frame_time(&self) -> &u128 {
        &self.current_frame_time
    }

    pub fn next_key_frame(&self) -> usize {
        self.next_key_frame
    }

    pub fn set_next_key_frame(&mut self, frame: usize) {
        self.next_key_frame = frame;
    }

    pub fn set_current_key_frame(&mut self, frame: usize) {
        self.current_key_frame = frame;
    }

    pub fn current_key_frame(&self) -> usize {
        self.current_key_frame
    }
}
