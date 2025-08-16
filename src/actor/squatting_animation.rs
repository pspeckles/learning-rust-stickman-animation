use crate::component::{graph::Graph, position::PositionData};

use super::{
    animation::{AnimationFrame, AnimationFrames},
    t_pose::{t_pose, E, N, QUARTER, S, W},
};

pub fn squatting_animation() -> AnimationFrames {
    let base = t_pose().pose;
    let mut standing_pose = Graph::copy_graph(&base);
    for node in standing_pose.entries_mut().iter_mut() {
        let node_idx = node.node_id().idx;
        //root
        if node_idx == 0 {
            let mut u = *node.get();
            u.point = (300.0, 250.0).into();
            node.set(u);
            continue;
        }
        // right_upper_hand
        if node_idx == 4 {
            node.set(PositionData::new(
                node.get().point,
                (W - QUARTER / 2.0).into(),
                node.get().width,
                node.get().height,
            ));
            continue;
        }
        // right_lower_hand
        if node_idx == 5 {
            node.set(PositionData::new(
                node.get().point,
                (S + QUARTER / 3.0).into(),
                node.get().width,
                node.get().height,
            ));
            continue;
        }
        // left_upper_hand
        if node_idx == 6 {
            node.set(PositionData::new(
                node.get().point,
                (E + QUARTER / 2.0).into(),
                node.get().width,
                node.get().height,
            ));
            continue;
        }
        // left_lower_hand
        if node_idx == 7 {
            node.set(PositionData::new(
                node.get().point,
                (S - QUARTER / 3.0).into(),
                node.get().width,
                node.get().height,
            ));
            continue;
        }
        // right_upper_leg
        if node_idx == 9 {
            node.set(PositionData::new(
                node.get().point,
                (W - QUARTER / 2.0).into(),
                node.get().width,
                node.get().height,
            ));
            continue;
        }

        // right_lower_leg
        if node_idx == 10 {
            node.set(PositionData::new(
                node.get().point,
                S.into(),
                node.get().width,
                node.get().height,
            ));
            continue;
        }
        // left_upper_leg
        if node_idx == 11 {
            node.set(PositionData::new(
                node.get().point,
                (E + QUARTER / 2.0).into(),
                node.get().width,
                node.get().height,
            ));
            continue;
        }
        // left_lower_leg
        if node_idx == 12 {
            node.set(PositionData::new(
                node.get().point,
                S.into(),
                node.get().width,
                node.get().height,
            ));
            continue;
        }
    }
    let mut squatting_pose = Graph::copy_graph(&base);
    for node in squatting_pose.entries_mut().iter_mut() {
        let node_idx = node.node_id().idx;
        //root
        if node_idx == 0 {
            let mut u = *node.get();
            u.point = (300.0, 320.0).into();
            node.set(u);
            continue;
        }
        // right_upper_hand
        if node_idx == 4 {
            node.set(PositionData::new(
                node.get().point,
                (W + QUARTER).into(),
                node.get().width,
                node.get().height,
            ));
            continue;
        }
        // right_lower_hand
        if node_idx == 5 {
            node.set(PositionData::new(
                node.get().point,
                (N).into(),
                node.get().width,
                node.get().height,
            ));
            continue;
        }
        // left_upper_hand
        if node_idx == 6 {
            node.set(PositionData::new(
                node.get().point,
                (E - QUARTER).into(),
                node.get().width,
                node.get().height,
            ));
            continue;
        }
        // left_lower_hand
        if node_idx == 7 {
            node.set(PositionData::new(
                node.get().point,
                (-N).into(),
                node.get().width,
                node.get().height,
            ));
            continue;
        }
        // right_upper_leg
        if node_idx == 9 {
            node.set(PositionData::new(
                node.get().point,
                (W + QUARTER / 3.0).into(),
                node.get().width,
                node.get().height,
            ));
            continue;
        }

        // right_lower_leg
        if node_idx == 10 {
            node.set(PositionData::new(
                node.get().point,
                S.into(),
                node.get().width,
                node.get().height,
            ));
            continue;
        }
        // left_upper_leg
        if node_idx == 11 {
            node.set(PositionData::new(
                node.get().point,
                (E - QUARTER / 3.0).into(),
                node.get().width,
                node.get().height,
            ));
            continue;
        }
        // left_lower_leg
        if node_idx == 12 {
            node.set(PositionData::new(
                node.get().point,
                S.into(),
                node.get().width,
                node.get().height,
            ));
            continue;
        }
    }

    let standing_pose_animation = AnimationFrame::new(standing_pose, 1000);
    let squatting_pose_animation = AnimationFrame::new(squatting_pose, 1000);

    AnimationFrames::new(vec![standing_pose_animation, squatting_pose_animation])
}
