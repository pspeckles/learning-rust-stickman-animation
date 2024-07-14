use std::f32::consts::PI;
use std::fmt::DebugStruct;
use std::ops::Add;

use sfml::graphics::Transformable;

use crate::stickman::position::Position;
use crate::stickman::skeleton::Skeleton;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    pub fn from_tuple(p: (f32, f32)) -> Self {
        Point { x: p.0, y: p.1 }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}

const TOP: f32 = -90.0;
const LEFT: f32 = -180.0;
const RIGHT: f32 = 90.0;
const BOTTOM: f32 = 180.0;

#[derive(Debug, Clone, Copy)]
pub struct Rotation {
    pub r: f32,
}

impl Rotation {
    pub fn new(r: f32) -> Self {
        Rotation { r }
    }
}

pub fn figure_end_y(size: f32, position: (f32, f32), rotation: f32) -> (f32, f32) {
    let x = position.0 + (size * (PI / 180.0 * rotation).cos());
    let y = position.1 + (size * (PI / 180.0 * rotation).sin());
    (x, y)
}

pub fn figure_end_x(size: f32, position: (f32, f32), rotation: f32) -> (f32, f32) {
    let x = position.0 - (size * (PI / 180.0 * rotation).sin());
    let y = position.1 + (size * (PI / 180.0 * rotation).cos());
    (x, y)
}

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
    pub fn new(
        position_shift: Point,
        arm_left_upper: Rotation,
        arm_left_lower: Rotation,
        arm_right_upper: Rotation,
        arm_right_lower: Rotation,
        leg_left_upper: Rotation,
        leg_left_lower: Rotation,
        leg_right_upper: Rotation,
        leg_right_lower: Rotation,
    ) -> Self {
        Pose {
            position_shift,
            arm_left_upper,
            arm_left_lower,
            arm_right_upper,
            arm_right_lower,
            leg_left_upper,
            leg_left_lower,
            leg_right_upper,
            leg_right_lower,
        }
    }
    pub fn interpolate(from: Pose, to: Pose, number_of_poses: usize) -> Vec<Pose> {
        let mut poses = Vec::with_capacity(number_of_poses.into());
        let number_of_poses_divider: f32 = number_of_poses as f32;
        let position_shift_step = Point::from_tuple((
            (to.position_shift.x() - from.position_shift.x()) / number_of_poses_divider,
            (to.position_shift.y() - from.position_shift.y()) / number_of_poses_divider,
        ));
        let arm_left_upper_step =
            Rotation::new((to.arm_left_upper.r - from.arm_left_upper.r) / number_of_poses_divider);
        let arm_left_lower_step =
            Rotation::new((to.arm_left_lower.r - from.arm_left_lower.r) / number_of_poses_divider);

        let arm_right_upper_step = Rotation::new(
            (to.arm_right_upper.r - from.arm_right_upper.r) / number_of_poses_divider,
        );
        let arm_right_lower_step = Rotation::new(
            (to.arm_right_lower.r - from.arm_right_lower.r) / number_of_poses_divider,
        );
        let leg_left_upper_step =
            Rotation::new((to.leg_left_upper.r - from.leg_left_upper.r) / number_of_poses_divider);
        let leg_left_lower_step =
            Rotation::new((to.leg_left_lower.r - from.leg_left_lower.r) / number_of_poses_divider);
        let leg_right_upper_step = Rotation::new(
            (to.leg_right_upper.r - from.leg_right_upper.r) / number_of_poses_divider,
        );
        let leg_right_lower_step = Rotation::new(
            (to.leg_right_lower.r - from.leg_right_lower.r) / number_of_poses_divider,
        );

        poses.push(from);

        for i in 0..number_of_poses - 1 {
            let prev_pose = poses.get(i).unwrap();

            let interim_pose = Pose::new(
                prev_pose.position_shift + position_shift_step,
                prev_pose.arm_left_upper + arm_left_upper_step,
                prev_pose.arm_left_lower + arm_left_lower_step,
                prev_pose.arm_right_upper + arm_right_upper_step,
                prev_pose.arm_right_lower + arm_right_lower_step,
                prev_pose.leg_left_upper + leg_left_upper_step,
                prev_pose.leg_left_lower + leg_left_lower_step,
                prev_pose.leg_right_upper + leg_right_upper_step,
                prev_pose.leg_right_lower + leg_right_lower_step,
            );
            poses.push(interim_pose);
        }
        poses
    }
}
impl Position for Pose {
    type Positionable = Skeleton;

    fn apply_to(&self, body: &mut Self::Positionable, point: &Point) {
        body.head.set_position((
            point.x + self.position_shift.x(),
            point.y + self.position_shift.y(),
        ));
        //body
        body.body.set_position((
            body.head.position().x + body.head.radius() - (body.body.size().x / 2.0),
            body.head.position().y + (body.head.radius() * 2.0) - 5.0,
        ));
        //arm left
        body.arm_left_upper.set_position((
            body.body.position().x,
            body.body.position().y + body.head.radius() / 2.0,
        ));
        body.arm_left_upper.set_rotation(self.arm_left_upper.r);

        let arm_left_upper_end = figure_end_y(
            body.arm_left_upper.size().x,
            (
                body.arm_left_upper.position().x,
                body.arm_left_upper.position().y,
            ),
            body.arm_left_upper.rotation(),
        );
        body.arm_left_lower.set_position(arm_left_upper_end);
        body.arm_left_lower.set_rotation(self.arm_left_lower.r);

        //arm right
        body.arm_right_upper.set_position((
            body.body.position().x + body.body.size().x,
            body.body.position().y + body.head.radius() / 2.0,
        ));
        body.arm_right_upper.set_rotation(self.arm_right_upper.r);

        let arm_right_upper_end = figure_end_y(
            body.arm_right_upper.size().x,
            (
                body.arm_right_upper.position().x,
                body.arm_right_upper.position().y,
            ),
            body.arm_right_upper.rotation(),
        );
        body.arm_right_lower.set_position(arm_right_upper_end);
        body.arm_right_lower.set_rotation(self.arm_right_lower.r);

        //body end
        //leg left
        //+ body.body.body.size().y
        body.leg_left_upper.set_position((
            body.body.position().x,
            body.body.position().y + body.body.size().y,
        ));
        body.leg_left_upper.set_rotation(self.leg_left_upper.r);

        let leg_left_upper_end = figure_end_y(
            body.leg_left_upper.size().x,
            (
                body.leg_left_upper.position().x,
                body.leg_left_upper.position().y,
            ),
            body.leg_left_upper.rotation(),
        );
        body.leg_left_lower.set_position(leg_left_upper_end);
        body.leg_left_lower.set_rotation(self.leg_left_lower.r);

        //leg right
        body.leg_right_upper.set_position((
            body.body.position().x + body.body.size().x,
            body.body.position().y + body.body.size().y,
        ));
        body.leg_right_upper.set_rotation(self.leg_right_upper.r);
        let leg_right_upper_end = figure_end_y(
            body.leg_right_upper.size().x,
            (
                body.leg_right_upper.position().x,
                body.leg_right_upper.position().y,
            ),
            body.leg_right_upper.rotation(),
        );
        body.leg_right_lower.set_position(leg_right_upper_end);
        body.leg_right_lower.set_rotation(self.leg_left_lower.r);
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add for Rotation {
    type Output = Rotation;

    fn add(self, rhs: Self) -> Self::Output {
        Rotation::new(self.r + rhs.r)
    }
}
