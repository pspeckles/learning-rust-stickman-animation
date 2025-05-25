use super::actor_pose::ActorPose;

pub trait Position {
    fn set_position(&mut self, pose: &ActorPose);
}
