use crate::component::animation::AnimationState;
use crate::component::graph::{Node, NodeId};
use crate::component::primitives::Point;
use crate::component::{graph::Graph, position::PositionData};

#[derive(Debug, Clone)]
pub struct AnimationFrames {
    frames: Vec<AnimationFrame>,
}

const ANIMATION_FRAME_TIME: u128 = 20;
impl AnimationFrames {
    pub fn new(frames: Vec<AnimationFrame>) -> AnimationFrames {
        AnimationFrames { frames }
    }

    pub fn update(&self, entity: &mut AnimationState, dt: &u128) {
        let last_frame_time = entity.current_frame_time();
        let mut time_passed = *dt;
        let skip_animation =
            (time_passed + (last_frame_time % ANIMATION_FRAME_TIME)) < ANIMATION_FRAME_TIME;

        if skip_animation {
            entity.set_current_frame_time(time_passed + last_frame_time);
            return;
        }

        let last_frame = entity.current();
        // fast forward to the latest frame based on dt
        let mut animation_time = last_frame.duration_ms;
        while entity.current_frame_time() + time_passed > animation_time {
            time_passed = (entity.current_frame_time() + time_passed) - animation_time;
            entity.set_current_frame_time(0);
            entity.set_current_key_frame(entity.next_key_frame());
            if entity.next_key_frame() + 1 >= self.frames.len() {
                entity.set_next_key_frame(0);
            } else {
                entity.set_next_key_frame(entity.next_key_frame() + 1);
            }
            animation_time = self.frames[entity.current_key_frame()].duration_ms;
            // push into animation ring buffer to draw next time
            // skipped in this implementation as animation system is simple and we don't expect
            // one frame will take longer than animation frequence to draw
        }
        // we don't need the calcualtion of how many animation cycles is in
        // time_passed, we can adjust q to keep things simple

        let q =
            (entity.current_frame_time() + time_passed) as f32 / (last_frame.duration_ms as f32);
        let interpolated_frame = AnimationFrame::interpolate(
            &self.frames[entity.current_key_frame()],
            &self.frames[entity.next_key_frame()],
            q,
        );

        entity.set_current_frame_time(entity.current_frame_time() + time_passed);
        entity.set_current(interpolated_frame.clone());
    }
}

#[derive(Debug, Clone)]
pub struct AnimationFrame {
    pub pose: Graph<PositionData>,
    duration_ms: u128,
}

impl AnimationFrame {
    pub fn new(pose: Graph<PositionData>, duration_ms: u128) -> Self {
        if duration_ms < ANIMATION_FRAME_TIME {
            panic!("poses animation time is smaller than one frame {:?}", pose);
        }
        AnimationFrame { pose, duration_ms }
    }

    pub fn interpolate(from: &AnimationFrame, to: &AnimationFrame, q: f32) -> Box<AnimationFrame> {
        let mut interim_pose = Graph::copy_graph(&from.pose);
        let target_pose = &to.pose;
        let target_joints = target_pose.traverse();
        for joint_num in 0..target_joints.len() {
            let (node_id, position) = AnimationFrame::interpolate_joint_position(
                target_joints.as_slice(),
                &interim_pose,
                joint_num,
                q,
            );
            interim_pose.get_mut(node_id).set(position);
        }
        Box::new(AnimationFrame {
            pose: interim_pose,
            duration_ms: from.duration_ms,
        })
    }

    fn interpolate_joint_position(
        target_joints: &[&Node<PositionData>],
        interim_pose: &Graph<PositionData>,
        join_num: usize,
        q: f32,
    ) -> (NodeId, PositionData) {
        let target_node = target_joints.get(join_num).unwrap();
        let interim_node = interim_pose.get(target_node.node_id());
        let angle: f32;
        let start_point: Point;
        if interim_node.parent().is_none() {
            angle = interim_node.get().angle.r
                + ((target_node.get().angle.r - interim_node.get().angle.r) * q);
            start_point = interim_node.get().point
                + Point::from_tuple((
                    ((target_node.get().point.x() - interim_node.get().point.x()) * q),
                    ((target_node.get().point.y() - interim_node.get().point.y()) * q),
                ));
        } else {
            angle = interim_node.get().angle.r
                + ((target_node.get().angle.r - interim_node.get().angle.r) * q);

            let parent = interim_node.parent().unwrap();
            let _point = interim_pose.get(parent).get().middle_x_end();
            start_point = Point::from_tuple((
                _point.x() + target_node.get().point.x(),
                _point.y() + target_node.get().point.y(),
            ));
        }

        (
            target_node.node_id(),
            PositionData::new(
                start_point,
                angle.into(),
                target_node.get().width,
                target_node.get().height,
                target_node.get().z,
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolate_preserves_z_values() {
        // Build two simple frames with different positions but known z values
        // Frame A: two nodes
        let mut g_from: Graph<PositionData> = Graph::new();
        let root = g_from.add(Node::new(PositionData::new((0.0, 0.0).into(), 0.0.into(), 0, 0, 0)));
        let child = g_from.add(Node::new(PositionData::new((10.0, 0.0).into(), 0.0.into(), 0, 0, 1)).set_parent(root));
        g_from.get_mut(root).append_child(child);

        // Frame B: move both nodes, change angles, but keep z; child has z = 1
        let mut g_to = Graph::copy_graph(&g_from);
        // mutate positions and angles but not z
        for n in g_to.entries_mut() {
            let mut p = *n.get();
            p.point = (p.point.x() + 5.0, p.point.y() + 2.5).into();
            p.angle = (p.angle.r + 15.0).into();
            n.set(p);
        }

        let a = AnimationFrame::new(g_from, 100);
        let b = AnimationFrame::new(g_to, 100);

        // Interpolate half-way
        let i = AnimationFrame::interpolate(&a, &b, 0.5);
        let zs: Vec<i32> = i.pose.entries().iter().map(|n| n.get().z).collect();
        assert_eq!(zs, vec![0, 1]);
    }
}
