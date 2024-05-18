use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Default)]
pub struct Player {
    pub pos_x: isize,
    pub pos_y: isize,
    pub hp: isize,
    pub max_hp: isize,
    pub mp: isize,
    pub max_mp: isize,
}

impl Player {
    pub fn walk(&mut self, delta_x: isize, delta_y: isize) -> AppResult<()> {
        self.pos_x += delta_x;
        self.pos_y += delta_y;

        Ok(())
    }
}

#[derive(Debug)]
pub struct GameLog(pub String);

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub player: Player,
    pub stage_index: usize,
    pub logs: Vec<GameLog>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            player: Player::default(),
            stage_index: 0,
            logs: vec![],
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
