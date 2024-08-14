use super::pose::Point;

pub trait Position {
    type Positionable;

    fn apply_to(&self, object: &mut Self::Positionable, point: &Point);
}
