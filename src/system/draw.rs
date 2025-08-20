use sfml::graphics::{RenderTarget, RenderWindow};

use crate::component::draw::DrawComponent;
use crate::event::{EventNames, EventQueue};

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
        entities: Vec<&dyn DrawComponent>,
    ) {
        for entity in entities {
            entity
                .get_drawables()
                .into_boxed_slice()
                .iter()
                .for_each(|drawable| {
                    render_target.draw(&**drawable);
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
}
