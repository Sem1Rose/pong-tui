use crate::app::*;
use crate::font;
use crate::game::*;
use ratatui::{
    layout::*,
    style::*,
    text::Line,
    widgets::{block::*, *},
    Frame,
};

pub fn ui(frame: &mut Frame, app: &mut App, update_ball: bool) {
    app.update_frame_size(frame.size());

    if app.main_menu_shown {
        render_main_menu(frame, app);
    } else if app.difficulty_menu_shown {
        render_difficulty_select_menu(frame, app);
    } else if app.game_started {
        render_game(frame, app, update_ball);
    }
}

fn render_main_menu(frame: &mut Frame, app: &mut App) {
    let frame_size = frame.size();
    const BUTTONS: [&str; MAIN_MENU_BUTTON_COUNT] = ["New Game", "Help", "Quit"];

    let ui_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Ratio(1, 5),
            Constraint::Max(13),
            Constraint::Max(2),
            Constraint::Max(2 * BUTTONS.len() as u16),
            Constraint::Min(1),
        ],
    )
    .split(frame_size);

    frame.render_widget(
        Paragraph::new(font::PONG_TUI)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .padding(Padding::vertical(1))
                    .title("Made with Rust 🦀")
                    .title_style(Style::new().yellow().bold().slow_blink())
                    .title_position(Position::Bottom)
                    .title_alignment(Alignment::Center),
            )
            .white(),
        ui_layout[1],
    );

    let mut text: Vec<Line<'_>> = vec![];
    for i in 0..BUTTONS.len() {
        let mut button = if app.main_menu_selected_button == i as i8 {
            "> ".to_string()
        } else {
            "".to_string()
        };

        button.push_str(BUTTONS[i]);

        if app.main_menu_selected_button == i as i8 {
            button.push_str(" <");
        }

        text.extend([Line::default(), Line::from(button)]);
    }

    frame.render_widget(
        Paragraph::new(text).alignment(Alignment::Center).white(),
        ui_layout[3],
    );

    if app.help_popup_shown {
        show_help_popup(frame);
    }
}

fn render_difficulty_select_menu(frame: &mut Frame, app: &mut App) {
    let frame_size = frame.size();
    const BUTTONS: [&str; DIFFICULTY_MENU_BUTTON_COUNT - DIFFICULTY_MENU_CHOICE_COUNT] =
        ["Start Game", "Back"];
    const CHOICES: [&str; DIFFICULTY_MENU_CHOICE_COUNT] =
        ["Select Difficulty:", "Select Win Score:"];

    let layout_vert = Layout::new(
        Direction::Vertical,
        [
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 2),
            Constraint::Ratio(1, 4),
        ],
    )
    .split(frame_size);

    let layout_horiz = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 2),
            Constraint::Ratio(1, 4),
        ],
    )
    .split(layout_vert[1]);

    let ui_layout = Layout::new(
        Direction::Vertical,
        (0..DIFFICULTY_MENU_CHOICE_COUNT)
            .map(|_| Constraint::Min(2))
            .chain([Constraint::Min(1)]),
    )
    .split(layout_horiz[1]);

    let button_layout = Layout::new(
        Direction::Vertical,
        (0..(DIFFICULTY_MENU_BUTTON_COUNT - DIFFICULTY_MENU_CHOICE_COUNT))
            .map(|_| Constraint::Max(2)),
    )
    .split(ui_layout[DIFFICULTY_MENU_CHOICE_COUNT]);

    let mut choice_widget =
        |layout_index: u8, name: &str, value: &str, i: i8, left_bound: bool, right_bound: bool| {
            frame.render_widget(
                Paragraph::new(name).alignment(Alignment::Left).white(),
                ui_layout[layout_index as usize],
            );

            let mut widget_text = if app.difficulty_menu_selected_button == i && left_bound {
                "< ".to_string()
            } else {
                "  ".to_string()
            };
            widget_text.push_str(value);
            widget_text.push_str(if app.difficulty_menu_selected_button == i && right_bound {
                " >"
            } else {
                "  "
            });

            frame.render_widget(
                Paragraph::new(widget_text)
                    .alignment(Alignment::Right)
                    .white(),
                ui_layout[layout_index as usize],
            );
        };

    choice_widget(
        0,
        CHOICES[0],
        Difficulty::try_from(app.difficulty)
            .unwrap_or(Difficulty::Insane)
            .to_string()
            .as_str(),
        0,
        app.difficulty > 0,
        app.difficulty < Difficulty::len() as i8 - 1,
    );

    choice_widget(
        1,
        CHOICES[1],
        format!("{: >5}  ", app.win_score.to_string()).as_str(),
        1,
        app.win_score > 1,
        app.win_score < 100,
    );

    let mut button_widget =
        |layout_index: usize, name: &str, alignment: Alignment, selected: bool| {
            let mut button = if selected {
                "> ".to_string()
            } else {
                "".to_string()
            };
            button.push_str(name);
            if selected {
                button.push_str(" <");
            }

            frame.render_widget(
                Paragraph::new(vec![Line::default(), Line::from(button)])
                    .alignment(alignment)
                    .white(),
                button_layout[layout_index],
            );
        };

    button_widget(
        0,
        BUTTONS[0],
        Alignment::Center,
        app.difficulty_menu_selected_button == DIFFICULTY_MENU_CHOICE_COUNT as i8,
    );

    button_widget(
        1,
        BUTTONS[1],
        Alignment::Left,
        app.difficulty_menu_selected_button == DIFFICULTY_MENU_CHOICE_COUNT as i8 + 1,
    );
}

