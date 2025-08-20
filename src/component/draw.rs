use sfml::graphics::Drawable;

pub trait DrawComponent {
    fn get_drawables(&self) -> Vec<Box<dyn Drawable>>;
}
