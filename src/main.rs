use ratatui::{
    backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, style::{Color, Style}, widgets::{Block, Borders, List, ListItem, Paragraph}, Frame, Terminal
};
use crossterm::{
    event::{self, Event, KeyCode}, execute, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, LeaveAlternateScreen}
};
use std::io::{self, stdout, Stdout};


use tracing_subscriber::{fmt, prelude::*};

pub mod menus {
    pub mod auth_menu;
}
pub mod ui_tooling{
    pub mod text_input;
}
pub mod pokemon {
    pub mod pokemon_indexer;
}
pub mod user_management {
    pub mod email_auth;
}
pub mod serde_handler;

use crate::menus::auth_menu;


/// Menu items to display
const MENU_ITEMS: &[&str] = &["Start", "Settings", "Pokedex", "Quit"];

use tracing::{info, error, warn};
use crossterm::tty::IsTty;

/// Application state
pub struct App {
    selected: usize, // Index of the selected menu item
    state: AppState,
    auth_state: AuthState, // NEW: Manages auth sub-state
    pub email_input: crate::ui_tooling::text_input::TextInputWidgetState,
}

#[derive(PartialEq, Clone, Copy)] // NEW: Added derive for state comparison
pub enum AppState {
    MainMenu,
    Settings,
    Auth,
    // Add other states (e.g., Game, Help, etc.)
}

// NEW: Enum for authentication sub-states
#[derive(PartialEq, Clone, Copy)]
pub enum AuthState {
    InputEmail,
    VerifyEmail,
}

#[quit::main]
#[tokio::main]
async fn main() -> Result<(), io::Error> {
    if !stdout().is_tty() {
        eprintln!("This application requires an interactive terminal.");
        std::process::exit(1);
    }
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    //logs
    let file = std::fs::File::create("tracing.log")?;
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(file);
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(non_blocking_writer)) // Use the non-blocking writer
        .init();


    let mut app = App {
        selected: 0,
        state: AppState::Auth,
        auth_state: AuthState::InputEmail, // NEW: Initialize auth state
        email_input: crate::ui_tooling::text_input::TextInputWidgetState::new(),
    };

    // Main event loop
    loop {
        terminal.clear()?;
        terminal.draw(|f| match app.state {
            AppState::MainMenu => main_menu(f, &app),
            AppState::Settings => settings(f, &app),
            AppState::Auth => auth_menu::menu(f, &app),
        })?;

        if let Event::Key(key) = event::read()? {
            info!(?key, "Key event received");
            let mut should_quit = false;
            if app.state == AppState::Auth {
                let menu_size = match app.auth_state {
                    AuthState::InputEmail => 2, // "Send", "Exit"
                    AuthState::VerifyEmail => 4, // "Submit", "Resend", "Change", "Exit"
                };
                clamp_selection(&mut app, menu_size); // Clamp selection before handling input

                info!("Auth state key event");
                // Auth state has special input handling due to the text field
                if app.email_input.mode == crate::ui_tooling::text_input::InputMode::Editing {
                    info!("Editing mode key event");
                    // In editing mode, keys control the text input
                    match key.code {
                        KeyCode::Enter | KeyCode::Tab | KeyCode::Esc => {
                            info!("Exiting editing mode");
                            app.email_input.set_mode(crate::ui_tooling::text_input::InputMode::Normal);
                        }
                        _ => {
                            // Pass other keys to the input handler
                            info!("Passing key to input handler");
                            app.email_input.handle_key(&key);
                        }
                    }
                } else {
                    info!("Normal mode key event");
                    // In normal mode, keys control the menu
                    match key.code {
                        KeyCode::Up => {
                            info!("Moving selection up");
                            app.selected = app.selected.saturating_sub(1);
                        }
                        KeyCode::Down => {
                            info!("Moving selection down");
                            app.selected = app.selected.saturating_add(1);
                        }
                        KeyCode::Enter => {
                            info!("Enter key pressed in normal mode");
                            auth_menu::input(&mut app);
                        }
                        KeyCode::Tab => {
                            info!("Entering editing mode");
                            app.email_input.set_mode(crate::ui_tooling::text_input::InputMode::Editing);
                        }
                        KeyCode::Esc => {
                            info!("Escape key pressed, quitting");
                            should_quit = true;
                        }
                        _ => {}
                    }
                }
            } else {
                info!("Other state key event");
                // Standard menu handling for other states
                match key.code {
                    KeyCode::Up => app.selected = app.selected.saturating_sub(1),
                    KeyCode::Down => app.selected = app.selected.saturating_add(1),
                    KeyCode::Enter => {
                        if let AppState::MainMenu = app.state {
                            match app.selected {
                                1 => app.state = AppState::Settings,
                                3 => should_quit = true,
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Esc => should_quit = true,
                    _ => {}
                }
            }

            if should_quit {
                info!("Quitting application");
                let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout())).unwrap();
                restore_terminal(&mut terminal).unwrap();
                let _ = quit::with_code(0);
            }
        }
    }


    disable_raw_mode()?;
    Ok(())
}

pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    // Reset terminal state
    execute!(stdout(), Clear(ClearType::All))?;
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

/// Draws the main menu
fn main_menu(f: &mut Frame<>, app: &App) {
    // Split the screen vertically: title (20%), menu (60%), footer (20%)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(5),
            Constraint::Percentage(75),
            Constraint::Percentage(20),
        ])
        .split(f.area());

    // Title block
    let title = Paragraph::new("Main Menu")
        .block(Block::default().borders(Borders::ALL).title("Welcome"))
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(title, chunks[0]);

    // Menu list
    let items: Vec<ListItem> = MENU_ITEMS
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
    f.render_widget(menu, chunks[1]);

    // Footer with navigation hint
    let footer = Paragraph::new("↑/↓ to navigate, Enter to select, q to quit")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

fn settings(f: &mut Frame<>, app: &App){
    let vert_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), 
            Constraint::Percentage(50)])
        .split(f.area());

    let top_horiz_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(vert_chunks[0]);
    let bottom_horiz_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(vert_chunks[1]);


    let items: Vec<ListItem> = MENU_ITEMS
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

    f.render_widget(Paragraph::new("Top Left"), top_horiz_chunks[0]);
    f.render_widget(Paragraph::new("Top Right"), top_horiz_chunks[1]);
    f.render_widget(Paragraph::new("Bottom"), bottom_horiz_chunks[1]);

    let menu = List::new(items)
       .block(Block::default().borders(Borders::ALL).title("Options"));
    f.render_widget(menu, bottom_horiz_chunks[0]);

}

fn pokedex(f: &mut Frame<>, app: &App){
    let horiz_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
        ).split(f.area());

    let left_vert_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
        ).split(horiz_chunks[0]);

    
}

// NEW: Function to clamp the selected index
fn clamp_selection(app: &mut App, menu_size: usize) {
    if app.selected >= menu_size {
        app.selected = menu_size - 1;
    }
}