fn render_game(frame: &mut Frame, app: &mut App, update_ball: bool) {
    let frame_size = frame.size();
    let split = 15;
    app.frame_size = frame_size;

    //Rendering the UI.
    let ui_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Max((frame_size.width as f32 / 2f32 - (split as f32 / 2.0).floor()) as u16),
            Constraint::Max(split),
            Constraint::Max((frame_size.width as f32 / 2f32 - (split as f32 / 2.0).ceil()) as u16),
        ],
    )
    .split(frame_size);

    let mut l_text: Vec<Paragraph> = vec![];
    let mut l_score = app.game.l_score;
    loop {
        if l_score == 0 {
            if l_text.len() == 0 {
                l_text.push(
                    Paragraph::new(String::from(format!("{}", font::NUMS[0])))
                        .alignment(Alignment::Right),
                );
            }
            break;
        }
        let num = l_score % 10;
        l_text.push(
            Paragraph::new(String::from(format!("{}", font::NUMS[num as usize])))
                .alignment(Alignment::Right),
        );
        l_score = (l_score as f32 / 10.0) as u16;
    }

    l_text.reverse();
    let mut l_area = Rect::new(ui_layout[0].width - 3, 0, 6, 5);
    loop {
        if l_text.len() == 0 {
            break;
        }
        let character = l_text.pop().unwrap();
        frame.render_widget(character.white(), l_area);
        l_area.x -= 6;
    }

    let mut r_text: Vec<Paragraph> = vec![];
    let mut r_score = app.game.r_score;
    loop {
        if r_score == 0 {
            if r_text.len() == 0 {
                r_text.push(
                    Paragraph::new(String::from(format!("{}", font::NUMS[0])))
                        .alignment(Alignment::Right),
                );
            }
            break;
        }
        let num = r_score % 10;
        r_text.push(
            Paragraph::new(String::from(format!("{}", font::NUMS[num as usize])))
                .alignment(Alignment::Right),
        );
        r_score = (r_score as f32 / 10.0) as u16;
    }

    let mut r_area = Rect::new(ui_layout[2].x - 4, 0, 6, 5);
    loop {
        if r_text.len() == 0 {
            break;
        }
        let character = r_text.pop().unwrap();
        frame.render_widget(character.white(), r_area);
        r_area.x += 6;
    }

    let mut mid: Vec<Line<'_>> = vec![];
    for _ in 0..frame_size.height {
        mid.push(Line::from(font::LINE));
    }
    frame.render_widget(
        Paragraph::new(mid).alignment(Alignment::Center).white(),
        ui_layout[1],
    );

    // Rendering the platforms.
    let platform_height = 3 - app.game.difficulty as u16;

    let mut l_platform_widget: Vec<Line> = vec![Line::from("▟")];
    if platform_height > 0 {
        for _ in 0..platform_height {
            l_platform_widget.push(Line::from(font::PLAT));
        }
    }
    l_platform_widget.push(Line::from("▜"));

    let l_pos = (app.game.l_pos * (frame_size.height - platform_height - 2) as f32).round() as u16;
    let l_area = Rect::new(1, l_pos, 1, platform_height + 2);
    frame.render_widget(Paragraph::new(l_platform_widget).white(), l_area);

    let mut r_platform_widget: Vec<Line> = vec![Line::from("▙")];
    if platform_height > 0 {
        for _ in 0..platform_height {
            r_platform_widget.push(Line::from(font::PLAT));
        }
    }
    r_platform_widget.push(Line::from("▛"));

    let r_pos = (app.game.r_pos * (frame_size.height - platform_height - 2) as f32).round() as u16;
    let r_area = Rect::new(frame_size.width - 2, r_pos, 1, platform_height + 2);
    frame.render_widget(Paragraph::new(r_platform_widget).white(), r_area);

    // Rendering the BALL.
    let mut b_pos = [
        (app.game.b_pos[0] * (frame_size.width - 1) as f32).round() as i16,
        (app.game.b_pos[1] * (frame_size.height - 1) as f32).round() as i16,
    ];
    let b_area = Rect::new(b_pos[0] as u16, b_pos[1] as u16, 1, 1);
    frame.render_widget(Paragraph::new(font::BALL).white(), b_area);

    if !app.game.game_paused && !app.game.game_over && update_ball {
        b_pos[0] += app.game.b_vel[0];
        b_pos[1] += app.game.b_vel[1];
        // Collision detection.
        let in_range = |var: i16, min: i16, max: i16| var >= min && var <= max;

        if app.game.b_vel[0] < 0
            && b_pos[0] - 1 == 1
            && in_range(b_pos[1] - l_pos as i16, 0, platform_height as i16)
        {
            app.game.b_vel[0] *= -1;
        } else if app.game.b_vel[0] > 0
            && b_pos[0] + 2 == (frame_size.width - 1) as i16
            && in_range(b_pos[1] - r_pos as i16, 0, platform_height as i16)
        {
            app.game.b_vel[0] *= -1;
        }

        if b_pos[0] == 0 {
            app.game.r_score += 1;
            app.game.reset();
            return;
        } else if b_pos[0] == (frame_size.width - 1) as i16 {
            app.game.l_score += 1;
            app.game.reset();
            return;
        }

        if b_pos[1] == 0 {
            app.game.b_vel[1] *= -1;
        } else if b_pos[1] == (frame_size.height - 1) as i16 {
            app.game.b_vel[1] *= -1;
        }

        app.game.b_pos = [
            b_pos[0] as f32 / (frame_size.width - 1) as f32,
            b_pos[1] as f32 / (frame_size.height - 1) as f32,
        ];
    } else if app.game.game_paused {
        add_pause_menu(frame, app);
    } else if app.game.game_over {
        add_game_over_menu(frame, app);
    }
}

