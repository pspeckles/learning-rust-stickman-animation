use super::primitives::{Point, Rotation};

pub const TOP: f32 = -90.0;
pub const LEFT: f32 = -180.0;
pub const RIGHT: f32 = 0.0;
pub const BOTTOM: f32 = 90.0;

#[derive(Debug, Clone, Copy)]
pub struct Pose {
    pub position_shift: Point,
    pub arm_left_upper: Rotation,
    pub arm_left_lower: Rotation,
    pub arm_right_upper: Rotation,
    pub arm_right_lower: Rotation,
    pub leg_left_upper: Rotation,
    pub leg_left_lower: Rotation,
    pub leg_right_upper: Rotation,
    pub leg_right_lower: Rotation,
}

impl Pose {
    pub fn new(position_shift: Point, joints: &[Rotation]) -> Self {
        Pose {
            position_shift,
            arm_right_upper: joints[0],
            arm_right_lower: joints[1],
            leg_right_upper: joints[2],
            leg_right_lower: joints[3],
            leg_left_upper: joints[4],
            leg_left_lower: joints[5],
            arm_left_upper: joints[6],
            arm_left_lower: joints[7],
        }
    }
}
