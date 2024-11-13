use super::{
    actor_pose::ActorPose,
    animation::AnimationFrames,
    pose::{BOTTOM, LEFT, RIGHT, TOP},
    primitives::Point,
};

pub fn squatting_animation() -> AnimationFrames {
    let standing_pose = ActorPose::new(
        Point::new(0.0, 8.0),
        vec![
            (RIGHT).into(),
            ((BOTTOM + RIGHT) / 2.0).into(),
            ((BOTTOM + RIGHT) / 2.0).into(),
            (BOTTOM).into(),
            (LEFT).into(),
            (BOTTOM).into(),
            (LEFT).into(),
            (BOTTOM).into(),
        ],
    );
    let squatting_pose = ActorPose::new(
        Point::new(0.0, -8.0),
        vec![
            (RIGHT + 10.0).into(),
            (BOTTOM).into(),
            (RIGHT + 10.0).into(),
            (BOTTOM).into(),
            (LEFT - 10.0).into(),
            (10.0).into(),
            (LEFT - 10.0).into(),
            (10.0).into(),
        ],
    );

    let mut animation = ActorPose::interpolate(&standing_pose, &squatting_pose, 4);
    animation.append(&mut ActorPose::interpolate(
        &squatting_pose,
        &standing_pose,
        8,
    ));

    AnimationFrames::new(animation)
}
