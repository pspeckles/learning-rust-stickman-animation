use core::panic;
use std::os::unix::raw::ino_t;

use crate::actor::pose;

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
    pub fn interpolate(poses: &Vec<&ActorPose>, number_of_poses: usize) -> Vec<ActorPose> {
        //check poses are equal
        if poses.len() < 2 {
            panic!("cannot interpolate less than 2 poses")
        }
        let joints_num = poses[0].joints.len();
        for p in poses {
            if p.joints.len() != joints_num {
                panic!("cannot interpolate different poses")
            }
        }
        if number_of_poses < 2 {
            panic!("number_of_poses cannot be less than 2")
        }

        let mut interpolated = Vec::with_capacity((number_of_poses - 1) * poses.len());
        interpolated.push(poses[0].clone());
        for pose_idx in 0..poses.len() - 1 {
            let from = poses[pose_idx];
            let to = poses[pose_idx + 1];
            let interpolation_steps = (number_of_poses - 1) as f32;
            let position_shift_step = Point::from_tuple((
                (to.position_shift.x() - from.position_shift.x()) / interpolation_steps,
                (to.position_shift.y() - from.position_shift.y()) / interpolation_steps,
            ));
            let mut joints_steps = Vec::with_capacity(from.joints.len());
            for joint_num in 0..from.joints.len() {
                joints_steps.push(Rotation::new(
                    (to.joints[joint_num].r - from.joints[joint_num].r) / interpolation_steps,
                ))
            }

            for i in pose_idx * (interpolation_steps as usize)
                ..(pose_idx * interpolation_steps as usize) + interpolation_steps as usize
            {
                let prev_pose = &interpolated[i];
                let interim_pose = ActorPose::new(
                    prev_pose.position_shift + position_shift_step,
                    prev_pose
                        .joints
                        .iter()
                        .enumerate()
                        .map(|(i, &v)| v + joints_steps[i])
                        .collect(),
                );
                interpolated.push(interim_pose);
            }
        }
        // remove last frame to make a full loop
        interpolated.pop();
        interpolated
    }
}

impl PartialEq for ActorPose {
    fn eq(&self, other: &ActorPose) -> bool {
        return self.position_shift.y() == other.position_shift.y()
            && self.position_shift.x() == other.position_shift.x();
    }
}
