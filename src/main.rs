use std::path::Path;
use std::fs;

use rand::rngs::OsRng;
use ratatui::{
    backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, style::{Color, Style}, widgets::{Block, Borders, List, ListItem, Paragraph}, Frame, Terminal
};
use crossterm::{
    execute, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, LeaveAlternateScreen, EnterAlternateScreen}
};
use std::io::{self, Stdout};
use std::collections::HashMap;

use std::sync::Arc;
use tokio::sync::Mutex;

use ratatui::{TerminalOptions, Viewport};
use russh::keys::ssh_key::{self};
use russh::server::*;
use russh::{Channel, ChannelId, Pty, CryptoVec};

use tracing_subscriber::{fmt, prelude::*};

pub mod menus {
    pub mod auth_menu;
    pub mod menu_system;
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
pub mod client_handler;

use crate::client_handler::{ClientHandler, TerminalHandle};
use crate::menus::menu_system::MenuSystem;

#[derive(Clone)]
pub struct App {
    selected: usize,
    state: AppState,
    auth_state: AuthState,
    pub email_input: crate::ui_tooling::text_input::TextInputWidgetState,
    pub user_email: String,
    pub verification_code: String,
    pub strikes: i32,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum AppState {
    MainMenu,
    Settings,
    Pokedex,
    Auth,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum AuthState {
    InputEmail,
    VerifyEmail,
    LoggedIn,
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging with console output for debugging
    tracing_subscriber::fmt::init();
    
    println!("Starting SSH server...");
    
    let mut server = AppServer::new();
    
    // Add error handling and status reporting
    match server.run().await {
        Some(()) => {
            println!("Server started successfully on 0.0.0.0:2222");
            // Wait for the Ctrl+C signal
            tokio::signal::ctrl_c().await?;
            println!("Shutting down server...");
            server.shutdown().await;
        }
        None => {
            eprintln!("Failed to start server");
        }
    }
    
    Ok(())
}

fn main_menu(f: &mut Frame<>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(5),
            Constraint::Percentage(75),
            Constraint::Percentage(20),
        ])
        .split(f.area());

    let menu = MenuSystem::get_current_menu(app.state, app.auth_state);
    
    let title = Paragraph::new("\n COMING SOON!!!\n\nPress Esc to quit")
        .block(Block::default().borders(Borders::ALL).title("Main Menu"))
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(title, chunks[1]);

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

    let menu = MenuSystem::get_current_menu(app.state, app.auth_state);
    let items: Vec<ListItem> = menu.items
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

    let menu_widget = List::new(items)
       .block(Block::default().borders(Borders::ALL).title("Options"));
    f.render_widget(menu_widget, bottom_horiz_chunks[0]);
}

fn pokedex(f: &mut Frame<>, app: &App){
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(70),
            Constraint::Percentage(20),
        ])
        .split(f.area());

    let menu = MenuSystem::get_current_menu(app.state, app.auth_state);
    
    let title = Paragraph::new(menu.title)
        .block(Block::default().borders(Borders::ALL).title("Pokedex"))
        .style(Style::default().fg(Color::Green));
    f.render_widget(title, chunks[0]);

    let items: Vec<ListItem> = menu.items
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
    let menu_widget = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Options"));
    f.render_widget(menu_widget, chunks[1]);

    let footer = Paragraph::new("Use the arrow keys to navigate, Enter to select, and Esc to go back")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}



#[derive(Clone)]
struct AppServer {
    clients: Arc<Mutex<HashMap<usize, tokio::sync::mpsc::Sender<Vec<u8>>>>>,
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
        // Generate or load a persistent host key
        let host_key = Self::get_or_create_host_key().await;
        
        let config = Config {
            inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
            auth_rejection_time: std::time::Duration::from_secs(3),
            auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
            keys: vec![host_key],
            nodelay: true,
            ..Default::default()
        };

        println!("Attempting to bind to 0.0.0.0:2222...");
        println!("You can connect with: ssh localhost -p 2222");
        println!("Or: ssh user@localhost -p 2222");
        
