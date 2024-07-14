use crate::stickman::pose::Pose;

use crate::stickman::pose::{Point, Rotation};

use crate::stickman::animation::Animation;

pub fn standing_animation() -> Animation<Pose> {
    let standing_pose_1 = Pose::new(
        Point::new(0.0, 0.0),
        Rotation::new(-20.0),
        Rotation::new(-30.0),
        Rotation::new(150.0),
        Rotation::new(50.0),
        Rotation::new(130.0),
        Rotation::new(70.0),
        Rotation::new(70.0),
        Rotation::new(30.0),
    );
    let standing_pose_2 = Pose::new(
        Point::new(0.0, 0.0),
        Rotation::new(-20.0),
        Rotation::new(-35.0),
        Rotation::new(140.0),
        Rotation::new(40.0),
        Rotation::new(120.0),
        Rotation::new(80.0),
        Rotation::new(60.0),
        Rotation::new(40.0),
    );
    let mut standing_animation = Pose::interpolate(standing_pose_1, standing_pose_2, 3);
    standing_animation.append(&mut Pose::interpolate(standing_pose_2, standing_pose_1, 3));
    Animation::new(standing_animation)
}
