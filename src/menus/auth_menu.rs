use ratatui::{
    backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, 
    style::{Color, Style}, symbols::line::HORIZONTAL, 
    widgets::{Block, Borders, List, ListItem, Paragraph, StatefulWidget}, 
    Frame, 
    Terminal
};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode},
};
use rat_text::text_input::{TextInput, TextInputState};
use rat_text::{text_input, HasScreenCursor};

use crate::App as MainApp; // Assuming App is defined in your crate root
use crate::AppState; // Assuming AppState is defined in your crate root
use crate::restore_terminal;
use crate::ui_tooling::text_input::{TextInputWidgetState, InputMode, draw_text_input};

/// Draws the authentication menu.
///
/// This function takes a `Frame` and an `App` reference to draw the UI. The `app`
/// parameter is currently unused but kept for future expansion
/// if the auth menu needs to interact with the application state.
pub fn menu(f: &mut Frame<'_>, app: &MainApp) {
    let horiz_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ])
        .split(f.area());
    let vert_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(horiz_chunks[1]);


    let menu_items: &[&str] = &["Send Verification Email", "Exit"];
    let items: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(i, &item)| {
            let style = if i == app.selected {
                Style::default().fg(Color::Black).bg(Color::White)
            } else {
                Style::default()
            };
            ListItem::new(item).style(style)
        })
        .collect();
    let menu = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Options"));
    f.render_widget(menu, vert_chunks[2]);

    // Draw the persistent email input widget
    draw_text_input(f, vert_chunks[0], &app.email_input, "Enter your email:");


    let footer = Paragraph::new(" ↑/↓ to navigate\n Enter to select\n Tab to toggle typing\n Esc to quit")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, vert_chunks[4]);
}

pub fn input(app: &mut MainApp){
    match app.selected {
        0 => { // Send Email
            
        }
        1 => { // Exit
            let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout())).unwrap();
            restore_terminal(&mut terminal).unwrap();
            let _ = quit::with_code(0);
        }
        _ => {}
    }
}