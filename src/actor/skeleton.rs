use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Cell, RefCell, RefMut},
    rc::Rc,
};

use sfml::{
    graphics::{
        CircleShape, Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable,
    },
    system::Vector2f,
};

use crate::animation_controller::AnimationController;

use super::{
    actor_pose::ActorPose, draw::Draw, geometry::figure_end_y, pose::Pose, position::Position,
    squatting_animation::squatting_animation, standing_animation::standing_animation,
};

type BodyPart = Box<RectangleShape<'static>>;
#[derive(Debug, Clone)]
pub struct RenderingParts {
    head: Box<CircleShape<'static>>,
    body: Box<RectangleShape<'static>>,
    arm_right_upper: BodyPart,
    arm_right_lower: BodyPart,
    arm_left_upper: BodyPart,
    arm_left_lower: BodyPart,
    leg_right_upper: BodyPart,
    leg_right_lower: BodyPart,
    leg_left_upper: BodyPart,
    leg_left_lower: BodyPart,
}

#[derive(Debug, Clone)]
pub struct Skeleton {
    parts: RefCell<RenderingParts>,
    animations: Vec<AnimationController>,
    current_animation: Cell<usize>,
}

impl Skeleton {
    pub fn new(position: (f32, f32), length: f32, width: f32) -> Self {
        let head_size = head_size_from(width);
        let body_width = body_width_from(width);
        let body_height = body_height_from(length);
        let limb_length = limb_length_from(length);
        let limb_width = limb_width_from(width);
        let mut head = head_shape(head_size);
        head.set_position((position.0, position.1));
        let animations = init_animations();
        let parts = RenderingParts {
            head,
            body: body_shape(body_width, body_height),
            arm_right_upper: limb_shape(limb_length, limb_width),
            arm_right_lower: limb_shape(limb_length, limb_width),
            arm_left_upper: limb_shape_mirrored(limb_length, limb_width),
            arm_left_lower: limb_shape_mirrored(limb_length, limb_width),
            leg_right_upper: limb_shape(limb_length, limb_width),
            leg_right_lower: limb_shape(limb_length, limb_width),
            leg_left_upper: limb_shape_mirrored(limb_length, limb_width),
            leg_left_lower: limb_shape_mirrored(limb_length, limb_width),
        };

        Self {
            parts: RefCell::new(parts),
            animations,
            current_animation: Cell::new(0),
        }
    }

    pub fn set_animation(&self, animation: usize) {
        self.current_animation.set(animation)
    }

    pub fn animate(&self) {
        let animation = self.animations.get(self.current_animation.get()).unwrap();
        self.parts.borrow_mut().into_pose(animation.animate());
    }

    pub fn draw(&self, window: &mut RenderWindow) {
        self.parts.borrow_mut().draw(window);
    }
}

fn init_animations() -> Vec<AnimationController> {
    let squatting = AnimationController::new(squatting_animation());
    let standing_animation = AnimationController::new(standing_animation());

    return vec![standing_animation, squatting];
}

impl Draw for RenderingParts {
    fn draw(&self, window: &mut RenderWindow) {
        window.draw(&*self.head);
        window.draw(&*self.body);
        window.draw(&*self.arm_right_upper);
        window.draw(&*self.arm_right_lower);
        window.draw(&*self.leg_right_upper);
        window.draw(&*self.leg_right_lower);
        window.draw(&*self.leg_left_upper);
        window.draw(&*self.leg_left_lower);
        window.draw(&*self.arm_left_upper);
        window.draw(&*self.arm_left_lower);
    }
}

fn head_shape(head_size: f32) -> Box<CircleShape<'static>> {
    let mut head = CircleShape::new(head_size, 36);
    head.set_fill_color(Color::WHITE);
    Box::new(head)
}

fn body_shape(body_width: f32, body_height: f32) -> Box<RectangleShape<'static>> {
    let mut shape = RectangleShape::with_size(Vector2f::new(body_width, body_height));
    shape.set_fill_color(Color::WHITE);
    Box::new(shape)
}

fn limb_shape(length: f32, width: f32) -> Box<RectangleShape<'static>> {
    let mut part = RectangleShape::with_size(Vector2f::new(length, width));
    part.set_fill_color(Color::MAGENTA);
    Box::new(part)
}

