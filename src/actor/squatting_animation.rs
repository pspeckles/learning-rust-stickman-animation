use super::{
    actor_pose::ActorPose,
    animation::AnimationFrames,
    pose::{BOTTOM, LEFT, QUARTER_ANTI_CLOCKWISE, QUARTER_CLOCKWISE, RIGHT, TOP, TOP_N},
    primitives::Point,
};

pub fn squatting_animation() -> AnimationFrames {
    let standing_pose = ActorPose::new(
        Point::new(0.0, -30.0),
        vec![
            (RIGHT + QUARTER_CLOCKWISE / 3.0).into(),
            (BOTTOM).into(),
            (RIGHT + QUARTER_CLOCKWISE / 3.0).into(),
            (BOTTOM).into(),
            (LEFT + QUARTER_ANTI_CLOCKWISE / 3.0).into(),
            (BOTTOM).into(),
            (LEFT + QUARTER_ANTI_CLOCKWISE / 3.0).into(),
            (BOTTOM).into(),
        ],
    );
    let squatting_pose = ActorPose::new(
        Point::new(0.0, 30.0),
        vec![
            (RIGHT + QUARTER_ANTI_CLOCKWISE / 3.0).into(),
            (TOP_N).into(),
            (RIGHT + QUARTER_ANTI_CLOCKWISE / 3.0).into(),
            (BOTTOM).into(),
            (LEFT + QUARTER_CLOCKWISE / 3.0).into(),
            (BOTTOM).into(),
            (LEFT + QUARTER_CLOCKWISE / 3.0).into(),
            (TOP).into(),
        ],
    );

    let mut animation = ActorPose::interpolate(&standing_pose, &squatting_pose, 5);
    animation.append(&mut ActorPose::interpolate(
        &squatting_pose,
        &standing_pose,
        5,
    ));

    AnimationFrames::new(animation)
}
