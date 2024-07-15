use std::ops::Deref;
use std::string;

use sfml::graphics::{Font, RectangleShape, RenderTarget, Shape, Text, Transformable};
use sfml::system::SfStr;
use sfml::system::Vector2f;
use sfml::SfBox;

static FONT_PATH: &str = "resources/OpenSans-Regular.ttf";
type ButtonClickCallback = fn();

pub struct Button {
    shape: RectangleShape<'static>,
    position: Vector2f,
    label: &'static str,
    callback: ButtonClickCallback,
}

pub struct UI {
    buttons: Vec<Button>,
    about_label: &'static str,
    is_paused: bool,
    show_about: bool,
    current_pose: usize,
}

impl UI {
    pub fn new() -> Self {
        UI {
            buttons: Vec::new(),
            about_label: "About: This is a stickman animation app",
            is_paused: false,
            show_about: false,
            current_pose: 0,
        }
    }

    pub fn add_button(
        &mut self,
        position: Vector2f,
        size: Vector2f,
        label: &'static str,
        callback: ButtonClickCallback,
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
        for button in self.buttons.iter() {
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

        if self.show_about {
            let about_text = Text::new(self.about_label, &font, 20);
            window.draw(&about_text);
        }
    }
}
