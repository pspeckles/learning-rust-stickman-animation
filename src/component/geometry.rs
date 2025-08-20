use std::f32::consts::PI;

pub fn figure_end_x(size: f32, position: (f32, f32), rotation: f32) -> (f32, f32) {
    let x = position.0 - (size * (PI / 180.0 * rotation).sin());
    let y = position.1 + (size * (PI / 180.0 * rotation).cos());
    (x, y)
}