fn limb_shape_mirrored(length: f32, width: f32) -> Box<RectangleShape<'static>> {
    let mut part = RectangleShape::with_size(Vector2f::new(length, width));
    part.set_fill_color(Color::BLUE);
    part.set_origin(Vector2f::new(0.0, width));
    Box::new(part)
}

fn head_size_from(width: f32) -> f32 {
    width * 20.0
}

fn body_width_from(width: f32) -> f32 {
    width * 1.5
}
fn body_height_from(length: f32) -> f32 {
    length * 1.2
}
fn limb_length_from(length: f32) -> f32 {
    length * 0.5
}
fn limb_width_from(width: f32) -> f32 {
    width
}

impl Position for RenderingParts {
    fn into_pose(&mut self, pose: &ActorPose) {
        let skeleton_pose = Pose::new(pose.position_shift, &pose.joints);
        let parts = self;
        let head = &mut parts.head;
        head.set_position((
            head.position().x + pose.position_shift.x(),
            head.position().y + pose.position_shift.y(),
        ));
        //body
        let body = &mut parts.body;
        body.set_position((
            head.position().x + head.radius() - (body.size().x / 2.0),
            head.position().y + (head.radius() * 2.0) - 5.0,
        ));
        //arm left
        let arm_left_upper = &mut parts.arm_left_upper;
        arm_left_upper.set_position((body.position().x, body.position().y + head.radius() / 2.0));
        arm_left_upper.set_rotation(skeleton_pose.arm_left_upper.r);

        let arm_left_upper_end = figure_end_y(
            arm_left_upper.size().x,
            (arm_left_upper.position().x, arm_left_upper.position().y),
            arm_left_upper.rotation(),
        );
        let arm_left_lower = &mut parts.arm_left_lower;
        arm_left_lower.set_position(arm_left_upper_end);
        let arm_left_lower_joint = skeleton_pose.arm_left_lower;
        arm_left_lower.set_rotation(arm_left_lower_joint.r);

        //arm right
        let arm_right_upper = &mut parts.arm_right_upper;
        arm_right_upper.set_position((
            body.position().x + body.size().x,
            body.position().y + head.radius() / 2.0,
        ));
        arm_right_upper.set_rotation(skeleton_pose.arm_right_upper.r);

        let arm_right_upper_end = figure_end_y(
            arm_right_upper.size().x,
            (arm_right_upper.position().x, arm_right_upper.position().y),
            arm_right_upper.rotation(),
        );
        let arm_right_lower = &mut parts.arm_right_lower;
        arm_right_lower.set_position(arm_right_upper_end);
        arm_right_lower.set_rotation(skeleton_pose.arm_right_lower.r);

        //body end
        //leg left
        //+ body.body.body.size().y
        let leg_left_upper = &mut parts.leg_left_upper;
        leg_left_upper.set_position((body.position().x, body.position().y + body.size().y));
        leg_left_upper.set_rotation(skeleton_pose.leg_left_upper.r);

        let leg_left_upper_end = figure_end_y(
            leg_left_upper.size().x,
            (leg_left_upper.position().x, leg_left_upper.position().y),
            leg_left_upper.rotation(),
        );
        let leg_left_lower = &mut parts.leg_left_lower;
        leg_left_lower.set_position(leg_left_upper_end);
        leg_left_lower.set_rotation(skeleton_pose.leg_left_lower.r);

        //leg right
        let leg_right_upper = &mut parts.leg_right_upper;
        leg_right_upper.set_position((
            body.position().x + body.size().x,
            body.position().y + body.size().y,
        ));
        leg_right_upper.set_rotation(skeleton_pose.leg_right_upper.r);
        let leg_right_upper_end = figure_end_y(
            leg_right_upper.size().x,
            (leg_right_upper.position().x, leg_right_upper.position().y),
            leg_right_upper.rotation(),
        );
        let leg_right_lower = &mut parts.leg_right_lower;
        leg_right_lower.set_position(leg_right_upper_end);
        leg_right_lower.set_rotation(skeleton_pose.leg_right_lower.r);
    }
}
