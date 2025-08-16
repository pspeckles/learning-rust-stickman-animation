use crate::component::{
    graph::{Graph, Node},
    position::PositionData,
    primitives::Point,
};

use super::animation::AnimationFrame;

pub const N: f32 = 180.0;
pub const E: f32 = 270.0;
pub const W: f32 = 90.0;
pub const S: f32 = 0.0;
pub const QUARTER: f32 = 90.0;

pub const LIMB_WIDTH: f32 = 20.0;
pub const LIMB_HEIGHT: f32 = 60.0;
pub const TORSO_HEIGHT: f32 = 150.0;
pub const TORSO_WIDTH: f32 = 60.0;

pub fn t_pose() -> AnimationFrame {
    let starting_p = PositionData::new(Point::new(300.0, 200.0), S.into(), 0, 0);
    let head_p = PositionData::new(Point::new(0.0, 0.0), S.into(), 40, 60);
    let neck_p = PositionData::new(Point::new(0.0, 0.0), S.into(), 15, 20);
    let torso_p = PositionData::new(
        Point::new(0.0, 0.0),
        S.into(),
        TORSO_WIDTH as u16,
        TORSO_HEIGHT as u16,
    );
    let right_upper_hand_p = PositionData::new(
        Point::new(-TORSO_WIDTH / 2.0, -TORSO_HEIGHT),
        W.into(),
        LIMB_WIDTH as u16,
        LIMB_HEIGHT as u16,
    );
    let right_lower_hand_p = PositionData::new(
        Point::new(0.0, 0.0),
        W.into(),
        LIMB_WIDTH as u16,
        LIMB_HEIGHT as u16,
    );
    let left_upper_hand_p = PositionData::new(
        Point::new(TORSO_WIDTH / 2.0, -TORSO_HEIGHT),
        // (E - 34.0).into(),
        E.into(),
        LIMB_WIDTH as u16,
        LIMB_HEIGHT as u16,
    );
    let left_lower_hand_p = PositionData::new(
        Point::new(0.0, 0.0),
        E.into(),
        LIMB_WIDTH as u16,
        LIMB_HEIGHT as u16,
    );
    let lower_back_p = PositionData::new(Point::new(0.0, 0.0), S.into(), 0, 0);
    let right_upper_leg_p = PositionData::new(
        Point::new(-TORSO_WIDTH / 2.0, 0.0),
        S.into(),
        LIMB_WIDTH as u16,
        LIMB_HEIGHT as u16,
    );
    let right_lower_leg_p = PositionData::new(
        Point::new(0.0, 0.0),
        S.into(),
        LIMB_WIDTH as u16,
        LIMB_HEIGHT as u16,
    );
    let left_upper_leg_p = PositionData::new(
        Point::new(TORSO_WIDTH / 2.0, 0.0),
        S.into(),
        LIMB_WIDTH as u16,
        LIMB_HEIGHT as u16,
    );
    let left_lower_leg_p = PositionData::new(
        Point::new(0.0, 0.0),
        S.into(),
        LIMB_WIDTH as u16,
        LIMB_HEIGHT as u16,
    );

    let mut standing_pose = Graph::new();
    let standing = standing_pose.add(Node::new(starting_p));
    let head = standing_pose.add(Node::new(head_p).set_parent(standing));
    let neck = standing_pose.add(Node::new(neck_p).set_parent(head));
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

    AnimationFrame::new(standing_pose, 1000)
}
