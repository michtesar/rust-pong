use raylib::{color::Color, math::Vector2};

use crate::config;

pub struct Paddle {
    pub position: Vector2,
    pub velocity: Vector2,
    pub size: Vector2,
    pub color: Color,
    pub acceleration: f32,
}

impl Paddle {
    pub fn new() -> Self {
        Self {
            position: Vector2 {
                x: 10.0,
                y: (config::SCREEN_HEIGHT / 2) as f32,
            },
            velocity: Vector2 { x: 0.0, y: 0.0 },
            color: Color::WHITE,
            size: Vector2 {
                x: config::SCREEN_WIDTH as f32 / 100.0,
                y: config::SCREEN_WIDTH as f32 / 10.0,
            },
            acceleration: 0.25,
        }
    }
}
