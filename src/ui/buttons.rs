use sfml::graphics::{Font, RectangleShape, RenderTarget, Shape, Text, Transformable};
use sfml::system::Vector2f;

use crate::actor::animation::AnimationFrames;
use crate::animation_controller::AnimationController;

static FONT_PATH: &str = "resources/OpenSans-Regular.ttf";
// type ButtonClickCallback = fn();
// type ButtonClickCallback = FnMut();

pub struct Button {
    shape: RectangleShape<'static>,
    position: Vector2f,
    label: &'static str,
    callback: Box<dyn Fn()>,
}

pub struct ButtonGroup {
    buttons: Vec<Button>,
}

impl ButtonGroup {
    pub fn new() -> Self {
        ButtonGroup {
            buttons: Vec::new(),
        }
    }

    pub fn add_button(
        &mut self,
        position: Vector2f,
        size: Vector2f,
        label: &'static str,
        callback: Box<dyn Fn()>,
    ) {
        let mut shape = RectangleShape::new();
        shape.set_position(position);
        shape.set_size(size);
        shape.set_fill_color(sfml::graphics::Color::WHITE);

        self.buttons.push(Button {
            shape,
            position,
            label,
            callback,
        });
    }

    pub fn handle_click(&mut self, mouse_pos: Vector2f) {
        for button in self.buttons.iter_mut() {
            if button.shape.global_bounds().contains(mouse_pos) {
                (button.callback)();
            }
        }
    }

    pub fn draw(&self, window: &mut sfml::graphics::RenderWindow) {
        let font = Font::from_file(FONT_PATH).unwrap();
        for button in &self.buttons {
            let mut text = Text::new("", &font, 20);
            text.set_position(button.position + Vector2f::new(10.0, 10.0));
            text.set_fill_color(sfml::graphics::Color::BLACK);
            text.set_string(button.label);

            window.draw(&button.shape);
            window.draw(&text);
        }
    }
}
