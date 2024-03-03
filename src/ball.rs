use raylib::{color::Color, math::Vector2};

#[derive(Clone, Copy)]
pub(crate) struct Ball {
    pub position: Vector2,
    pub velocity: Vector2,
    pub color: Color,
    pub radius: f32,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            position: Vector2 { x: 0.0, y: 0.0 },
            velocity: Vector2 { x: 5.0, y: 5.0 },
            color: Color::WHITE,
            radius: 10.0,
        }
    }
}
