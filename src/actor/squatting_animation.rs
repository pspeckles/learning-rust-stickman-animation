use crate::component::graph::Graph;
use crate::component::position::PositionNode;

use super::{
    animation::{AnimationFrame, AnimationFrames},
    skeleton::PoseView,
    t_pose::{t_pose_with_handles, E, N, QUARTER, S, W},
};

pub fn squatting_animation() -> AnimationFrames {
    let (base_frame, handles) = t_pose_with_handles();
    let base = base_frame.pose;

    let mut standing_pose = Graph::copy_graph(&base);
    {
        let mut view = PoseView::new(&mut standing_pose, &handles);
        let r = *view.root_mut().get();
        view.root_mut().set_point((r.point.x(), 245.0).into());
        view.right_upper_arm_mut()
            .set_angle((W - QUARTER / 2.0).into());
        view.right_lower_arm_mut()
            .set_angle((S + QUARTER / 3.0).into());
        view.left_upper_arm_mut()
            .set_angle((E + QUARTER / 2.0).into());
        view.left_lower_arm_mut()
            .set_angle((S - QUARTER / 3.0).into());
        view.right_upper_leg_mut()
            .set_angle((W - QUARTER / 1.2).into());
        view.right_lower_leg_mut().set_angle(S.into());
        view.left_upper_leg_mut()
            .set_angle((E + QUARTER / 1.2).into());
        view.left_lower_leg_mut().set_angle(S.into());
    }

    let mut squatting_pose = Graph::copy_graph(&base);
    {
        let mut view = PoseView::new(&mut squatting_pose, &handles);
        let r = *view.root_mut().get();
        view.root_mut().set_point((r.point.x(), 325.0).into());
        // arms
        view.right_upper_arm_mut().set_angle((W + QUARTER).into());
        view.right_lower_arm_mut().set_angle((N).into());
        view.left_upper_arm_mut().set_angle((E - QUARTER).into());
        view.left_lower_arm_mut().set_angle((-N).into());
        // legs
        view.right_upper_leg_mut()
            .set_angle((W + QUARTER / 3.0).into());
        view.right_lower_leg_mut().set_angle(S.into());
        view.left_upper_leg_mut()
            .set_angle((E - QUARTER / 3.0).into());
        view.left_lower_leg_mut().set_angle(S.into());
    }

    let standing_pose_animation = AnimationFrame::new(standing_pose, 1000);
    let squatting_pose_animation = AnimationFrame::new(squatting_pose, 1000);

    AnimationFrames::new(vec![standing_pose_animation, squatting_pose_animation])
}
