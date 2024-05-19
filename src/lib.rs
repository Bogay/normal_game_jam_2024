/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Event handler.
pub mod handler;

mod battle;
mod skill;
mod speech_recog;
pub use speech_recog::record_speech;

// generated binding
mod gen {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use gen::create_bullet;
use gen::Bullet as BulletInfo;
use gen::RGBA as Rgba;

/// Some math-related functions

pub(crate) fn norm(x: f64, y: f64) -> (f64, f64) {
    const EPSILON: f64 = 0.0001;
    let m = x * x + y * y;
    if m < EPSILON {
        return (0., 0.);
    }
    let m = m.sqrt();
    (x / m, y / m)
}

pub(crate) fn dis(x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
    let dx = x0 - x1;
    let dy = y0 - y1;

    (dx * dx + dy * dy).sqrt()
}

use std::f64::consts::PI;

pub(crate) fn rotate_vector(x: f64, y: f64, degrees: f64) -> (f64, f64) {
    let radians = degrees * PI / 180.0;
    let cos_theta = radians.cos();
    let sin_theta = radians.sin();

    let x_new = x * cos_theta - y * sin_theta;
    let y_new = x * sin_theta + y * cos_theta;

    (x_new, y_new)
}

pub(crate) fn map_range(min_s: f64, max_s: f64, min_t: f64, max_t: f64, t: f64) -> f64 {
    let diff_s = max_s - min_s;
    let diff_t = max_t - min_t;

    let ratio = (t - min_t) / diff_t;
    min_s + (diff_s * ratio)
}
