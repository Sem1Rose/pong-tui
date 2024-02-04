use crate::app::App;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use std::io::Result;

pub fn handle(event: Event, app: &mut App) -> Result<()> {
    if let Event::Key(key) = event {
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }

        match key.code {
            KeyCode::Esc => {
                if app.game_started && app.game.game_paused {
                    app.game.resume_game();
                } else if app.help_popup_shown {
                    app.hide_help_popup();
                } else {
                    app.should_quit = true;
                }
            }
            KeyCode::Char('p') => {
                if app.game_started && !app.game.game_over {
                    if app.game.game_paused {
                        app.game.resume_game();
                    } else {
                        app.game.pause_game();
                    }
                }
            }
            KeyCode::Char('w') => {
                if app.game_started && !app.game.game_paused && !app.game.game_over {
                    if app.game.l_pos <= 0.0 {
                        ()
                    }
                    let platform_height = 3 - app.difficulty as u16;
                    let view_height = (app.frame_size.height - platform_height - 2) as f32;

                    app.game.l_pos =
                        0.0f32.max((f32::round(app.game.l_pos * view_height) - 1.0) / view_height);
                }
            }
            KeyCode::Char('s') => {
                if app.game_started && !app.game.game_paused && !app.game.game_over {
                    if app.game.l_pos >= 1.0 {
                        ()
                    }
                    let platform_height = 3 - app.game.difficulty as u16;
                    let view_height = (app.frame_size.height - platform_height - 2) as f32;

                    app.game.l_pos =
                        1.0f32.min((f32::round(app.game.l_pos * view_height) + 1.0) / view_height);
                }
            }
            KeyCode::Char('h') => {
                if app.game_started && !app.game.game_paused && !app.game.game_over {
                    app.game.show_help_popup();
                }
            }
            KeyCode::Up => {
                if app.main_menu_shown && !app.help_popup_shown {
                    app.dec_main_menu_button();
                } else if app.difficulty_menu_shown {
                    app.dec_difficulty_menu_button();
                } else if app.game_started && !app.game.game_paused && !app.game.game_over {
                    if app.game.r_pos <= 0.0 {
                        ()
                    }
                    let platform_height = 3 - app.game.difficulty as u16;
                    let view_height = (app.frame_size.height - platform_height - 2) as f32;

                    app.game.r_pos =
                        0.0f32.max((f32::round(app.game.r_pos * view_height) - 1.0) / view_height);
                }
            }
            KeyCode::Down => {
                if app.main_menu_shown && !app.help_popup_shown {
                    app.inc_main_menu_button();
                } else if app.difficulty_menu_shown {
                    app.inc_difficulty_menu_button();
                } else if app.game_started && !app.game.game_paused && !app.game.game_over {
                    if app.game.r_pos >= 1.0 {
                        ()
                    }
                    let platform_height = 3 - app.game.difficulty as u16;
                    let view_height = (app.frame_size.height - platform_height - 2) as f32;

                    app.game.r_pos =
                        1.0f32.min((f32::round(app.game.r_pos * view_height) + 1.0) / view_height);
                }
            }
            KeyCode::Right => {
                if app.difficulty_menu_shown {
                    match app.difficulty_menu_selected_button {
                        0 => app.inc_difficulty(),
                        1 => app.inc_win_score(),
                        _ => (),
                    }
                } else if app.game_started && app.game.game_paused {
                    app.game.inc_pause_menu_button();
                } else if app.game_started && app.game.game_over {
                    app.game.inc_game_over_menu_button();
                }
            }
            KeyCode::Left => {
                if app.difficulty_menu_shown {
                    match app.difficulty_menu_selected_button {
                        0 => app.dec_difficulty(),
                        1 => app.dec_win_score(),
                        _ => (),
                    }
                } else if app.game_started && app.game.game_paused {
                    app.game.dec_pause_menu_button();
                } else if app.game_started && app.game.game_over {
                    app.game.dec_game_over_menu_button();
                }
            }
            KeyCode::Enter => {
                if app.main_menu_shown {
                    match app.main_menu_selected_button {
                        0 => app.show_difficulty_select_menu(),
                        1 => app.show_help_popup(),
                        2 => app.should_quit = true,
                        _ => (),
                    }
                } else if app.difficulty_menu_shown {
                    match app.difficulty_menu_selected_button {
                        2 => app.start_game(),
                        3 => app.show_main_menu(),
                        _ => (),
                    }
                } else if app.game_started && app.game.game_paused {
                    match app.game.pause_menu_selected_button {
                        0 => app.game.resume_game(),
                        1 => app.game.show_help_popup(),
                        2 => app.show_difficulty_select_menu(),
                        3 => app.show_main_menu(),
                        _ => (),
                    }
                } else if app.game_started && app.game.game_over {
                    match app.game.game_over_menu_selected_button {
                        0 => app.show_difficulty_select_menu(),
                        1 => app.show_main_menu(),
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
    Ok(())
}