        match self.run_on_address(Arc::new(config), ("0.0.0.0", 2222)).await {
            Ok(_) => {
                println!("Server bound successfully and listening on port 2222");
                Some(())
            }
            Err(e) => {
                eprintln!("Failed to bind server: {:?}", e);
                None
            }
        }
    }

    pub async fn shutdown(&mut self) {
        let mut clients = self.clients.lock().await;
        for (_, tx) in clients.iter() {
            let _ = tx.send(vec![3]).await; // Ctrl+C
        }
        clients.clear();
    }

    async fn get_or_create_host_key() -> russh::keys::PrivateKey {
        let key_path = "host_key.pem";
        
        if Path::new(key_path).exists() {
            // Load existing key
            match fs::read(key_path) {
                Ok(key_data) => {
                    match russh::keys::PrivateKey::from_openssh(&key_data) {
                        Ok(key) => {
                            println!("Loaded existing host key from {}", key_path);
                            return key;
                        }
                        Err(e) => {
                            eprintln!("Failed to parse existing host key: {:?}", e);
                            // Remove corrupted key file
                            let _ = fs::remove_file(key_path);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read host key file: {:?}", e);
                }
            }
        }
        
        // Generate new key if loading failed or file doesn't exist
        println!("Generating new host key...");
        let key = russh::keys::PrivateKey::random(&mut OsRng, ssh_key::Algorithm::Ed25519).unwrap();
        
        // Save the key for future use
        match key.to_openssh(ssh_key::LineEnding::LF) {
            Ok(key_data) => {
                if let Err(e) = fs::write(key_path, key_data) {
                    eprintln!("Failed to save host key: {:?}", e);
                } else {
                    println!("Host key saved to {}", key_path);
                }
            }
            Err(e) => {
                eprintln!("Failed to serialize host key: {:?}", e);
            }
        }
        
        key
    }
}

impl Server for AppServer {
    type Handler = Self;
    fn new_client(&mut self, addr: Option<std::net::SocketAddr>) -> Self {
        println!("New client connected: {:?}", addr);
        let s = self.clone();
        self.id += 1;
        s
    }
}

impl Handler for AppServer {
    type Error = anyhow::Error;

    async fn auth_none(&mut self, user: &str) -> Result<Auth, Self::Error> {
        println!("Authentication attempt from user: {}", user);
        Ok(Auth::Accept)
    }

    // Accept password authentication (but don't actually check the password)
    async fn auth_password(&mut self, user: &str, _password: &str) -> Result<Auth, Self::Error> {
        println!("Password authentication attempt from user: {}", user);
        Ok(Auth::Accept)
    }

    // Accept public key authentication (but don't actually verify the key)
    async fn auth_publickey(&mut self, user: &str, _key: &russh::keys::PublicKey) -> Result<Auth, Self::Error> {
        println!("Public key authentication attempt from user: {}", user);
        Ok(Auth::Accept)
    }

    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        session: &mut Session,
    ) -> Result<bool, Self::Error> {
        println!("Channel session opened for client {}", self.id);
        
        // Create a channel for input events with a larger buffer
        let (tx_input, rx_input) = tokio::sync::mpsc::channel::<Vec<u8>>(1000);
        let (tx_output, mut rx_output) = tokio::sync::mpsc::channel::<CryptoVec>(1000);
        
        // Store the input sender
        let mut clients = self.clients.lock().await;
        clients.insert(self.id, tx_input);
        drop(clients); // Release the lock
        
        // Spawn a dedicated task for sending output over SSH
        let russh_handle_clone = session.handle().clone();
        let russh_channel_id_clone = channel.id();
        tokio::spawn(async move {
            while let Some(data) = rx_output.recv().await {
                if let Err(e) = russh_handle_clone.data(russh_channel_id_clone, data).await {
                    eprintln!("Error sending SSH channel data: {:?}", e);
                }
            }
            println!("Output sender task ended for client {}", russh_channel_id_clone);
        });
        
        let terminal_handle = TerminalHandle::start(tx_output.clone()).await;
        let backend = CrosstermBackend::new(terminal_handle);

        // Use a larger default size that's more likely to work with modern terminals
        let options = TerminalOptions {
            viewport: Viewport::Fixed(ratatui::layout::Rect::new(0, 0, 120, 30)),
        };

        let terminal = Terminal::with_options(backend, options)?;
        
        let mut client_handler = ClientHandler::new(
            terminal,
            self.id,
            rx_input,
            tx_output,
            session.handle().clone(),
            channel.id(),
        );

        // Spawn the client handler task
        let client_id = self.id;
        let clients_ref = self.clients.clone();
        tokio::spawn(async move {
            println!("Starting TUI for client {}", client_id);
            
            // Add a small delay to ensure everything is set up
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            
            if let Err(e) = client_handler.run_tui().await {
                eprintln!("Client {} TUI error: {:?}", client_id, e);
            }
            
            // Clean up on exit
            let mut clients = clients_ref.lock().await;
            clients.remove(&client_id);
            
            println!("TUI ended for client {}", client_id);
        });

        Ok(true)
    }

    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let clients = self.clients.lock().await;
        if let Some(tx_input) = clients.get(&self.id) {
            if tx_input.send(data.to_vec()).await.is_err() {
                println!("Client {} handler has closed, removing from clients", self.id);
                drop(clients);
                self.clients.lock().await.remove(&self.id);
                session.close(channel)?;
            }
        } else {
            println!("No client handler found for client {}", self.id);
        }
        Ok(())
    }

    async fn window_change_request(
        &mut self,
        _channel: ChannelId,
        col_width: u32,
        row_height: u32,
        _: u32,
        _: u32,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        println!("Window size change: {}x{}", col_width, row_height);
        
        // Forward the size change to the client handler
        let clients = self.clients.lock().await;
        if let Some(tx_input) = clients.get(&self.id) {
            let resize_event = format!("\x1b[8;{};{}t", row_height, col_width);
            let _ = tx_input.send(resize_event.as_bytes().to_vec()).await;
        }
        
        Ok(())
    }

    async fn pty_request(
        &mut self,
        channel: ChannelId,
        term: &str,
        col_width: u32,
        row_height: u32,
        pix_width: u32,
        pix_height: u32,
        terminal_modes: &[(Pty, u32)],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        println!("PTY request: term={}, size={}x{}, pix={}x{}", term, col_width, row_height, pix_width, pix_height);
        
        // Set up the terminal environment for raw mode
        println!("Terminal modes:");
        for (mode, value) in terminal_modes {
            println!("  {:?}: {}", mode, value);
        }
        
        // Send the resize event to the client handler if it exists
        let clients = self.clients.lock().await;
        if let Some(tx_input) = clients.get(&self.id) {
            let resize_event = format!("\x1b[8;{};{}t", row_height, col_width);
            let _ = tx_input.send(resize_event.as_bytes().to_vec()).await;
        }
        
        session.channel_success(channel)?;
        Ok(())
    }

    async fn channel_close(&mut self, _channel: ChannelId, _session: &mut Session) -> Result<(), Self::Error> {
        println!("Channel {} closed for client {}", _channel, self.id);
        
        // Clean up client
        let mut clients = self.clients.lock().await;
        if let Some(tx) = clients.remove(&self.id) {
            // The client handler will exit automatically when the input channel is closed.
        }
        
        Ok(())
    }
}



pub async fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>){
    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)
        .unwrap();
}

/// Quit terminal function that can be called from any menu state
pub async fn quit_terminal() -> Result<(), Box<dyn std::error::Error>> {
    println!("Quit terminal requested");
    
    // Restore terminal to normal mode
    disable_raw_mode()?;
    
    // Clear screen and show cursor
    execute!(
        std::io::stdout(),
        Clear(ClearType::All),
        crossterm::cursor::Show
    )?;
    
    // ANSI escape sequence to clear the terminal and move cursor to top left
    print!("\x1B[2J\x1B[H");
    // Ensure everything is written out
    std::io::Write::flush(&mut std::io::stdout()).ok();
    // Exit process
    quit::with_code(1);

    println!("Terminal restored to normal mode");
    Ok(())
}