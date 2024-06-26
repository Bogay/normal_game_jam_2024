use ratatui::{
    prelude::*,
    widgets::{canvas::Canvas, Block, BorderType, List},
};

use crate::{app::App, battle::DrawEnemy};

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

    render_game_screen(app, frame, stage_screen);
    render_player_info(app, frame, info_panel);
    render_game_logs(app, frame, logs);
}

fn render_player_info(app: &mut App, frame: &mut Frame, area: Rect) {
    frame.render_widget(
        List::new(
            [
                Line::raw(format!("HP: {}/{}", app.player.hp, app.player.max_hp)),
                Line::raw(format!("MP: {}/{}", app.player.mp, app.player.max_mp)),
                Line::raw(format!(
                    "Pos: ({:.2}, {:.2})",
                    app.player.pos_x, app.player.pos_y
                )),
            ]
            .into_iter()
            .chain(app.player.skills.iter().map(|s| Line::raw(&s.name))),
        )
        .block(
            Block::bordered()
                .title("Player Info")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        ),
        area,
    );
}

fn render_game_screen(app: &mut App, frame: &mut Frame, area: Rect) {
    let x_size = app.world_width;
    let y_size = x_size * (area.height as f64 / area.width as f64) * App::CHAR_RATIO;

    // HACK: modify data in rendering logic
    app.canvas_rect = area;

    frame.render_widget(
        Canvas::default()
            .marker(Marker::Braille)
            .block(
                Block::bordered()
                    .title(format!("  嘴砲遊戲：Stage {}  ", app.stage_index))
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
        List::new(
            app.logs
                .iter()
                .rev()
                .map(|l| Line::raw(&l.0))
                .take(area.height as usize),
        )
        .block(
            Block::bordered()
                .title("Logs")
                .title_alignment(Alignment::Left)
                .border_type(BorderType::Rounded),
        ),
        area,
    );
}
