use sfml::graphics::RenderWindow;

pub trait Draw {
    fn draw(&self, window: &mut RenderWindow);
}
