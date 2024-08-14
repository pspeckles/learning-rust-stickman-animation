use crate::stickman::position::Position;

#[derive(Debug)]
pub struct Animation<T: Position> {
    frames: Vec<T>,
    latest_frame: usize,
}

impl<T: Position> Animation<T> {
    pub fn new(frames: Vec<T>) -> Animation<T> {
        Animation {
            frames,
            latest_frame: 0,
        }
    }

    pub fn next_frame(&mut self) -> &T {
        if self.frames.len() == self.latest_frame {
            self.latest_frame = 0;
        }
        let frame = &self.frames[self.latest_frame];
        self.latest_frame += 1;

        frame
    }

    pub fn reset(&mut self) {
        self.latest_frame = 0;
    }
}
