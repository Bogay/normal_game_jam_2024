use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, List, Paragraph},
};

use crate::app::App;

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

    // stage screen
    frame.render_widget(
        Paragraph::new("")
            .block(
                Block::bordered()
                    .title("Normal Game Jam 2024")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .centered(),
        stage_screen,
    );

    // player info
    frame.render_widget(
        List::new([
            Line::raw(format!("HP: {}/{}", app.player.hp, app.player.max_hp)),
            Line::raw(format!("MP: {}/{}", app.player.mp, app.player.max_mp)),
            Line::raw(format!("Pos: ({}, {})", app.player.pos_x, app.player.pos_y)),
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

fn render_game_logs(_app: &mut App, frame: &mut Frame, area: Rect) {
    frame.render_widget(
        List::new([Line::raw("log0"), Line::raw("log0")]).block(
            Block::bordered()
                .title("Logs")
                .title_alignment(Alignment::Left)
                .border_type(BorderType::Rounded),
        ),
        area,
    );
}
