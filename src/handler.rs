use crate::app::{App, AppResult, GameEvent};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Move handlers
        KeyCode::Right => {
            app.events.push(GameEvent::PlayerMove(1., 0.));
        }
        KeyCode::Left => {
            app.events.push(GameEvent::PlayerMove(-1., 0.));
        }
        KeyCode::Up => {
            app.events.push(GameEvent::PlayerMove(0., 1.));
        }
        KeyCode::Down => {
            app.events.push(GameEvent::PlayerMove(0., -1.));
        }
        KeyCode::Char(' ') => {
            app.events.push(GameEvent::Shoot(
                app.player.pos_x + app.player.face_x,
                app.player.pos_y + app.player.face_y,
            ));
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
