use std::borrow::Borrow;

use ratatui::{
    prelude::*,
    widgets::{canvas::Canvas, Block, BorderType, List},
};

use crate::{
    app::{App, GameLog},
    battle::{DrawEnemy, Enemy},
};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    let [game_screen, info_panel] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Length(32)]).areas(frame.size());
    let [stage_screen, logs] =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(10)]).areas(game_screen);

    app.logs.clear();
    for a in &[game_screen, info_panel, stage_screen, logs] {
        app.logs.push(GameLog(format!("{a:?}")));
    }

    render_game_screen(app, frame, stage_screen);

    // player info
    frame.render_widget(
        List::new([
            Line::raw(format!("HP: {}/{}", app.player.hp, app.player.max_hp)),
            Line::raw(format!("MP: {}/{}", app.player.mp, app.player.max_mp)),
            Line::raw(format!(
                "Pos: ({:.2}, {:.2})",
                app.player.pos_x, app.player.pos_y
            )),
        ])
        .block(
            Block::bordered()
                .title("Player Info")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        ),
        info_panel,
    );

    render_game_logs(app, frame, logs);
}

fn render_game_screen(app: &mut App, frame: &mut Frame, area: Rect) {
    let x_size = 100.;
    let y_size = x_size * (area.height as f64 / area.width as f64) * App::CHAR_RATIO;

    frame.render_widget(
        Canvas::default()
            .marker(Marker::Braille)
            .block(
                Block::bordered()
                    .title("Normal Game Jam 2024")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .paint(|ctx| {
                ctx.draw(&DrawEnemy(&app.enemy));
                ctx.draw(&app.player);
                for b in &app.bullets {
                    ctx.draw(b);
                }
            })
            .x_bounds([-x_size, x_size])
            .y_bounds([-y_size, y_size]),
        area,
    );
}

fn render_game_logs(app: &mut App, frame: &mut Frame, area: Rect) {
    frame.render_widget(
        List::new(app.logs.iter().map(|l| Line::raw(&l.0))).block(
            Block::bordered()
                .title("Logs")
                .title_alignment(Alignment::Left)
                .border_type(BorderType::Rounded),
        ),
        area,
    );
}
