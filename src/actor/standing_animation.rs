use super::actor_pose::ActorPose;

use super::primitives::{Point, Rotation};

use super::animation::AnimationFrames;

pub fn standing_animation() -> AnimationFrames {
    let standing_pose_1 = ActorPose::new(
        Point::new(0.0, 3.0),
        vec![
            Rotation::new(70.0),
            Rotation::new(30.0),
            Rotation::new(-20.0),
            Rotation::new(-30.0),
            Rotation::new(130.0),
            Rotation::new(70.0),
            Rotation::new(150.0),
            Rotation::new(50.0),
        ],
    );
    let standing_pose_2 = ActorPose::new(
        Point::new(0.0, -3.0),
        vec![
            Rotation::new(60.0),
            Rotation::new(40.0),
            Rotation::new(-20.0),
            Rotation::new(-35.0),
            Rotation::new(120.0),
            Rotation::new(80.0),
            Rotation::new(160.0),
            Rotation::new(40.0),
        ],
    );
    let mut standing_animation = ActorPose::interpolate(&standing_pose_1, &standing_pose_2, 3);
    standing_animation.append(&mut ActorPose::interpolate(
        &standing_pose_2,
        &standing_pose_1,
        8,
    ));
    AnimationFrames::new(standing_animation)
}
