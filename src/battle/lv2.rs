use crate::app::{AppResult, Bullet, Player};
use crate::battle::{Enemy, EnemyAction};
use ratatui::{
    prelude::*,
    widgets::canvas::{Circle, Shape},
};
use std::{fmt::Debug, time::Duration};

#[derive(Debug)]
enum State {
    Idle,
    Shooting(usize),
}

#[derive(Debug)]
pub struct EnemyLevel2 {
    pos_x: f64,
    pos_y: f64,
    hp: isize,
    timer: Duration,
    bullets: Vec<Bullet>,
    state: State,
}

impl EnemyLevel2 {
    pub fn new() -> Self {
        Self {
            pos_x: 30.,
            pos_y: 30.,
            hp: 10,
            timer: Duration::ZERO,
            bullets: vec![],
            state: State::Idle,
        }
    }

    fn new_bullets(&self, player: &Player, extra_degree: f64) -> Vec<Bullet> {
        let delta_x = player.pos_x - self.pos_x;
        let delta_y = player.pos_y - self.pos_y;
        let (delta_x, delta_y) = crate::norm(delta_x, delta_y);

        const VELOCITY: f64 = 6.;

        let b0 = Bullet {
            pos_x: self.pos_x,
            pos_y: self.pos_y,
            velocity_x: delta_x * VELOCITY,
            velocity_y: delta_y * VELOCITY,
            is_player: false,
            ..Default::default()
        };

        let count = 12;
        let delta = 360. / 12.;

        (0..count)
            .map(|i| {
                let b = b0.rotated(extra_degree + delta * i as f64);
                b
            })
            .collect()
    }
}

impl Shape for EnemyLevel2 {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        let circle = Circle {
            x: self.pos_x,
            y: self.pos_y,
            radius: 3.,
            color: Color::Red,
        };
        circle.draw(painter);
    }
}

impl Enemy for EnemyLevel2 {
    fn tick(&mut self, delta: Duration, player: &mut Player) -> AppResult<EnemyAction> {
        if self.hp <= 0 {
            return Ok(EnemyAction::Die);
        }

        self.timer += delta;
        match self.state {
            State::Idle => {
                if self.timer > Duration::from_secs_f32(2.) {
                    self.timer = Duration::ZERO;
                    self.state = State::Shooting(0);
                }
            }
            State::Shooting(i) if i < 3 => {
                if self.timer > Duration::from_secs_f32(0.8) {
                    self.timer = Duration::ZERO;
                    self.state = State::Shooting(i + 1);
                    self.bullets.extend(self.new_bullets(player, 5. * i as f64));
                }
            }
            State::Shooting(_) => {
                self.state = State::Idle;
            }
        }

        Ok(EnemyAction::Idle)
    }

    fn bullets(&mut self) -> Vec<Bullet> {
        self.bullets.drain(..).collect()
    }

    fn hurt(&mut self, bullets: &mut Vec<Bullet>) {
        for b in bullets {
            if b.is_player && crate::dis(b.pos_x, b.pos_y, self.pos_x, self.pos_y) <= 3. {
                b.will_remove = true;
                self.hp -= 1;
            }
        }
    }
}
