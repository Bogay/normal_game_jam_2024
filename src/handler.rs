use crate::{
    app::{App, AppResult, GameEvent},
    skill::Skill,
};
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
        KeyCode::Right | KeyCode::Char('d') => {
            app.events.push(GameEvent::PlayerMove(1., 0.));
        }
        KeyCode::Left | KeyCode::Char('a') => {
            app.events.push(GameEvent::PlayerMove(-1., 0.));
        }
        KeyCode::Up | KeyCode::Char('w') => {
            app.events.push(GameEvent::PlayerMove(0., 1.));
        }
        KeyCode::Down | KeyCode::Char('s') => {
            app.events.push(GameEvent::PlayerMove(0., -1.));
        }
        // for testing
        KeyCode::Char('p') => {
            app.player.skills.push(Skill {
                name: "Python".to_string(),
            });
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
