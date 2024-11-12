pub enum CurrentScreen {
    Main,
    Menu,
    Commands,
}

pub enum CurrentlyPlaying {
    Block(Block),
}

pub enum Level {
    Easy,
    Medium,
    Hard,
    Evil,
}

pub struct Block {
    value: Option<u8>,
    row: u8,
    col: u8,
}

pub struct App {
    pub game_board: Vec<Block>, // NOTE: Could make this a fixed sized array
    pub level: Level,
    pub current_screen: CurrentScreen,
    pub currently_playing: Option<CurrentlyPlaying>,
}

impl App {
    pub fn new() -> App {
        App {
            game_board: Vec::<Block>::with_capacity(81),
            level: Level::Easy,
            current_screen: CurrentScreen::Main,
            currently_playing: None,
        }
    }
}
