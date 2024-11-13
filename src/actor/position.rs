use std::cell::RefMut;

use super::{actor_pose::ActorPose, skeleton::RenderingParts};

pub trait Position {
    fn into_pose(&mut self, pose: &ActorPose);
}
