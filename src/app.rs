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
    pub move_velocity: f64,
}

/// In-game events
#[derive(Debug)]
pub enum GameEvent {
    PlayerMove(f64, f64),
    Shoot,
}

fn norm(x: f64, y: f64) -> (f64, f64) {
    const EPSILON: f64 = 0.0001;
    let m = x * x + y * y;
    if m < EPSILON {
        return (0., 0.);
    }
    let m = m.sqrt();
    (x / m, y / m)
}

impl Player {
    pub fn walk(&mut self, delta_x: f64, delta_y: f64) -> AppResult<()> {
        self.pos_x += delta_x;
        self.pos_y += delta_y;

        let (delta_x, delta_y) = norm(delta_x, delta_y);
        if (delta_x + delta_y).abs() > 0. {
            self.face_x = delta_x;
            self.face_y = delta_y;
        }

        Ok(())
    }

    pub fn new_bullet(&self) -> Bullet {
        const BULLET_VELOCITY: f64 = 12.;
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
    pub events: Vec<GameEvent>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            player: Player {
                move_velocity: 6.,
                face_x: 1.,
                ..Player::default()
            },
            stage_index: 0,
            logs: vec![],
            bullets: vec![],
            events: vec![],
        }
    }
}

impl App {
    pub const CHAR_RATIO: f64 = 2.;

    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self, delta: Duration) {
        // player
        let mut player_move_x = 0.;
        let mut player_move_y = 0.;
        let mut shoot = false;

        for evt in self.events.drain(..) {
            match evt {
                GameEvent::PlayerMove(x, y) => {
                    player_move_x += x;
                    player_move_y += y;
                }
                GameEvent::Shoot => {
                    shoot = true;
                }
            }
        }

        player_move_x *= self.player.move_velocity * delta.as_secs_f64();
        player_move_y *= self.player.move_velocity * delta.as_secs_f64();
        self.player.walk(player_move_x, player_move_y).unwrap();

        if shoot {
            let bullet = self.player.new_bullet();
            self.bullets.push(bullet);
        }

        // bullets
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
