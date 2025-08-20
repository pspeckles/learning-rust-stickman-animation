use crate::component::{graph::Graph, position::PositionData};

use super::{
    animation::{AnimationFrame, AnimationFrames},
    skeleton::PoseView,
    t_pose::{t_pose_with_handles, E, N, QUARTER, S, W},
};

pub fn running_animation() -> AnimationFrames {
    let (base_frame, handles) = t_pose_with_handles();
    let base = base_frame.pose;

    // Standing variant
    let mut standing_pose = Graph::copy_graph(&base);
    {
        let mut view = PoseView::new(&mut standing_pose, &handles);
        // root
        let mut r = *view.root_mut().get();
        r.point = (200.0, 150.0).into();
        view.root_mut().set(r);
        // right upper/lower hand
        let ru = *view.right_upper_arm_mut().get();
        view.right_upper_arm_mut().set(PositionData::new(ru.point, (E).into(), ru.width, ru.height, ru.z));
        let rl = *view.right_lower_arm_mut().get();
        view.right_lower_arm_mut().set(PositionData::new(rl.point, (S + QUARTER / 3.0).into(), rl.width, rl.height, rl.z));
        // left upper/lower hand
        let lu = *view.left_upper_arm_mut().get();
        view.left_upper_arm_mut().set(PositionData::new(lu.point, (E + QUARTER / 2.0).into(), lu.width, lu.height, lu.z));
        let ll = *view.left_lower_arm_mut().get();
        view.left_lower_arm_mut().set(PositionData::new(ll.point, (S - QUARTER / 3.0).into(), ll.width, ll.height, ll.z));
        // legs
        let rul = *view.right_upper_leg_mut().get();
        view.right_upper_leg_mut().set(PositionData::new(rul.point, (W - QUARTER / 1.2).into(), rul.width, rul.height, rul.z));
        let rll = *view.right_lower_leg_mut().get();
        view.right_lower_leg_mut().set(PositionData::new(rll.point, S.into(), rll.width, rll.height, rll.z));
        let lul = *view.left_upper_leg_mut().get();
        view.left_upper_leg_mut().set(PositionData::new(lul.point, (E + QUARTER / 1.2).into(), lul.width, lul.height, lul.z));
        let lll = *view.left_lower_leg_mut().get();
        view.left_lower_leg_mut().set(PositionData::new(lll.point, S.into(), lll.width, lll.height, lll.z));
    }

    // Squatting variant
    let mut squatting_pose = Graph::copy_graph(&base);
    {
        let mut view = PoseView::new(&mut squatting_pose, &handles);
        // root
        let r = *view.root_mut().get();
        view.root_mut().set(PositionData::new((400.0, 150.0).into(), r.angle, r.width, r.height, r.z));
        // right upper/lower hand
        let ru = *view.right_upper_arm_mut().get();
        view.right_upper_arm_mut().set(PositionData::new(ru.point, (E + QUARTER).into(), ru.width, ru.height, ru.z));
        let rl = *view.right_lower_arm_mut().get();
        view.right_lower_arm_mut().set(PositionData::new(rl.point, (N).into(), rl.width, rl.height, rl.z));
        // left upper/lower hand
        let lu = *view.left_upper_arm_mut().get();
        view.left_upper_arm_mut().set(PositionData::new(lu.point, (E - QUARTER).into(), lu.width, lu.height, lu.z));
        let ll = *view.left_lower_arm_mut().get();
        view.left_lower_arm_mut().set(PositionData::new(ll.point, (-N).into(), ll.width, ll.height, ll.z));
        // legs
        let rul = *view.right_upper_leg_mut().get();
        view.right_upper_leg_mut().set(PositionData::new(rul.point, (W + QUARTER / 3.0).into(), rul.width, rul.height, rul.z));
        let rll = *view.right_lower_leg_mut().get();
        view.right_lower_leg_mut().set(PositionData::new(rll.point, S.into(), rll.width, rll.height, rll.z));
        let lul = *view.left_upper_leg_mut().get();
        view.left_upper_leg_mut().set(PositionData::new(lul.point, (E - QUARTER / 3.0).into(), lul.width, lul.height, lul.z));
        let lll = *view.left_lower_leg_mut().get();
        view.left_lower_leg_mut().set(PositionData::new(lll.point, S.into(), lll.width, lll.height, lll.z));
    }

    let standing_pose_animation = AnimationFrame::new(standing_pose, 1000);
    let squatting_pose_animation = AnimationFrame::new(squatting_pose, 1000);

    AnimationFrames::new(vec![standing_pose_animation, squatting_pose_animation])
}