fn add_pause_menu(frame: &mut Frame, app: &mut App) {
    let frame_size = frame.size();
    const BUTTONS: [&str; PAUSE_MENU_BUTTON_COUNT] = ["Continue", "Help", "Restart", "Main Menu"];

    let layout_vert = Layout::new(
        Direction::Vertical,
        [
            Constraint::Ratio(3, 8),
            Constraint::Ratio(2, 8),
            Constraint::Ratio(3, 8),
        ],
    )
    .split(frame_size);

    let min_popup_width = BUTTONS.concat().len() as u16 + BUTTONS.len() as u16 * 8;
    let popup_width = min_popup_width.max((frame_size.width as f32 / 2.0).round() as u16);
    let popup_window = Rect::new(
        (frame_size.width as f32 / 2.0 - popup_width as f32 / 2.0).round() as u16,
        layout_vert[1].y,
        popup_width,
        layout_vert[1].height,
    );

    let ui_block = Block::new()
        .on_dark_gray()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().white())
        .padding(Padding::horizontal(2));

    frame.render_widget(Clear, popup_window);

    let ui_layout = Layout::new(
        Direction::Vertical,
        [Constraint::Min(1), Constraint::Max(1)],
    )
    .split(ui_block.inner(popup_window));

    frame.render_widget(ui_block, popup_window);

    frame.render_widget(
        Paragraph::new("Game Paused!")
            .alignment(Alignment::Center)
            .yellow()
            .bold(),
        ui_layout[0],
    );

    let button_widget = |name: &str, i: i8| {
        let mut button = if app.game.pause_menu_selected_button == i {
            "> ".to_string()
        } else {
            "  ".to_string()
        };
        button.push_str(name);
        button.push_str(if app.game.pause_menu_selected_button == i {
            " <"
        } else {
            "  "
        });

        button
    };

    let button_layout = Layout::new(
        Direction::Horizontal,
        (0..BUTTONS.len()).map(|_| Constraint::Ratio(1, BUTTONS.len() as u32)),
    )
    .split(ui_layout[1]);

    for i in 0..BUTTONS.len() {
        frame.render_widget(
            Paragraph::new(button_widget(BUTTONS[i], i as i8).as_str())
                .alignment(Alignment::Center)
                .white(),
            button_layout[i],
        );
    }

    if app.game.help_popup_shown {
        show_help_popup(frame);
    }
}

