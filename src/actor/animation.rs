use super::actor_pose::ActorPose;
//
// pub trait Animation {
//     fn set_animation(&mut self, num: usize);
//     fn next_frame(&mut self);
// }

#[derive(Debug, Clone)]
pub struct AnimationFrames {
    frames: Vec<ActorPose>,
}

impl AnimationFrames {
    pub fn new(frames: Vec<ActorPose>) -> AnimationFrames {
        AnimationFrames { frames }
    }

    pub fn next_frame(&self, num: &usize) -> (&ActorPose, usize) {
        if *num >= self.frames.len() {
            (&self.frames[0], 0)
        } else {
            (&self.frames[*num], num + 1)
        }
    }
}
