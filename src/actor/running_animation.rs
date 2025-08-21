use crate::component::graph::Graph;
use crate::component::position::PositionNode;

use super::{
    animation::{AnimationFrame, AnimationFrames},
    skeleton::PoseView,
    t_pose::{t_pose_with_handles, E, HALF, N, S, W},
};

pub fn running_animation() -> AnimationFrames {
    let (base_frame, handles) = t_pose_with_handles();
    let base = base_frame.pose;

    let base_root = (300.0, 180.0);

    let mut f0 = Graph::copy_graph(&base);
    {
        let mut view = PoseView::new(&mut f0, &handles);
        view.root_mut()
            .set_point((base_root.0, base_root.1 + 10.0).into());
        view.right_upper_arm_mut()
            .set_angle((W - HALF / 10.0).into());
        view.right_lower_arm_mut()
            .set_angle((S + HALF / 12.0).into());
        view.left_upper_arm_mut().set_angle((E - HALF / 3.0).into());
        view.left_lower_arm_mut()
            .set_angle((N - HALF / 14.0).into());
        view.right_upper_leg_mut()
            .set_angle((S + HALF / 1.0).into());
        view.right_lower_leg_mut()
            .set_angle((S + HALF / 1.0).into());
        view.left_upper_leg_mut().set_angle((E - HALF / 1.0).into());
        view.left_lower_leg_mut().set_angle((S + HALF / 9.0).into());
    }

    let mut f1 = Graph::copy_graph(&base);
    {
        let mut view = PoseView::new(&mut f1, &handles);
        view.root_mut()
            .set_point((base_root.0, base_root.1 - 6.0).into());
        view.right_upper_arm_mut()
            .set_angle((S + HALF / 6.0).into());
        view.right_lower_arm_mut()
            .set_angle((E - HALF / 6.0).into());
        view.left_upper_arm_mut().set_angle((E + HALF / 2.0).into());
        view.left_lower_arm_mut().set_angle((E - HALF / 2.0).into());
        view.right_upper_leg_mut()
            .set_angle((S - HALF / 2.0).into());
        view.right_lower_leg_mut()
            .set_angle((S + HALF / 2.0).into());
        view.left_upper_leg_mut().set_angle((E + HALF / 2.0).into());
        view.left_lower_leg_mut()
            .set_angle((S + HALF / 10.0).into());
    }

    let mut f2 = Graph::copy_graph(&base);
    {
        let mut view = PoseView::new(&mut f2, &handles);
        view.root_mut()
            .set_point((base_root.0, base_root.1 + 10.0).into());
        view.right_upper_arm_mut()
            .set_angle((E - HALF / 3.0).into());
        view.right_lower_arm_mut()
            .set_angle((N - HALF / 14.0).into());
        view.left_upper_arm_mut()
            .set_angle((W - HALF / 10.0).into());
        view.left_lower_arm_mut()
            .set_angle((S + HALF / 12.0).into());
        view.right_upper_leg_mut()
            .set_angle((E + HALF / 1.5).into());
        view.right_lower_leg_mut()
            .set_angle((S + HALF / 1.5).into());
        view.left_upper_leg_mut().set_angle((E + HALF / 1.0).into());
        view.left_lower_leg_mut().set_angle((E + HALF / 1.5).into());
    }

    let mut f3 = Graph::copy_graph(&base);
    {
        let mut view = PoseView::new(&mut f3, &handles);
        view.root_mut()
            .set_point((base_root.0, base_root.1 - 6.0).into());
        view.right_upper_arm_mut()
            .set_angle((E + HALF / 2.0).into());
        view.right_lower_arm_mut()
            .set_angle((E - HALF / 2.0).into());
        view.left_upper_arm_mut().set_angle((S + HALF / 6.0).into());
        view.left_lower_arm_mut().set_angle((E - HALF / 6.0).into());
        view.right_upper_leg_mut()
            .set_angle((E - HALF / 1.0).into());
        view.right_lower_leg_mut()
            .set_angle((S + HALF / 9.0).into());
        view.left_upper_leg_mut().set_angle((S + HALF / 1.0).into());
        view.left_lower_leg_mut().set_angle((S + HALF / 1.0).into());
    }
    let mut f4 = Graph::copy_graph(&base);
    {
        let mut view = PoseView::new(&mut f4, &handles);
        view.root_mut()
            .set_point((base_root.0, base_root.1 - 6.0).into());
        view.right_upper_arm_mut()
            .set_angle((E + HALF / 2.0).into());
        view.right_lower_arm_mut()
            .set_angle((E - HALF / 2.0).into());
        view.left_upper_arm_mut().set_angle((S + HALF / 6.0).into());
        view.left_lower_arm_mut().set_angle((E - HALF / 6.0).into());
        view.right_upper_leg_mut()
            .set_angle((E + HALF / 2.0).into());
        view.right_lower_leg_mut()
            .set_angle((S + HALF / 10.0).into());
        view.left_upper_leg_mut().set_angle((S - HALF / 2.0).into());
        view.left_lower_leg_mut().set_angle((S + HALF / 2.0).into());
    }
    let mut f5 = Graph::copy_graph(&base);
    {
        let mut view = PoseView::new(&mut f5, &handles);
        view.root_mut()
            .set_point((base_root.0, base_root.1 - 6.0).into());
        view.right_upper_arm_mut()
            .set_angle((E + HALF / 2.0).into());
        view.right_lower_arm_mut()
            .set_angle((E - HALF / 2.0).into());
        view.left_upper_arm_mut().set_angle((S + HALF / 6.0).into());
        view.left_lower_arm_mut().set_angle((E - HALF / 6.0).into());
        view.right_upper_leg_mut()
            .set_angle((E + HALF / 1.5).into());
        view.right_lower_leg_mut()
            .set_angle((E + HALF / 1.5).into());
        view.left_upper_leg_mut().set_angle((E + HALF / 1.0).into());
        view.left_lower_leg_mut().set_angle((S + HALF / 1.0).into());
    }
    let d = 400; // ms per key pose, smoother 8-step cycle
    AnimationFrames::new(vec![
        AnimationFrame::new(f0, d),
        AnimationFrame::new(f1, d),
        AnimationFrame::new(f2, d),
        AnimationFrame::new(f3, d),
        AnimationFrame::new(f4, d),
        AnimationFrame::new(f5, d),
    ])
}
