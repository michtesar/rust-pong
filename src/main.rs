use raylib::ffi::KeyboardKey::{KEY_DOWN, KEY_UP};
use raylib::prelude::*;
use std::ops::AddAssign;
use std::time::{Duration, Instant};

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

#[derive(Clone, Copy)]
struct Ball {
    position: Vector2,
    velocity: Vector2,
    color: Color,
    radius: f32,
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

struct Paddle {
    position: Vector2,
    velocity: Vector2,
    size: Vector2,
    color: Color,
    acceleration: f32,
}

impl Paddle {
    pub fn new() -> Self {
        Self {
            position: Vector2 {
                x: 10.0,
                y: (SCREEN_HEIGHT / 2) as f32,
            },
            velocity: Vector2 { x: 0.0, y: 0.0 },
            color: Color::WHITE,
            size: Vector2 {
                x: SCREEN_WIDTH as f32 / 100.0,
                y: SCREEN_WIDTH as f32 / 10.0,
            },
            acceleration: 0.25,
        }
    }
}

struct Score {
    user: i32,
    cpu: i32,
}

fn main() {
    let cpu_paddle_reaction_time = Duration::from_secs_f64(1.5); // Adjust the reaction time as needed (in seconds)
    let last_cpu_update_time = Instant::now();

    let (mut rl, thread) = init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Pong")
        .vsync()
        .build();

    let mut score = Score { user: 0, cpu: 0 };

    let mut ball: Ball = Ball::new();
    ball.position.x = (SCREEN_WIDTH / 2) as f32;
    ball.position.y = (SCREEN_HEIGHT / 2) as f32;

    let mut user_paddle: Paddle = Paddle::new();
    user_paddle.position.y -= user_paddle.size.y / 2.0;

    let mut cpu_paddle: Paddle = Paddle::new();
    cpu_paddle.position.x = SCREEN_WIDTH as f32 - cpu_paddle.size.x * 2.0;
    cpu_paddle.position.y -= cpu_paddle.size.y / 2.0;
    cpu_paddle.acceleration = 0.4;

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        // Draw playground
        d.draw_line(
            SCREEN_WIDTH / 2,
            0,
            SCREEN_WIDTH / 2,
            SCREEN_HEIGHT,
            Color::WHITE,
        );
        d.draw_circle_lines(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, 100.0, Color::WHITE);

        // Draw a ball
        ball.position.add_assign(ball.velocity);
        d.draw_circle_v(ball.position, 10.0, ball.color);
        // if ball.position.x > (SCREEN_WIDTH as f32 - ball.radius) || ball.position.x < ball.radius {
        //     ball.velocity.x *= -1.0;
        // }
        if ball.position.y > (SCREEN_HEIGHT as f32 - ball.radius) || ball.position.y < ball.radius {
            ball.velocity.y *= -1.0;
        }

        // Draw user paddle
        d.draw_rectangle_v(user_paddle.position, user_paddle.size, user_paddle.color);

        if d.is_key_down(KEY_UP) {
            user_paddle.velocity.y -= user_paddle.acceleration;
        }
        if d.is_key_down(KEY_DOWN) {
            user_paddle.velocity.y += user_paddle.acceleration;
        }
        if d.is_key_released(KEY_UP) || d.is_key_released(KEY_DOWN) {
            user_paddle.velocity.x = 0.0;
            user_paddle.velocity.y = 0.0;
        }

        user_paddle.position.add_assign(user_paddle.velocity);

        // Clip the paddle on the screen
        if user_paddle.position.y > SCREEN_HEIGHT as f32 - user_paddle.size.y {
            user_paddle.position.y = SCREEN_HEIGHT as f32 - user_paddle.size.y;
        }
        if user_paddle.position.y < 0.0 {
            user_paddle.position.y = 0.0;
        }

        let current_time = Instant::now();
        let elapsed_time = current_time.duration_since(last_cpu_update_time);
        d.draw_rectangle_v(cpu_paddle.position, cpu_paddle.size, cpu_paddle.color);
        if elapsed_time >= cpu_paddle_reaction_time {
            let ball_speed = ball.velocity.length();
            let paddle_y: f32 = cpu_paddle.position.y + cpu_paddle.size.y / 2.0;
            let ball_y: f32 = ball.position.y;
            let dist: f32 = paddle_y - ball_y;
            if dist.abs() > ball_speed {
                if dist > 0.0 {
                    cpu_paddle.velocity.y -= cpu_paddle.acceleration;
                } else {
                    cpu_paddle.velocity.y += cpu_paddle.acceleration;
                }
            } else {
                cpu_paddle.velocity.y = 0.0;
            }

            cpu_paddle.position.add_assign(cpu_paddle.velocity);

            // Clip the CPU paddle on the screen
            if cpu_paddle.position.y > SCREEN_HEIGHT as f32 - cpu_paddle.size.y {
                cpu_paddle.position.y = SCREEN_HEIGHT as f32 - cpu_paddle.size.y;
            }
            if cpu_paddle.position.y < 0.0 {
                cpu_paddle.position.y = 0.0;
            }
        }

        // Check score and reset the game
        if ball.position.x > SCREEN_WIDTH as f32 {
            score.user += 1;
            ball = Ball::new();
            ball.position.x = (SCREEN_WIDTH / 2) as f32;
            ball.position.y = (SCREEN_HEIGHT / 2) as f32;
        }
        if ball.position.x < 0.0 {
            score.cpu += 1;
            ball = Ball::new();
            ball.position.x = (SCREEN_WIDTH / 2) as f32;
            ball.position.y = (SCREEN_HEIGHT / 2) as f32;
        }

        // Check the paddle collisions
        let cpu_paddle_rect: Rectangle = Rectangle::new(
            cpu_paddle.position.x,
            cpu_paddle.position.y,
            cpu_paddle.size.x,
            cpu_paddle.size.y,
        );
        if cpu_paddle_rect.check_collision_circle_rec(ball.position, ball.radius) {
            ball.velocity.x *= -1.0;
        }
        let user_paddle_rect: Rectangle = Rectangle::new(
            user_paddle.position.x,
            user_paddle.position.y,
            user_paddle.size.x,
            user_paddle.size.y,
        );
        if user_paddle_rect.check_collision_circle_rec(ball.position, ball.radius) {
            ball.velocity.x *= -1.0;
        }

        // Draw score
        let user_score: String = format!("User: {}", score.user);
        d.draw_text(&*user_score, 12, 12, 20, Color::WHITE);
        let cpu_score: String = format!("CPU: {}", score.cpu);
        d.draw_text(&*cpu_score, SCREEN_WIDTH - 100, 12, 20, Color::WHITE);

        // d.draw_fps(10, 10);
    }
}
