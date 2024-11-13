use core::panic;

use super::primitives::{Point, Rotation};

#[derive(Debug, Clone)]
pub struct ActorPose {
    pub position_shift: Point,
    pub joints: Vec<Rotation>,
}

impl ActorPose {
    pub fn new(position: Point, joints: Vec<Rotation>) -> Self {
        ActorPose {
            position_shift: position,
            joints,
        }
    }
    pub fn interpolate(from: &ActorPose, to: &ActorPose, number_of_poses: usize) -> Vec<ActorPose> {
        //check poses are equal
        if from.joints.len() != to.joints.len() {
            println!("from {:?}, to {:?}", from, to);
            panic!("cannot interpolate different poses")
        }
        let mut poses = Vec::with_capacity(number_of_poses);
        let number_of_poses_divider: f32 = number_of_poses as f32;
        let position_shift_step = Point::from_tuple((
            (to.position_shift.x() - from.position_shift.x()) / number_of_poses_divider,
            (to.position_shift.y() - from.position_shift.y()) / number_of_poses_divider,
        ));
        let mut joints_steps = Vec::with_capacity(from.joints.len());
        for joint_num in 0..from.joints.len() {
            joints_steps.push(Rotation::new(
                (to.joints[joint_num].r - from.joints[joint_num].r) / number_of_poses_divider,
            ))
        }

        poses.push(ActorPose::new(from.position_shift, from.joints.to_owned()));

        for i in 0..number_of_poses - 1 {
            let prev_pose = poses.get(i).unwrap();
            let interim_pose = ActorPose::new(
                prev_pose.position_shift + position_shift_step,
                prev_pose
                    .joints
                    .iter()
                    .enumerate()
                    .map(|(i, &v)| v + joints_steps[i])
                    .collect(),
            );
            poses.push(interim_pose);
        }
        poses
    }
}
