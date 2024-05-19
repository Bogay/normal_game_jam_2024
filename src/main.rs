use normal_game_jam_2024::app::{App, AppResult};
use normal_game_jam_2024::event::{Event, EventHandler};
use normal_game_jam_2024::handler::handle_key_events;
use normal_game_jam_2024::tui::Tui;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

use normal_game_jam_2024::record_speech;

fn main() -> AppResult<()> {
    record_speech();

    // // Create an application.
    // let mut app = App::new();

    // // Initialize the terminal user interface.
    // let backend = CrosstermBackend::new(io::stderr());
    // let terminal = Terminal::new(backend)?;
    // let events = EventHandler::new(33);
    // let mut tui = Tui::new(terminal, events);
    // tui.init()?;

    // // Start the main loop.
    // while app.running {
    //     // Render the user interface.
    //     tui.draw(&mut app)?;
    //     // Handle events.
    //     match tui.events.next()? {
    //         Event::Tick(delta) => app.tick(delta),
    //         Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
    //         Event::Mouse(mouse_event) => app.on_mouse_event(mouse_event)?,
    //         Event::Resize(_, _) => {}
    //     }
    // }

    // // Exit the user interface.
    // tui.exit()?;
    Ok(())
}
