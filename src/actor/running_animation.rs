use crate::component::graph::Graph;
use crate::component::position::PositionNode;

use super::{
    animation::{AnimationFrame, AnimationFrames},
    skeleton::PoseView,
    t_pose::{t_pose_with_handles, E, QUARTER, S},
};

pub fn running_animation() -> AnimationFrames {
    let (base_frame, handles) = t_pose_with_handles();
    let base = base_frame.pose;

    // 4-step side-scroller running cycle based on current t-pose
    let base_root = (300.0, 180.0);

    // Frame 0: contact (right leg forward, left back)
    let mut f0 = Graph::copy_graph(&base);
    {
        let mut view = PoseView::new(&mut f0, &handles);
        view.root_mut().set_point(base_root.into());
        // Arms (opposite to legs), add elbow bend
        view.right_upper_arm_mut().set_angle((E + QUARTER / 4.0).into());
        view.right_lower_arm_mut().set_angle((E + QUARTER / 3.0).into());
        view.left_upper_arm_mut().set_angle((E - QUARTER / 4.0).into());
        view.left_lower_arm_mut().set_angle((E - QUARTER / 6.0).into());
        // Legs (front extended, back bent)
        view.right_upper_leg_mut().set_angle((S - QUARTER / 5.0).into());
        view.right_lower_leg_mut().set_angle((S - QUARTER / 12.0).into());
        view.left_upper_leg_mut().set_angle((S + QUARTER / 5.0).into());
        view.left_lower_leg_mut().set_angle((S + QUARTER / 6.0).into());
    }

    // Frame 1: recoil (lower, pushing)
    let mut f1 = Graph::copy_graph(&base);
    {
        let mut view = PoseView::new(&mut f1, &handles);
        view.root_mut()
            .set_point((base_root.0, base_root.1 + 6.0).into());
        view.right_upper_arm_mut().set_angle((E + QUARTER / 8.0).into());
        view.right_lower_arm_mut().set_angle((E + QUARTER / 6.0).into());
        view.left_upper_arm_mut().set_angle((E - QUARTER / 8.0).into());
        view.left_lower_arm_mut().set_angle((E - QUARTER / 10.0).into());
        view.right_upper_leg_mut().set_angle((S - QUARTER / 12.0).into());
        view.right_lower_leg_mut().set_angle(S.into());
        view.left_upper_leg_mut().set_angle((S + QUARTER / 12.0).into());
        view.left_lower_leg_mut().set_angle((S + QUARTER / 12.0).into());
    }

    // Frame 2: passing (near neutral, higher)
    let mut f2 = Graph::copy_graph(&base);
    {
        let mut view = PoseView::new(&mut f2, &handles);
        view.root_mut()
            .set_point((base_root.0, base_root.1 - 4.0).into());
        view.right_upper_arm_mut().set_angle((E - QUARTER / 10.0).into());
        view.right_lower_arm_mut().set_angle((E - QUARTER / 8.0).into());
        view.left_upper_arm_mut().set_angle((E + QUARTER / 10.0).into());
        view.left_lower_arm_mut().set_angle((E + QUARTER / 8.0).into());
        view.right_upper_leg_mut().set_angle((S + QUARTER / 24.0).into());
        view.right_lower_leg_mut().set_angle((S + QUARTER / 24.0).into());
        view.left_upper_leg_mut().set_angle((S - QUARTER / 24.0).into());
        view.left_lower_leg_mut().set_angle((S - QUARTER / 24.0).into());
    }

    // Frame 3: contact mirrored (right back, left forward)
    let mut f3 = Graph::copy_graph(&base);
    {
        let mut view = PoseView::new(&mut f3, &handles);
        view.root_mut().set_point(base_root.into());
        view.right_upper_arm_mut().set_angle((E - QUARTER / 4.0).into());
        view.right_lower_arm_mut().set_angle((E - QUARTER / 6.0).into());
        view.left_upper_arm_mut().set_angle((E + QUARTER / 4.0).into());
        view.left_lower_arm_mut().set_angle((E + QUARTER / 3.0).into());
        view.right_upper_leg_mut().set_angle((S + QUARTER / 5.0).into());
        view.right_lower_leg_mut().set_angle((S + QUARTER / 6.0).into());
        view.left_upper_leg_mut().set_angle((S - QUARTER / 5.0).into());
        view.left_lower_leg_mut().set_angle((S - QUARTER / 12.0).into());
    }

    let d = 120; // ms per key pose
    AnimationFrames::new(vec![
        AnimationFrame::new(f0, d),
        AnimationFrame::new(f1, d),
        AnimationFrame::new(f2, d),
        AnimationFrame::new(f3, d),
    ])
}
