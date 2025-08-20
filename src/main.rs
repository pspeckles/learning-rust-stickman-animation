use std::time::Instant;

use actor::human::Human;
use component::animation::AnimationComponent;
use component::draw::DrawComponent;
use sfml::graphics::Color;
use sfml::graphics::{RenderTarget, RenderWindow};
use sfml::system::Vector2f;
use sfml::window::{mouse, Event, Style};

mod actor;
mod component;
mod event;
mod system;

mod ui;
use event::EventQueue;
use system::animation::AnimationSystem;
use system::draw::DrawSystem;
use ui::buttons::ButtonGroup;

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "Stickman Animation",
        Style::CLOSE,
        &Default::default(),
    )
    .unwrap();
    window.set_vertical_sync_enabled(true);

    //
    let sk = Human::new();

    let mut entities = vec![sk];
    let mut main_buttons = ButtonGroup::new();
    main_buttons.add_button(
        Vector2f::new(10.0, 10.0),
        Vector2f::new(100.0, 40.0),
        "Squat",
        Box::new(|| println!("squat")), // Box::new(move || squat.set_animation(1)),
    );

    main_buttons.add_button(
        (130.0, 10.0).into(),
        (100.0, 40.0).into(),
        "Stand",
        Box::new(|| println!("Stand")), // Box::new(move || stand.set_animation(0)),
    );
    //
    // Animation variables
    let mut dt: u128 = 0;
    let mut now = Instant::now();
    let mut events = Box::new(EventQueue::new());
    let animation_system = AnimationSystem::new(&mut events);
    let draw_system = DrawSystem::new(&mut events);

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
        {
            let mut animatables: Vec<&mut dyn AnimationComponent> = vec![];
            for h in &mut entities {
                animatables.push(h as &mut dyn AnimationComponent);
            }
            animation_system.apply(&mut events, animatables, &dt);
        }
        // Clear the window
        window.clear(Color::BLACK);

        // Draw
        {
            draw_system.draw(
                &mut events,
                &mut window,
                entities
                    .iter()
                    .map(|e| e as &dyn DrawComponent)
                    .collect::<Vec<&dyn DrawComponent>>(),
            );
        }
        main_buttons.draw(&mut window);
        window.display();
        dt = now.elapsed().as_millis();
        now = Instant::now();
    }
}
