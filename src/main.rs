use rand::rngs::OsRng;
use ratatui::{
    backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, style::{Color, Style}, widgets::{Block, Borders, List, ListItem, Paragraph}, Frame, Terminal
};
use crossterm::{
    event::{self, Event, KeyCode}, execute, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, LeaveAlternateScreen}
};
use std::io::{self, stdout, Stdout};
use std::time::Duration;
use std::collections::HashMap;


use std::sync::Arc;

use ratatui::layout::Rect;
use ratatui::{TerminalOptions, Viewport};
use russh::keys::ssh_key::{self, PublicKey};
use russh::server::*;
use russh::{Channel, ChannelId, Pty};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::Mutex;

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

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let mut server = AppServer::new();
    server.run().await.expect("Failed running server");;
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




// All of the server logic is here
struct ServerApp {
    pub counter: usize,
}

impl ServerApp {
    pub fn new() -> ServerApp {
        Self { counter: 0 }
    }
}

struct TerminalHandle {
    sender: UnboundedSender<Vec<u8>>,
    // The sink collects the data which is finally sent to sender.
    sink: Vec<u8>,
}

impl TerminalHandle {
    async fn start(handle: Handle, channel_id: ChannelId) -> Self {
        let (sender, mut receiver) = unbounded_channel::<Vec<u8>>();
        tokio::spawn(async move {
            while let Some(data) = receiver.recv().await {
                let result = handle.data(channel_id, data.into()).await;
                if result.is_err() {
                    eprintln!("Failed to send data: {:?}", result);
                }
            }
        });
        Self {
            sender,
            sink: Vec::new(),
        }
    }
}

// The crossterm backend writes to the terminal handle.
impl std::io::Write for TerminalHandle {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.sink.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let result = self.sender.send(self.sink.clone());
        if result.is_err() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                result.unwrap_err(),
            ));
        }

        self.sink.clear();
        Ok(())
    }
}

#[derive(Clone)]
struct AppServer {
    clients: Arc<Mutex<HashMap<usize, (Terminal<CrosstermBackend<TerminalHandle>>, ServerApp)>>>,
    id: usize,
}

impl AppServer {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            id: 0,
        }
    }

    pub async fn run(&mut self) -> Option<()> {
        let clients = self.clients.clone();
        tokio::spawn(async move {
            // Pre loop setup

            // Terminal setup
            enable_raw_mode().unwrap();
            let mut stdout = io::stdout();
            let backend = CrosstermBackend::new(&mut stdout); // This line might still cause issues if `stdout` is not `Send` or `Sync`
            let mut terminal = Terminal::new(backend);
            
            terminal.as_mut().unwrap().clear();
            
            //logs
            let file = std::fs::File::create("tracing.log").unwrap();
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

    // Main event loop
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                        if let Ok(new_app) = rx.try_recv() {
                            app = new_app;
                        }           

                terminal.as_mut().unwrap().draw(|f| match app.state {
                    AppState::MainMenu => main_menu(f, &app),
                    AppState::Settings => settings(f, &app),
                    AppState::Auth => auth_menu::menu(f, &app), // This is the line that causes the error
                }).expect("Failed to draw to terminal");

                if event::poll(Duration::from_millis(100)).expect("Failed to poll events") { // Removed ?
                    if let Event::Key(key) = event::read().expect("something went wrong reading the key") {
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


                // Does something?
                for (_, (terminal, app)) in clients.lock().await.iter_mut() {
                    app.counter += 1;  
                }
            }
        });

        let config = Config {
            inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
            auth_rejection_time: std::time::Duration::from_secs(3),
            auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
            keys: vec![
                russh::keys::PrivateKey::random(&mut OsRng, ssh_key::Algorithm::Ed25519).unwrap(),
            ],
            nodelay: true,
            ..Default::default()
        };

        let _ = self.run_on_address(Arc::new(config), ("0.0.0.0", 2222))
            .await;

        // The run_on_address method returns `Result<(), russh::Error>`, so we need to return `()`
        // or handle the error. For now, we'll just return `Some(())` to match the `Option<()>` return type.
        Some(())
        
    }
}

impl Server for AppServer {
    type Handler = Self;
    fn new_client(&mut self, _: Option<std::net::SocketAddr>) -> Self {
        let s = self.clone();
        self.id += 1;
        s
    }
}

impl Handler for AppServer {
    type Error = anyhow::Error;

    // Accept any connection without credentials
    async fn auth_none(&mut self, _user: &str) -> Result<Auth, Self::Error> {
        Ok(Auth::Accept)
    }

    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        session: &mut Session,
    ) -> Result<bool, Self::Error> {
        let terminal_handle = TerminalHandle::start(session.handle(), channel.id()).await;

        let backend = CrosstermBackend::new(terminal_handle);

        // the correct viewport area will be set when the client request a pty
        let options = TerminalOptions {
            viewport: Viewport::Fixed(Rect::default()),
        };

        let terminal = Terminal::with_options(backend, options)?;
        let app = ServerApp::new();

        let mut clients = self.clients.lock().await;
        clients.insert(self.id, (terminal, app));

        Ok(true)
    }

    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        match data {
            // Pressing 'q' closes the connection.
            b"q" => {
                self.clients.lock().await.remove(&self.id);
                session.close(channel)?;
            }
            // Pressing 'c' resets the counter for the app.
            // Only the client with the id sees the counter reset.
            b"c" => {
                let mut clients = self.clients.lock().await;
                let (_, app) = clients.get_mut(&self.id).unwrap();
                app.counter = 0;
            }
            _ => {}
        }

        Ok(())
    }

    /// The client's window size has changed.
    async fn window_change_request(
        &mut self,
        _: ChannelId,
        col_width: u32,
        row_height: u32,
        _: u32,
        _: u32,
        _: &mut Session,
    ) -> Result<(), Self::Error> {
        let rect = Rect {
            x: 0,
            y: 0,
            width: col_width as u16,
            height: row_height as u16,
        };

        let mut clients = self.clients.lock().await;
        let (terminal, _) = clients.get_mut(&self.id).unwrap();
        terminal.resize(rect)?;

        Ok(())
    }

    /// The client requests a pseudo-terminal with the given
    /// specifications.
    ///
    /// **Note:** Success or failure should be communicated to the client by calling
    /// `session.channel_success(channel)` or `session.channel_failure(channel)` respectively.
    async fn pty_request(
        &mut self,
        channel: ChannelId,
        _: &str,
        col_width: u32,
        row_height: u32,
        _: u32,
        _: u32,
        _: &[(Pty, u32)],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let rect = Rect {
            x: 0,
            y: 0,
            width: col_width as u16,
            height: row_height as u16,
        };

        let mut clients = self.clients.lock().await;
        let (terminal, _) = clients.get_mut(&self.id).unwrap();
        terminal.resize(rect)?;

        session.channel_success(channel)?;

        Ok(())
    }
}

impl Drop for AppServer {
    fn drop(&mut self) {
        let id = self.id;
        let clients = self.clients.clone();
        tokio::spawn(async move {
            let mut clients = clients.lock().await;
            clients.remove(&id);
        });
    }
}