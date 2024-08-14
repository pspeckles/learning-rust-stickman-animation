use sfml::{
    graphics::{
        CircleShape, Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable,
    },
    system::Vector2f,
    window::{self, Window},
};

#[derive(Debug)]
pub struct Skeleton {
    pub head: CircleShape<'static>,
    pub body: RectangleShape<'static>,
    pub arm_right_upper: RectangleShape<'static>,
    pub arm_right_lower: RectangleShape<'static>,
    pub arm_left_upper: RectangleShape<'static>,
    pub arm_left_lower: RectangleShape<'static>,
    pub leg_right_upper: RectangleShape<'static>,
    pub leg_right_lower: RectangleShape<'static>,
    pub leg_left_upper: RectangleShape<'static>,
    pub leg_left_lower: RectangleShape<'static>,
}

impl Skeleton {
    pub fn new(length: f32, width: f32) -> Self {
        let head_size = head_size_from(width);
        let body_width = body_width_from(width);
        let body_height = body_height_from(length);
        let limb_length = limb_length_from(length);
        let limb_width = limb_width_from(width);
        Skeleton {
            head: head_shape(head_size),
            body: body_shape(body_width, body_height),
            arm_right_upper: limb_shape(limb_length, limb_width),
            arm_right_lower: limb_shape(limb_length, limb_width),
            arm_left_upper: limb_shape_mirrored(limb_length, limb_width),
            arm_left_lower: limb_shape_mirrored(limb_length, limb_width),
            leg_right_upper: limb_shape(limb_length, limb_width),
            leg_right_lower: limb_shape(limb_length, limb_width),
            leg_left_upper: limb_shape_mirrored(limb_length, limb_width),
            leg_left_lower: limb_shape_mirrored(limb_length, limb_width),
        }
    }
    pub fn draw(&self, window: &mut RenderWindow) {
        window.draw(&self.head);
        window.draw(&self.head);
        window.draw(&self.body);
        window.draw(&self.arm_left_upper);
        window.draw(&self.arm_left_lower);
        window.draw(&self.arm_right_upper);
        window.draw(&self.arm_right_lower);
        window.draw(&self.leg_left_upper);
        window.draw(&self.leg_left_lower);
        window.draw(&self.leg_right_upper);
        window.draw(&self.leg_right_lower);
    }
}

fn head_shape<'a>(head_size: f32) -> CircleShape<'a> {
    let mut head = CircleShape::new(head_size, 36);
    head.set_fill_color(Color::WHITE);
    head
}

fn body_shape<'a>(body_width: f32, body_height: f32) -> RectangleShape<'a> {
    let mut shape = RectangleShape::with_size(Vector2f::new(body_width, body_height));
    shape.set_fill_color(Color::WHITE);
    shape
}
//     }

fn limb_shape<'a>(length: f32, width: f32) -> RectangleShape<'a> {
    let mut part = RectangleShape::with_size(Vector2f::new(length, width));
    part.set_fill_color(Color::MAGENTA);
    part
}

fn limb_shape_mirrored<'a>(length: f32, width: f32) -> RectangleShape<'a> {
    let mut part = RectangleShape::with_size(Vector2f::new(length, width));
    part.set_fill_color(Color::BLUE);
    part.set_origin(Vector2f::new(0.0, width));
    part
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
