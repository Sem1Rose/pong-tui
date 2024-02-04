use rand::Rng;
use std::fmt::Display;

#[derive(Clone, Copy)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    Insane,
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Easy => write!(f, " Easy "),
            Self::Normal => write!(f, "Normal"),
            Self::Hard => write!(f, " Hard "),
            Self::Insane => write!(f, "Insane"),
        }
    }
}

impl TryFrom<i8> for Difficulty {
    type Error = ();
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Easy),
            1 => Ok(Self::Normal),
            2 => Ok(Self::Hard),
            3 => Ok(Self::Insane),
            _ => Err(()),
        }
    }
}

impl Difficulty {
    pub fn len() -> u8 {
        4
    }
}

pub const PAUSE_MENU_BUTTON_COUNT:usize = 4;
pub const GAME_OVER_MENU_BUTTON_COUNT:usize = 2;

pub struct Game {
    pub difficulty: i8,
    pub win_score: u16,
    pub r_pos: f32,
    pub l_pos: f32,
    pub b_pos: [f32; 2],
    pub b_vel: [i16; 2],
    pub r_score: u16,
    pub l_score: u16,
    pub help_popup_shown: bool,
    pub game_paused: bool,
    pub pause_menu_selected_button: i8,
    pub game_over: bool,
    pub game_over_menu_selected_button: i8,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            difficulty: Difficulty::Easy as i8,
            win_score: 0,
            r_pos: 0.0,
            l_pos: 0.0,
            b_pos: [0.0; 2],
            b_vel: [0; 2],
            r_score: 0,
            l_score: 0,
            help_popup_shown: false,
            game_paused: false,
            pause_menu_selected_button: 0,
            game_over: false,
            game_over_menu_selected_button: 0,
        }
    }
}

impl Game {
    pub fn new(_difficulty: i8, _win_score: u16) -> Self {
        Self {
            difficulty: _difficulty,
            win_score: _win_score,
            r_pos: 0.5,
            l_pos: 0.5,
            b_pos: [0.5; 2],
            b_vel: [1; 2],
            r_score: 0,
            l_score: 0,
            help_popup_shown: false,
            game_paused: false,
            pause_menu_selected_button: 0,
            game_over: false,
            game_over_menu_selected_button: 0,
        }
    }

    pub fn reset(&mut self) {
        self.b_pos = [0.5; 2];
        self.b_vel = [1; 2];
        if rand::random() {
            self.b_vel[0] *= -1;
        }
        if rand::random() {
            self.b_vel[1] *= -1;
        }
        self.b_pos[1] = rand::thread_rng().gen::<f32>() / 2.0 + 0.25;
        self.game_paused = false;
    }

    pub fn full_reset(&mut self) {
        self.reset();
        self.l_score = 0;
        self.r_score = 0;
    }

    pub fn pause_game(&mut self) {
        self.game_paused = true;
    }

    pub fn resume_game(&mut self) {
        self.game_paused = false;
        self.help_popup_shown = false;
        self.pause_menu_selected_button = 0;
    }

    pub fn show_help_popup(&mut self) {
        self.help_popup_shown = true;
        self.pause_game();
    }

    pub fn inc_pause_menu_button(&mut self) {
        self.pause_menu_selected_button += 1;
        if self.pause_menu_selected_button >= PAUSE_MENU_BUTTON_COUNT as i8 {
            self.pause_menu_selected_button = 0;
        }
    }

    pub fn dec_pause_menu_button(&mut self) {
        self.pause_menu_selected_button -= 1;
        if self.pause_menu_selected_button < 0 {
            self.pause_menu_selected_button = PAUSE_MENU_BUTTON_COUNT as i8 - 1;
        }
    }

    pub fn check_game_over(&mut self) {
        if self.l_score.max(self.r_score) >= self.win_score {
            self.game_over = true;
            self.game_paused = false;
        } 
    }
    
    pub fn inc_game_over_menu_button(&mut self) {
        self.game_over_menu_selected_button += 1;
        if self.game_over_menu_selected_button >= GAME_OVER_MENU_BUTTON_COUNT as i8 {
            self.game_over_menu_selected_button = 0;
        }
    }

    pub fn dec_game_over_menu_button(&mut self) {
        self.game_over_menu_selected_button -= 1;
        if self.game_over_menu_selected_button < 0 {
            self.game_over_menu_selected_button = GAME_OVER_MENU_BUTTON_COUNT as i8 - 1;
        }
    }
}