fn add_game_over_menu(frame: &mut Frame, app: &mut App) {
    let frame_size = frame.size();
    const BUTTONS: [&str; GAME_OVER_MENU_BUTTON_COUNT] = ["Restart", "Main Menu"];

    let layout_vert = Layout::new(
        Direction::Vertical,
        [
            Constraint::Ratio(3, 8),
            Constraint::Ratio(2, 8),
            Constraint::Ratio(3, 8),
        ],
    )
    .split(frame_size);

    let min_popup_width = BUTTONS.concat().len() as u16 + BUTTONS.len() as u16 * 8;
    let popup_width = min_popup_width.max((frame_size.width as f32 / 3.0).round() as u16);
    let popup_window = Rect::new(
        (frame_size.width as f32 / 2.0 - popup_width as f32 / 2.0).round() as u16,
        layout_vert[1].y,
        popup_width,
        layout_vert[1].height,
    );

    let ui_block = Block::new()
        .on_dark_gray()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().white())
        .padding(Padding::horizontal(2));

    frame.render_widget(Clear, popup_window);

    let ui_layout = Layout::new(
        Direction::Vertical,
        [Constraint::Min(1), Constraint::Max(1)],
    )
    .split(ui_block.inner(popup_window));

    frame.render_widget(ui_block, popup_window);

    let winner: &str = if app.game.l_score > app.game.r_score {
        "Left"
    } else if app.game.r_score > app.game.l_score {
        "Right"
    } else {
        "WTF"
    };
    let mut game_over_text: Vec<Line<'_>> = vec![Line::from("Game Over!"), Line::default()];
    game_over_text.push(Line::styled(
        format!("{winner} Wins!"),
        Style::new().green().bold().slow_blink(),
    ));

    frame.render_widget(
        Paragraph::new(game_over_text).alignment(Alignment::Center),
        ui_layout[0],
    );

    let create_button = |name: &str, i: i8| {
        let mut button = if app.game.game_over_menu_selected_button == i {
            "> ".to_string()
        } else {
            "  ".to_string()
        };
        button.push_str(name);
        button.push_str(if app.game.game_over_menu_selected_button == i {
            " <"
        } else {
            "  "
        });

        button
    };

    let button_layout = Layout::new(
        Direction::Horizontal,
        (0..BUTTONS.len()).map(|_| Constraint::Ratio(1, BUTTONS.len() as u32)),
    )
    .split(ui_layout[1]);

    let mut buttons_text: Vec<String> = vec![];
    for i in 0..BUTTONS.len() {
        buttons_text.push(create_button(BUTTONS[i], i as i8));

        frame.render_widget(
            Paragraph::new(buttons_text[i].as_str())
                .alignment(Alignment::Center)
                .white(),
            button_layout[i],
        );
    }
}

fn show_help_popup(frame: &mut Frame) {
    let frame_size = frame.size();

    let layout_vert = Layout::new(
        Direction::Vertical,
        [
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 2),
            Constraint::Ratio(1, 4),
        ],
    )
    .split(frame_size);

    let popup_width = 50.max((frame_size.width as f32 * 2.0 / 3.0).round() as u16);
    let popup_window = Rect::new(
        (frame_size.width as f32 / 2.0 - popup_width as f32 / 2.0).round() as u16,
        layout_vert[1].y,
        popup_width,
        layout_vert[1].height,
    );

    let text_block = Block::new()
        .on_dark_gray()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().white())
        .padding(Padding::horizontal(1))
        .title("Press Esc to close this window.")
        .title_alignment(Alignment::Center)
        .title_position(Position::Bottom)
        .title_style(Style::new().light_yellow());

    frame.render_widget(Clear, popup_window);

    let mut help_text: Vec<Line<'_>> = vec![
        Line::styled("Help", Style::new().bold().yellow()).alignment(Alignment::Center),
        Line::default(),
    ];

    help_text.extend([
        Line::from("Up/Down, W/S: moves the Left/Right platforms up and down."),
        Line::default(),
    ]);
    help_text.extend([Line::from("P: pause game."), Line::default()]);
    help_text.extend([Line::from("H: show this window while in the game."), Line::default()]);
    help_text.extend([Line::from("Esc: force quit the app."), Line::default()]);

    frame.render_widget(
        Paragraph::new(help_text)
            .wrap(Wrap { trim: true })
            .white()
            .alignment(Alignment::Left)
            .block(text_block.clone()),
        text_block.inner(popup_window),
    );
}
