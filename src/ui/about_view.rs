use sfml::system::Vector2f;

use super::buttons::ButtonGroup;

fn empty_callback() {
    print!("Fill me!");
}

fn init_about_buttons(window_size: Vector2f) -> ButtonGroup {
    let mut about_buttons = ButtonGroup::new();
    let help_size = Vector2f::new(90.0, 30.0);
    let help_position = Vector2f::new(
        window_size.x / 2.0 - help_size.x,
        window_size.y / 3.0 - help_size.y,
    );
    let help_label = "Справка";
    about_buttons.add_button(help_position, help_size, help_label, empty_callback);
    about_buttons
}
