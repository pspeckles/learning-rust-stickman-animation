use crate::component::{
    graph::{Graph, Node},
    position::PositionData,
    primitives::Point,
};

use super::{animation::AnimationFrame, animation::AnimationFrames};

pub const TOP: f32 = 270.0;
pub const TOP_N: f32 = -90.0;
pub const LEFT: f32 = 180.0;
pub const RIGHT: f32 = 0.0;
pub const BOTTOM: f32 = 90.0;
pub const QUARTER_CLOCKWISE: f32 = 90.0;
pub const QUARTER_ANTI_CLOCKWISE: f32 = -90.0;

pub fn squatting_animation() -> AnimationFrames {
    let starting_p = PositionData::new(Point::new(300.0, 300.0), 0.0.into(), 0, 0);
    let head_p = PositionData::new(Point::new(0.0, 0.0), 0.0.into(), 30, 30);
    let neck_p = PositionData::new(Point::new(0.0, 0.0), 0.0.into(), 50, 0);
    let torso_p = PositionData::new(Point::new(0.0, 0.0), 0.0.into(), 10, 150);
    let right_upper_hand_p = PositionData::new(
        Point::new(0.0, 0.0),
        (RIGHT + QUARTER_CLOCKWISE / 3.0).into(),
        20,
        20,
    );
    let right_lower_hand_p = PositionData::new(Point::new(0.0, 0.0), BOTTOM.into(), 20, 20);
    let left_upper_hand_p = PositionData::new(
        Point::new(0.0, 0.0),
        (LEFT + QUARTER_ANTI_CLOCKWISE / 3.0).into(),
        20,
        20,
    );
    let left_lower_hand_p = PositionData::new(Point::new(0.0, 0.0), BOTTOM.into(), 20, 20);
    let lower_back_p = PositionData::new(Point::new(0.0, 0.0), 0.0.into(), 0, 0);
    let right_upper_leg_p = PositionData::new(
        Point::new(0.0, 0.0),
        (RIGHT + QUARTER_CLOCKWISE / 3.0).into(),
        20,
        20,
    );
    let right_lower_leg_p = PositionData::new(Point::new(0.0, 0.0), BOTTOM.into(), 20, 20);
    let left_upper_leg_p = PositionData::new(
        Point::new(0.0, 0.0),
        (LEFT + QUARTER_ANTI_CLOCKWISE / 3.0).into(),
        20,
        20,
    );
    let left_lower_leg_p = PositionData::new(Point::new(0.0, 0.0), BOTTOM.into(), 20, 20);

    let mut standing_pose = Graph::new();
    let standing = standing_pose.add(Node::new(starting_p));
    let head = standing_pose.add(Node::new(head_p).set_parent(standing));
    let neck = standing_pose.add(Node::new(neck_p).set_parent(standing));
    let torso = standing_pose.add(Node::new(torso_p).set_parent(neck));
    let right_upper_hand = standing_pose.add(Node::new(right_upper_hand_p).set_parent(torso));
    let right_lower_hand =
        standing_pose.add(Node::new(right_lower_hand_p).set_parent(right_upper_hand));
    let left_upper_hand = standing_pose.add(Node::new(left_upper_hand_p).set_parent(torso));
    let left_lower_hand =
        standing_pose.add(Node::new(left_lower_hand_p).set_parent(left_upper_hand));
    let lower_back = standing_pose.add(Node::new(lower_back_p).set_parent(torso));
    let right_upper_leg = standing_pose.add(Node::new(right_upper_leg_p).set_parent(lower_back));
    let right_lower_leg =
        standing_pose.add(Node::new(right_lower_leg_p).set_parent(right_upper_leg));
    let left_upper_leg = standing_pose.add(Node::new(left_upper_leg_p).set_parent(lower_back));
    let left_lower_leg = standing_pose.add(Node::new(left_lower_leg_p).set_parent(left_upper_leg));

    standing_pose
        .get_mut(standing)
        .append_children(vec![head, neck]);
    standing_pose.get_mut(neck).append_child(torso);
    standing_pose.get_mut(torso).append_children(vec![
        right_upper_hand,
        left_upper_hand,
        lower_back,
    ]);
    standing_pose
        .get_mut(right_upper_hand)
        .append_child(right_lower_hand);
    standing_pose
        .get_mut(left_upper_hand)
        .append_child(left_lower_hand);
    standing_pose
        .get_mut(lower_back)
        .append_children(vec![right_upper_leg, left_upper_leg]);
    standing_pose
        .get_mut(right_upper_leg)
        .append_child(right_lower_leg);
    standing_pose
        .get_mut(left_upper_leg)
        .append_child(left_lower_leg);

    let mut squatting_pose = Graph::copy_graph(&standing_pose);
    for node in squatting_pose.entries_mut().iter_mut() {
        let node_idx = node.node_id().idx;
        //root
        if node_idx == 0 {
            node.set(PositionData::new(
                Point::new(300.0, 150.0),
                0.0.into(),
                10,
                10,
            ));
            continue;
        }
        // right_upper_hand
        if node_idx == 4 {
            node.set(PositionData::new(
                Point::new(0.0, 0.0),
                (RIGHT + QUARTER_ANTI_CLOCKWISE / 3.0).into(),
                20,
                20,
            ));
            continue;
        }
        // right_lower_hand
        if node_idx == 5 {
            node.set(PositionData::new(
                Point::new(0.0, 0.0),
                (LEFT + QUARTER_CLOCKWISE / 3.0).into(),
                20,
                20,
            ));
            continue;
        }
        // right_lower_hand
        if node_idx == 7 {
            node.set(PositionData::new(
                Point::new(0.0, 0.0),
                TOP_N.into(),
                20,
                20,
            ));
            continue;
        }
        // left_lower_hand
        if node_idx == 8 {
            node.set(PositionData::new(
                Point::new(0.0, 0.0),
                TOP_N.into(),
                20,
                20,
            ));
            continue;
        }
        // right_upper_leg
        if node_idx == 9 {
            node.set(PositionData::new(
                Point::new(0.0, 0.0),
                (RIGHT + QUARTER_ANTI_CLOCKWISE / 3.0).into(),
                20,
                20,
            ));
            continue;
        }
        // left_upper_leg
        if node_idx == 10 {
            node.set(PositionData::new(
                Point::new(0.0, 0.0),
                (LEFT + QUARTER_CLOCKWISE / 3.0).into(),
                20,
                20,
            ));
            continue;
        }
        // right_lower_leg
        if node_idx == 11 {
            node.set(PositionData::new(
                Point::new(0.0, 0.0),
                BOTTOM.into(),
                20,
                20,
            ));
            continue;
        }
        // left_lower_leg
        if node_idx == 12 {
            node.set(PositionData::new(
                Point::new(0.0, 0.0),
                BOTTOM.into(),
                20,
                20,
            ));
            continue;
        }
    }

    let standing_pose_animation = AnimationFrame::new(standing_pose, 1000);
    let squatting_pose_animation = AnimationFrame::new(squatting_pose, 1000);

    AnimationFrames::new(vec![standing_pose_animation, squatting_pose_animation])
}
