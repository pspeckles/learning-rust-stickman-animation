use std::rc::Rc;

use actor::skeleton::Skeleton;
use actor::squatting_animation::{self, squatting_animation};
use sfml::graphics::Color;
use sfml::graphics::{RenderTarget, RenderWindow};
use sfml::system::Vector2f;
use sfml::window::{mouse, Event, Style};

mod actor;
mod animation_controller;

mod ui;
use ui::buttons::ButtonGroup;

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "Stickman Animation",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    let sk = Skeleton::new((300.0, 200.0), 120.0, 2.0);
    let figure = Rc::new(sk);
    let mut main_buttons = ButtonGroup::new();
    let squat = figure.clone();
    main_buttons.add_button(
        Vector2f::new(10.0, 10.0),
        Vector2f::new(100.0, 40.0),
        "Squat",
        Box::new(move || squat.set_animation(1)),
    );

    let stand = figure.clone();
    main_buttons.add_button(
        (130.0, 10.0).into(),
        (100.0, 40.0).into(),
        "Stand",
        Box::new(move || stand.set_animation(0)),
    );
    //
    // Animation variables
    let mut frame = 0;

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::MouseButtonPressed { button, x, y } => {
                    if button == mouse::Button::Left {
                        println!("{:?}", button);
                        main_buttons.handle_click(Vector2f::new(x as f32, y as f32));
                    }
                }
                _ => {}
            }
        }

        // Update animation frame
        frame = (frame + 1) % 60;
        if frame % 10 == 0 {
            figure.animate();
        }
        // Clear the window
        window.clear(Color::BLACK);

        // Draw
        figure.draw(&mut window);
        main_buttons.draw(&mut window);
        window.display();
    }
}
