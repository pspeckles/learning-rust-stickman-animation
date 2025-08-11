use super::animation::AnimationFrame;

use crate::component::primitives::{Point, Rotation};

use super::animation::AnimationFrames;

pub fn standing_animation() -> AnimationFrames {
    // let standing_pose_1 = &AnimationFrame::new(
    //     Point::new(0.0, 3.0),
    //     vec![
    //         Rotation::new(70.0),
    //         Rotation::new(30.0),
    //         Rotation::new(-20.0),
    //         Rotation::new(-30.0),
    //         Rotation::new(130.0),
    //         Rotation::new(70.0),
    //         Rotation::new(150.0),
    //         Rotation::new(50.0),
    //     ],
    // );
    // let standing_pose_2 = &AnimationFrame::new(
    //     Point::new(0.0, -3.0),
    //     vec![
    //         Rotation::new(60.0),
    //         Rotation::new(40.0),
    //         Rotation::new(-20.0),
    //         Rotation::new(-35.0),
    //         Rotation::new(120.0),
    //         Rotation::new(80.0),
    //         Rotation::new(160.0),
    //         Rotation::new(40.0),
    //     ],
    // );
    // let standing_animation =
    //     AnimationFrame::interpolate(&vec![standing_pose_1, standing_pose_2, standing_pose_1], 4);
    // AnimationFrames::new(standing_animation)
}
