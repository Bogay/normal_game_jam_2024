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

pub(crate) fn norm(x: f64, y: f64) -> (f64, f64) {
    const EPSILON: f64 = 0.0001;
    let m = x * x + y * y;
    if m < EPSILON {
        return (0., 0.);
    }
    let m = m.sqrt();
    (x / m, y / m)
}
