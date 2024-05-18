use std::{error, time::Duration};

use ratatui::{
    style::Color,
    widgets::canvas::{self, Circle, Points, Shape},
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Default)]
pub struct Player {
    pub pos_x: f64,
    pub pos_y: f64,
    pub face_x: f64,
    pub face_y: f64,
    pub hp: isize,
    pub max_hp: isize,
    pub mp: isize,
    pub max_mp: isize,
}

impl Player {
    pub fn walk(&mut self, delta_x: f64, delta_y: f64) -> AppResult<()> {
        self.pos_x += delta_x;
        self.pos_y += delta_y;

        self.face_x = delta_x;
        self.face_y = delta_y;

        Ok(())
    }

    pub fn new_bullet(&self) -> Bullet {
        const BULLET_VELOCITY: f64 = 8.;
        const BULLET_OFFSET: f64 = 1.5;
        Bullet {
            pos_x: self.pos_x + self.face_x * BULLET_OFFSET,
            pos_y: self.pos_y + self.face_y * BULLET_OFFSET,
            velocity_x: self.face_x * BULLET_VELOCITY,
            velocity_y: self.face_y * BULLET_VELOCITY,
        }
    }
}

impl Shape for Player {
    fn draw(&self, painter: &mut canvas::Painter) {
        let circle = Circle {
            x: self.pos_x,
            y: self.pos_y,
            radius: 2.,
            color: Color::White,
        };
        circle.draw(painter);
    }
}

#[derive(Debug)]
pub struct Bullet {
    pub pos_x: f64,
    pub pos_y: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
}

impl Shape for Bullet {
    fn draw(&self, painter: &mut canvas::Painter) {
        let points = Points {
            coords: &[(self.pos_x, self.pos_y)],
            color: Color::Yellow,
        };
        points.draw(painter);
    }
}

#[derive(Debug)]
pub struct GameLog(pub String);

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub player: Player,
    pub stage_index: usize,
    pub logs: Vec<GameLog>,
    pub bullets: Vec<Bullet>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            player: Player::default(),
            stage_index: 0,
            logs: vec![],
            bullets: vec![],
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self, delta: Duration) {
        for b in &mut self.bullets {
            b.pos_x += b.velocity_x * delta.as_secs_f64();
            b.pos_y += b.velocity_y * delta.as_secs_f64();

            // TODO: check collision
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
