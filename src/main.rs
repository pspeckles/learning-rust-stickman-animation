use sfml::graphics::{Color, Transformable};
use sfml::graphics::{RenderTarget, RenderWindow};
use sfml::system::Vector2f;
use sfml::window::{mouse, Event, Style};

mod stickman;
use stickman::animation::Animation;
use stickman::pose::{figure_end_y, Point, Pose, Rotation};
use stickman::position::Position;
use stickman::prop::{Flag, FlagPosition};
use stickman::standing_animation::standing_animation;

mod animation_controller;
use animation_controller::AnimationController;

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

    let figure = &mut stickman::skeleton::Skeleton::new(120.0, 2.0);
    let standing_still = standing_animation();
    let mut animator = AnimationController::new(standing_still);

    let mut main_buttons = ButtonGroup::new();
    main_buttons.add_button(
        Vector2f::new(10.0, 10.0),
        Vector2f::new(100.0, 40.0),
        "Pause",
        test_print,
    );

    fn test_print() {
        println!("Pause")
    }
    // ui.add_button(
    //     Vector2f::new(120.0, 10.0),
    //     Vector2f::new(100.0, 40.0),
    //     "Change Pose",
    // );
    // ui.add_button(
    //     Vector2f::new(230.0, 10.0),
    //     Vector2f::new(100.0, 40.0),
    //     "About",
    // );

    let left_flag = &mut Flag::new(30.0, 40.0);
    let left_flag_position = FlagPosition::new(270.0);
    let left_flag_position_2 = FlagPosition::new(250.0);
    let left_flag_waving = Animation::new(vec![left_flag_position, left_flag_position_2]);
    let mut left_flag_animator = AnimationController::new(left_flag_waving);

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
            animator.animate(figure, &Point::new(300.0, 200.0));

            let left_flag_start_point = figure_end_y(
                figure.arm_left_lower.size().x,
                (
                    figure.arm_left_lower.position().x,
                    figure.arm_left_lower.position().y,
                ),
                figure.arm_left_lower.rotation(),
            );
            left_flag_animator.animate(left_flag, &Point::from_tuple(left_flag_start_point));
        }
        // Clear the window
        window.clear(Color::BLACK);

        // Draw stickman
        figure.draw(&mut window);
        left_flag.draw(&mut window);
        main_buttons.draw(&mut window);
        window.display();
    }
}
