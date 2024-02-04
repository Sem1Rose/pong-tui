use crate::game::{Game, Difficulty};
use ratatui::layout::Rect;

pub const MAIN_MENU_BUTTON_COUNT:usize = 3;
pub const DIFFICULTY_MENU_BUTTON_COUNT:usize = 4;
pub const DIFFICULTY_MENU_CHOICE_COUNT:usize = 2;

pub struct App {
    pub should_quit: bool,
    pub main_menu_shown: bool,
    pub main_menu_selected_button: i8,
    pub help_popup_shown: bool,
    pub difficulty_menu_shown: bool,
    pub difficulty_menu_selected_button: i8,
    pub difficulty: i8,
    pub win_score: u16,
    pub game_started: bool,
    pub game: Game,
    pub frame_size: Rect,
}

impl App {
    pub fn new() -> Self {
        Self {
            game: Game::default(),
            should_quit: false,
            main_menu_shown: true,
            main_menu_selected_button: 0,
            help_popup_shown: false,
            difficulty_menu_shown: false,
            difficulty_menu_selected_button: 2,
            difficulty: 0,
            win_score: 10,
            game_started: false,
            frame_size: Rect::default(),
        }
    }

    pub fn start_game(&mut self) {
        self.game_started = true;
        self.main_menu_shown = false;
        self.difficulty_menu_shown = false;
        self.game = Game::new(self.difficulty, self.win_score);
        self.game.full_reset();
    }

    pub fn update_frame_size(&mut self, _frame_size: Rect) {
        self.frame_size = _frame_size;
    }

    pub fn show_main_menu(&mut self) {
        self.main_menu_shown = true;
        self.help_popup_shown = false;
        self.difficulty_menu_shown = false;
        self.game_started = false;
        self.main_menu_selected_button = 0;
    }

    pub fn inc_main_menu_button(&mut self) {
        self.main_menu_selected_button += 1;
        if self.main_menu_selected_button >= MAIN_MENU_BUTTON_COUNT as i8 {
            self.main_menu_selected_button = 0;
        }
    }

    pub fn dec_main_menu_button(&mut self) {
        self.main_menu_selected_button -= 1;
        if self.main_menu_selected_button < 0 {
            self.main_menu_selected_button = MAIN_MENU_BUTTON_COUNT as i8 - 1;
        }
    }

    pub fn show_help_popup(&mut self) {
        self.main_menu_shown = true;
        self.help_popup_shown = true;
        self.difficulty_menu_shown = false;
        self.game_started = false;
    }

    pub fn hide_help_popup(&mut self) {
        self.main_menu_shown = true;
        self.help_popup_shown = false;
        self.difficulty_menu_shown = false;
        self.game_started = false;
    }

    pub fn show_difficulty_select_menu(&mut self) {
        self.difficulty_menu_shown = true;
        self.main_menu_shown = false;
        self.help_popup_shown = false;
        self.game_started = false;
        self.difficulty_menu_selected_button = 2;
        self.difficulty = 0;
        self.win_score = 10;
    }

    pub fn inc_difficulty_menu_button(&mut self) {
        self.difficulty_menu_selected_button += 1;
        if self.difficulty_menu_selected_button >= DIFFICULTY_MENU_BUTTON_COUNT as i8 {
            self.difficulty_menu_selected_button = 0;
        }
    }

    pub fn dec_difficulty_menu_button(&mut self) {
        self.difficulty_menu_selected_button -= 1;
        if self.difficulty_menu_selected_button < 0 {
            self.difficulty_menu_selected_button = DIFFICULTY_MENU_BUTTON_COUNT as i8 - 1;
        }
    }

    pub fn inc_difficulty(&mut self) {
        self.difficulty += 1;
        if self.difficulty >= Difficulty::len() as i8 {
            self.difficulty = Difficulty::len() as i8 - 1;
        }
    }

    pub fn dec_difficulty(&mut self) {
        self.difficulty -= 1;
        if self.difficulty < 0 {
            self.difficulty = 0;
        }
    }

    pub fn inc_win_score(&mut self) {
        self.win_score += 1;
        if self.win_score > 100 {
            self.win_score = 100;
        }
    }

    pub fn dec_win_score(&mut self) {
        self.win_score -= 1;
        if self.win_score < 1 {
            self.win_score = 1;
        }
    }
}
