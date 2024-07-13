use sfml::graphics::{Color, Transformable};
use sfml::graphics::{RenderTarget, RenderWindow};
use sfml::window::{Event, Style};

mod stickman;
use stickman::animation::Animation;
use stickman::pose::{figure_end_y, Point, Pose, Rotation};

mod animation_controller;
use animation_controller::AnimationController;
use stickman::position::Position;
use stickman::prop::{Flag, FlagPosition};

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "Stickman Animation",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);
    let standing_pose_1 = Pose {
        position_shift: Point::new(0.0, 0.0),
        arm_left_upper: Rotation::new(-20.0),
        arm_left_lower: Rotation::new(-30.0),
        arm_right_upper: Rotation::new(150.0),
        arm_right_lower: Rotation::new(50.0),
        leg_left_upper: Rotation::new(130.0),
        leg_left_lower: Rotation::new(70.0),
        leg_right_upper: Rotation::new(70.0),
        leg_right_lower: Rotation::new(30.0),
    };
    let standing_pose_2 = Pose {
        position_shift: Point::new(0.0, 0.0),
        arm_left_upper: Rotation::new(-20.0),
        arm_left_lower: Rotation::new(-35.0),
        arm_right_upper: Rotation::new(140.0),
        arm_right_lower: Rotation::new(40.0),
        leg_left_upper: Rotation::new(120.0),
        leg_left_lower: Rotation::new(80.0),
        leg_right_upper: Rotation::new(60.0),
        leg_right_lower: Rotation::new(40.0),
    };
    let figure = &mut stickman::skeleton::Skeleton::new(120.0, 2.0);
    let standing_still = Animation::new(vec![standing_pose_1, standing_pose_2]);
    let mut animator = AnimationController::new(standing_still);

    let left_flag = &mut Flag::new(30.0, 40.0);
    let left_flag_position = FlagPosition::new(270.0);
    let left_flag_position_2 = FlagPosition::new(250.0);
    let left_flag_waving = Animation::new(vec![left_flag_position, left_flag_position_2]);
    let mut left_flag_animator = AnimationController::new(left_flag_waving);
    // // Animation variables
    let mut frame = 0;
    // let mut breathing_offset: f32 = 0.0;
    // let mut breathing_direction = 1.0;
    // let breathing_speed: f32 = 0.2;
    //
    // let breathing_offset_max = 3.0;
    // let breathing_offset_min = -2.0;
    // let breathing_rotation_multiplier = 1.5;
    // sfml::system::sleep(Time::seconds(10.0));
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }
        }
        // Update animation frame
        frame = (frame + 1) % 60;
        //
        // // Breathing effect
        if frame % 10 == 0 {
            animator.animate(figure, &Point::new(300.0, 300.0));

            let left_flag_start_point = figure_end_y(
                figure.arm_left_lower.size().x,
                (
                    figure.arm_left_lower.position().x,
                    figure.arm_left_lower.position().y,
                ),
                figure.arm_left_lower.rotation(),
            );
            left_flag_animator.animate(left_flag, &Point::from_tuple(left_flag_start_point));

            //     breathing_offset += (breathing_speed + (breathing_speed * breathing_offset.abs()))
            //         * breathing_direction;
            //     if breathing_offset >= breathing_offset_max {
            //         breathing_direction = -1.0;
            //         breathing_offset = breathing_offset_max;
            //     } else if breathing_offset < breathing_offset_min {
            //         breathing_direction = 1.0;
            //         breathing_offset = breathing_offset_min;
            //     }
        }
        //
        // // Update stickman position
        // head.set_position((figure.head_pos().0, figure.head_pos().1 + breathing_offset));
        // body.set_position((
        //     figure.body_position().0,
        //     figure.body_position().1 + breathing_offset,
        // ));
        //
        // arm_left.set_position((
        //     figure.left_arm_pos().0,
        //     figure.left_arm_pos().1 + breathing_offset,
        // ));
        // arm_left.set_rotation(30.0 + breathing_offset * breathing_rotation_multiplier);
        //
        // arm_right.set_position((
        //     figure.righ_arm_pos().0,
        //     figure.righ_arm_pos().1 + breathing_offset,
        // ));
        // arm_right.set_rotation(-30.0 - breathing_offset * breathing_rotation_multiplier);
        //
        // leg_left.set_position((figure.left_leg_pos().0, figure.left_leg_pos().1));
        // leg_left.set_rotation(25.0 + breathing_offset * breathing_rotation_multiplier);
        //
        // leg_right.set_position((figure.right_leg_pos().0, figure.right_leg_pos().1));
        // leg_right.set_rotation(-25.0 - breathing_offset * breathing_rotation_multiplier);
        //
        // // Clear the window
        window.clear(Color::BLACK);

        // Draw stickman
        figure.draw(&mut window);
        left_flag.draw(&mut window);
        window.display();
    }
}
