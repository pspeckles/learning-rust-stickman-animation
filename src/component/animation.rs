use crate::actor::animation::AnimationFrame;

#[derive(Clone, Debug)]
pub struct AnimationState {
    last_frame: Box<AnimationFrame>,
    last_frame_time: u128,
    next_key_frame: usize,
}

impl AnimationState {
    pub fn new(last_frame: Box<AnimationFrame>) -> Self {
        AnimationState {
            last_frame,
            last_frame_time: 0,
            next_key_frame: 0,
        }
    }

    pub fn last_frame(&self) -> Box<AnimationFrame> {
        self.last_frame.clone()
    }

    pub fn last_frame_time(&self) -> &u128 {
        &self.last_frame_time
    }

    pub fn next_key_frame(&self) -> &usize {
        &self.next_key_frame
    }

    pub fn set_last_frame(&mut self, frame: Box<AnimationFrame>) {
        self.last_frame = frame.clone();
    }

    pub fn set_last_frame_time(&mut self, t: u128) {
        self.last_frame_time = t;
    }

    pub fn set_next_key_frame(&mut self, frame: usize) {
        self.next_key_frame = frame;
    }
}
