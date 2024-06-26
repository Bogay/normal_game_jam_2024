use crate::app::{AppResult, Bullet, Player};
use crate::battle::{Enemy, EnemyAction};
use ratatui::{
    prelude::*,
    widgets::canvas::{Circle, Shape},
};
use std::{fmt::Debug, time::Duration};

#[derive(Debug)]
pub struct EnemyLevel0 {
    pos_x: f64,
    pos_y: f64,
    hp: isize,
    timer: Duration,
    bullets: Vec<Bullet>,
}

impl EnemyLevel0 {
    pub fn new() -> Self {
        Self {
            pos_x: 30.,
            pos_y: 30.,
            hp: 10,
            timer: Duration::ZERO,
            bullets: vec![],
        }
    }

    fn new_bullet(&self, player: &Player) -> Bullet {
        let delta_x = player.pos_x - self.pos_x;
        let delta_y = player.pos_y - self.pos_y;
        let (delta_x, delta_y) = crate::norm(delta_x, delta_y);

        const VELOCITY: f64 = 6.;
        const OFFSET: f64 = 2.5;
        Bullet {
            pos_x: self.pos_x + OFFSET * delta_x,
            pos_y: self.pos_y + OFFSET * delta_y,
            velocity_x: delta_x * VELOCITY,
            velocity_y: delta_y * VELOCITY,
            is_player: false,
            ..Default::default()
        }
    }
}

impl Shape for EnemyLevel0 {
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

impl Enemy for EnemyLevel0 {
    fn tick(&mut self, delta: Duration, player: &mut Player) -> AppResult<EnemyAction> {
        if self.hp <= 0 {
            return Ok(EnemyAction::Die);
        }

        self.timer += delta;
        if self.timer > Duration::from_secs_f32(2.) {
            self.timer = Duration::ZERO;
            self.bullets.push(self.new_bullet(player));
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
