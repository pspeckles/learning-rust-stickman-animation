use sfml::graphics::{
    Color, Drawable, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable,
};
use sfml::system::Vector2;

use crate::{
    component::{animation::AnimationState, position::PositionData},
    event::{EventNames, EventQueue},
};

const EVENT_QUEUE_NAME: &str = "DrawSystem";

pub struct DrawSystem {}

impl DrawSystem {
    pub fn new(event_queue: &mut Box<EventQueue>) -> DrawSystem {
        event_queue.subscribe(EVENT_QUEUE_NAME);
        DrawSystem {}
    }

    pub fn draw(
        &self,
        event_queue: &mut Box<EventQueue>,
        render_target: &mut RenderWindow,
        entities: Vec<&AnimationState>,
    ) {
        for entity in entities {
            entity.last_frame().pose.traverse().iter().for_each(|node| {
                let obj = &DrawSystem::to_drawable(node.get());
                render_target.draw(obj);
                // render_target.draw(&DrawSystem::to_joint(node.get()));
            })
        }
        for elem in event_queue.poll(EVENT_QUEUE_NAME) {
            if elem.name == EventNames::About {
                todo!()
            } else if elem.name == EventNames::Main {
                todo!()
            }
        }
    }

    fn to_drawable(position: &PositionData) -> impl Drawable {
        let mut rect = RectangleShape::new();
        rect.set_size(Vector2::new(position.width as f32, position.height as f32));
        rect.set_position(Vector2::new(position.point.x(), position.point.y()));
        rect.set_rotation(position.angle.r);
        rect.set_outline_thickness(1.0);
        rect.set_outline_color(Color::GREEN);
        rect
    }

    fn to_joint(position: &PositionData) -> impl Drawable {
        let mut joint = RectangleShape::with_size(Vector2 {
            x: position.width as f32,
            y: position.width as f32,
        });
        joint.set_position(Vector2::new(position.point.x(), position.point.y()));
        joint.set_rotation(position.angle.r);
        joint.set_fill_color(Color::YELLOW);
        joint.set_outline_thickness(1.0);
        joint.set_outline_color(Color::CYAN);
        joint
    }
}
