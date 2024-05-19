use std::{error, fmt::Debug, time::Duration};

use crossterm::event::{MouseEvent, MouseEventKind};
use ratatui::{
    layout::Rect,
    style::Color,
    widgets::canvas::{self, Circle, Points, Shape},
};

use crate::{
    battle::{create_enemy, Enemy, EnemyAction},
    skill::Skill,
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
    pub size: f64,
    pub skills: Vec<Skill>,
}

/// In-game events
#[derive(Debug)]
pub enum GameEvent {
    /// move to direction
    PlayerMove(f64, f64),
    /// shoot aim to direction
    Shoot(f64, f64),
}

impl Player {
    pub fn walk(&mut self, delta_x: f64, delta_y: f64) -> AppResult<()> {
        self.pos_x += delta_x;
        self.pos_y += delta_y;

        let (delta_x, delta_y) = crate::norm(delta_x, delta_y);
        if (delta_x + delta_y).abs() > 0. {
            self.face_x = delta_x;
            self.face_y = delta_y;
        }

        Ok(())
    }

    /// Create a bullet aim to (sx, sy)
    pub fn new_bullet(&self, sx: f64, sy: f64) -> Bullet {
        const BULLET_VELOCITY: f64 = 12.;
        const BULLET_OFFSET: f64 = 1.5;

        let delta_x = sx - self.pos_x;
        let delta_y = sy - self.pos_y;
        let (delta_x, delta_y) = crate::norm(delta_x, delta_y);
        Bullet {
            pos_x: self.pos_x + delta_x * BULLET_OFFSET,
            pos_y: self.pos_y + delta_y * BULLET_OFFSET,
            velocity_x: delta_x * BULLET_VELOCITY,
            velocity_y: delta_y * BULLET_VELOCITY,
            is_player: true,
            ..Default::default()
        }
    }
}

impl Shape for Player {
    fn draw(&self, painter: &mut canvas::Painter) {
        let circle = Circle {
            x: self.pos_x,
            y: self.pos_y,
            radius: self.size,
            color: Color::White,
        };
        circle.draw(painter);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Bullet {
    pub pos_x: f64,
    pub pos_y: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub is_player: bool,
    pub will_remove: bool,
}

impl Bullet {
    pub fn rotated(&self, degrees: f64) -> Self {
        let mut ret = self.clone();
        (ret.velocity_x, ret.velocity_y) =
            crate::rotate_vector(ret.velocity_x, ret.velocity_y, degrees);
        ret
    }
}

impl Shape for Bullet {
    fn draw(&self, painter: &mut canvas::Painter) {
        let points = Points {
            coords: &[(self.pos_x, self.pos_y)],
            color: if self.is_player {
                Color::Yellow
            } else {
                Color::Red
            },
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
    pub world_width: f64,
    pub enemy: Box<dyn Enemy>,
    pub casting: bool,
    // hack for calculate player shoot direction
    pub canvas_rect: Rect,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            player: Player {
                max_hp: 100,
                hp: 100,
                max_mp: 50,
                mp: 50,
                move_velocity: 6.,
                face_x: 1.,
                size: 2.,
                ..Player::default()
            },
            stage_index: 0,
            enemy: create_enemy(0).unwrap(),
            world_width: 100.,
            logs: vec![],
            bullets: vec![],
            events: vec![],
            canvas_rect: Rect::default(),
            casting: false,
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
        let mut shoot = None;

        for evt in self.events.drain(..) {
            match evt {
                GameEvent::PlayerMove(x, y) => {
                    player_move_x += x;
                    player_move_y += y;
                }
                GameEvent::Shoot(x, y) => {
                    shoot = Some((x, y));
                }
            }
        }

        player_move_x *= self.player.move_velocity * delta.as_secs_f64();
        player_move_y *= self.player.move_velocity * delta.as_secs_f64();
        self.player.walk(player_move_x, player_move_y).unwrap();

        // TODO: check skill
        let c_strings = self
            .player
            .skills
            .iter()
            .map(|s| std::ffi::CString::new(s.name.clone()).unwrap())
            .collect::<Vec<_>>();
        let c_char_ptrs = c_strings
            .iter()
            .map(|s| s.as_ptr())
            .collect::<Vec<*const std::os::raw::c_char>>();
        let bullet_info =
            unsafe { crate::create_bullet(c_char_ptrs.as_ptr(), c_char_ptrs.len() as i32) };
        self.logs.push(GameLog(format!("{bullet_info:?}")));

        if let Some((sx, sy)) = shoot {
            // TODO: check skill

            if self.player.mp <= 0 {
                self.logs.push(GameLog("not enough MP".to_string()));
            } else {
                let bullet = self.player.new_bullet(sx, sy);
                self.bullets.push(bullet);
                self.logs
                    .push(GameLog(format!("shoot pos=({:.2}, {:.2})", sx, sy)));
                self.player.mp -= 1;
            }

            self.player.skills.clear();
        }

        // bullets
        for b in &mut self.bullets {
            b.pos_x += b.velocity_x * delta.as_secs_f64();
            b.pos_y += b.velocity_y * delta.as_secs_f64();

            // check collision for player, enemy should do this in their own impl
            if !b.is_player
                && crate::dis(b.pos_x, b.pos_y, self.player.pos_x, self.player.pos_y)
                    <= self.player.size
            {
                b.will_remove = true;
                self.player.hp -= 1;
            }
        }

        // enemy
        if matches!(
            self.enemy.tick(delta, &mut self.player).unwrap(),
            EnemyAction::Die
        ) {
            self.logs
                .push(GameLog(format!("enemy {} died.", self.stage_index)));
            self.stage_index += 1;
            if let Some(e) = create_enemy(self.stage_index) {
                self.enemy = e;
                // TODO: gain new skill
            } else {
                // TODO: game ending
            }
            self.bullets.retain(|b| b.is_player);
        } else {
            self.bullets.extend(self.enemy.bullets());
            self.enemy.hurt(&mut self.bullets);
        }

        self.bullets.retain(|b| !b.will_remove);
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn on_mouse_event(&mut self, evt: MouseEvent) -> AppResult<()> {
        match evt.kind {
            MouseEventKind::Down(_btn) => {
                self.casting = true;
                self.logs.push(GameLog("casting...".to_string()));
            }
            MouseEventKind::Up(_btn) => {
                let x_size = self.world_width;
                let y_size = x_size
                    * (self.canvas_rect.height as f64 / self.canvas_rect.width as f64)
                    * App::CHAR_RATIO;

                let x_bound = [-x_size, x_size];
                let y_bound = [-y_size, y_size];
                let x_grid_bound = [
                    self.canvas_rect.x,
                    self.canvas_rect.x + self.canvas_rect.width,
                ];
                let y_grid_bound = [
                    self.canvas_rect.y,
                    self.canvas_rect.y + self.canvas_rect.height,
                ];

                let grid_x = evt.column - self.canvas_rect.x;
                let grid_y = evt.row - self.canvas_rect.y;

                let click_x = crate::map_range(
                    x_bound[0],
                    x_bound[1],
                    x_grid_bound[0] as f64,
                    x_grid_bound[1] as f64,
                    grid_x as f64,
                );
                let click_y = crate::map_range(
                    y_bound[1],
                    y_bound[0],
                    y_grid_bound[0] as f64,
                    y_grid_bound[1] as f64,
                    grid_y as f64,
                );
                self.events.push(GameEvent::Shoot(click_x, click_y));
            }
            _ => {}
        }

        Ok(())
    }
}
