use ratatui::{
    backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, style::{Color, Style}, widgets::{Block, Borders, List, ListItem, Paragraph}, Frame, Terminal
};
use crossterm::{
    event::{self, Event, KeyCode}, execute, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, LeaveAlternateScreen}
};
use std::io::{self, stdout, Stdout};
use std::time::Duration;
use terminput::Event as TermEvent;

use std::sync::Arc;
use russh::{server::{self, Session}, Channel, ChannelId, CryptoVec};
use async_trait::async_trait;
use tokio::net::TcpListener;
use std::collections::HashMap;



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

use tracing::{info};

/// Application state
#[derive(Clone)]
pub struct App {
    selected: usize, // Index of the selected menu item
    state: AppState,

    //for auth
    auth_state: AuthState, // NEW: Manages auth sub-state
    pub email_input: crate::ui_tooling::text_input::TextInputWidgetState,
    pub user_email: String,
    pub verification_code: String,
    pub strikes: i32,
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
    LoggedIn,
}

// MyHandler will contain per-client TUI state
struct MyHandler {
    // Add fields here to store TUI state for each client
    client_id: usize, // A unique identifier for this client's session
}

// MyServer manages the overall SSH server and creates new handlers for clients
#[derive(Clone)]
struct MyServer {
    next_client_id: Arc<tokio::sync::Mutex<usize>>, // Shared counter for client IDs
}

#[async_trait]
impl server::Handler for MyHandler {
    type Error = anyhow::Error; // Or a more specific error type if needed
    // Implement methods required by the `russh::server::Handler` trait here
}

impl server::Server for MyServer {
    type Handler = MyHandler;

    fn new_client(&mut self, _addr: Option<std::net::SocketAddr>) -> Self::Handler {
        // Increment client ID and create a new handler for each connection.
        // A blocking_lock is used here as new_client is not an async method.
        let mut next_id = self.next_client_id.blocking_lock();
        let client_id = *next_id;
        *next_id += 1;
        println!("New client connected. Assigning ID: {}", client_id);
        MyHandler { client_id }
    }

    // This method is invoked when a session encounters an error, allowing for custom error handling.
    fn handle_session_error(&mut self, error: <Self::Handler as server::Handler>::Error) {
        eprintln!("Server session error: {:?}", error);
    }
}

#[quit::main]
#[tokio::main]
async fn main() -> Result<(), io::Error> {
    
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    
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
        user_email: String::new(),
        verification_code: String::new(),
        strikes: 0,
    };

    let (tx, mut rx) = tokio::sync::mpsc::channel(1);

    // Initialize SSH server 
    let config = server::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(3600)), // Set a 1-hour inactivity timeout
        keys: vec![private_key], // Include the server's host key
        auth_rejection_time: std::time::Duration::from_secs(3), // Delay authentication failures
        auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)), // No initial delay
       ..Default::default()
    };
    let config = Arc::new(config);

        // Initialize the MyServer instance, which will manage client handlers.
    let my_server = MyServer {
        next_client_id: Arc::new(tokio::sync::Mutex::new(0)),
    };

    // Start the SSH server, binding to all network interfaces on port 2222.
    println!("SSH server listening on 0.0.0.0:2222");
    russh::server::run_on_address(config, ("0.0.0.0", 2222), my_server).await?;

    // Main event loop
    loop {
        if let Ok(new_app) = rx.try_recv() {
            app = new_app;
        }

        terminal.draw(|f| match app.state {
            AppState::MainMenu => main_menu(f, &app),
            AppState::Settings => settings(f, &app),
            AppState::Auth => auth_menu::menu(f, &app),
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                info!(?key, "Key event received");
                let mut should_quit = false;
                if app.state == AppState::Auth {
                    let menu_size = match app.auth_state {
                        AuthState::InputEmail => 2, // "Send", "Exit"
                        AuthState::VerifyEmail => 4, // "Submit", "Resend", "Change", "Exit"
                        AuthState::LoggedIn => 1, // "Exit"
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
                                let mut app_clone = app.clone();
                                let tx_clone = tx.clone();
                                tokio::spawn(async move {
                                    auth_menu::input(&mut app_clone).await;
                                    let _ = tx_clone.send(app_clone).await;
                                });
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
    }
}


fn handle_input(mut channel_reader: impl std::io::Read) -> std::io::Result<()> {
    let mut buf = [0u8; 64];
    loop {
        let n = channel_reader.read(&mut buf)?;
        if n == 0 { break; }
        let input = &buf[..n];

        // Parse input bytes into events
        match TermEvent::parse_from(input) {
            Ok(Some(event)) => {
                // Handle the event (key press, mouse, etc.)
                println!("Parsed event: {:?}", event);
            }
            Ok(None) => {
                // Need more bytes for a full event
            }
            Err(e) => {
                eprintln!("Error parsing event: {:?}", e);
            }
        }
    }
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

fn pokedex(f: &mut Frame<>, _app: &App){
    let horiz_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
        ).split(f.area());

    let _left_vert_chunks = Layout::default()
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