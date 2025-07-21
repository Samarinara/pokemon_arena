use std::io::{self, Write};
use std::time::Duration;
use tokio::sync::mpsc;
use crate::ui_tooling::text_input::TextInputWidgetState;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    Frame,
};
use russh::{ChannelId, CryptoVec, server::Handle};

use crate::{App, AppState, AuthState, main_menu, menus::auth_menu, menus::menu_system::MenuSystem, settings, pokedex, quit_terminal};

use termwiz::input::{InputEvent, KeyCode, KeyEvent, Modifiers};

pub struct ClientHandler {
    terminal: Terminal<CrosstermBackend<TerminalHandle>>,
    client_id: usize,
    input_receiver: mpsc::Receiver<Vec<u8>>,
    app: App,
    parser: termwiz::input::InputParser,
    output_tx: mpsc::Sender<CryptoVec>,
    russh_handle: Handle,
    russh_channel_id: ChannelId,
}

impl ClientHandler {
    pub fn new(
        terminal: Terminal<CrosstermBackend<TerminalHandle>>,
        client_id: usize,
        input_receiver: mpsc::Receiver<Vec<u8>>,
        output_tx: mpsc::Sender<CryptoVec>,
        russh_handle: Handle,
        russh_channel_id: ChannelId,
    ) -> Self {
        let app = App {
            selected: 0,
            state: AppState::Auth,
            auth_state: AuthState::InputEmail,
            email_input: TextInputWidgetState::new(),
            user_email: String::new(),
            verification_code: String::new(),
            strikes: 0,
        };

        Self {
            terminal,
            client_id,
            input_receiver,
            app,
            parser: termwiz::input::InputParser::new(),
            output_tx,
            russh_handle,
            russh_channel_id,
        }
    }

    pub async fn run_tui(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("TUI started for client {}", self.client_id);
        
        // Send initial terminal setup sequences with proper flushing
        println!("Client {} sending initial terminal setup", self.client_id);
        
        // Clear screen and move cursor to home position
        self.terminal.backend_mut().write_all(b"\x1b[2J")?;
        self.terminal.backend_mut().flush()?;
        self.terminal.backend_mut().write_all(b"\x1b[H")?;
        self.terminal.backend_mut().flush()?;
        
        // Send a simple test message to verify terminal output is working
        self.terminal.backend_mut().write_all(b"Terminal test - if you see this, terminal output is working!\r\n")?;
        self.terminal.backend_mut().flush()?;
        
        println!("Client {} initial terminal setup complete", self.client_id);

        // Small delay to ensure terminal setup is processed
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Initial draw
        self.draw().await?;
        
        // Main event loop - this is the key change: it's fully async
        loop {
            // Use tokio::select! to handle multiple async operations
            tokio::select! {
                // Handle input events
                Some(data) = self.input_receiver.recv() => {
                    let mut events = Vec::new();
                    self.parser.parse(&data, |event| {
                        events.push(event);
                    }, false);
                    for event in events.drain(..) {
                        if let InputEvent::Key(key) = event {
                            println!("Client {} processing event: {:?}", self.client_id, key);
                            if self.handle_input(key).await? {
                                return Ok(()); // Exit requested
                            }
                        } else if let InputEvent::Resized { cols, rows } = event {
                            println!("Client {} processing resize event: {}x{}", self.client_id, cols, rows);
                            self.terminal.backend_mut().write_all(b"\x1b[2J\x1b[H")?;
                            self.terminal.resize(ratatui::layout::Rect::new(0, 0, cols as u16, rows as u16))?;
                        }
                    }
                    // Redraw after handling input
                    self.draw().await?;
                }
                
                // Add a timeout to prevent hanging
                _ = tokio::time::sleep(Duration::from_secs(1)) => {
                    // Periodic refresh or keep-alive
                    // You can add periodic tasks here if needed
                }
            }
        }
        
        println!("TUI loop ended for client {}", self.client_id);
        Ok(())
    }

    async fn handle_input(&mut self, key: KeyEvent) -> Result<bool, Box<dyn std::error::Error>> {
        let crossterm_key_event = crossterm::event::KeyEvent::new(
            match key.key {
                KeyCode::Char(c) => crossterm::event::KeyCode::Char(c),
                KeyCode::Backspace => crossterm::event::KeyCode::Backspace,
                KeyCode::Enter => crossterm::event::KeyCode::Enter,
                KeyCode::LeftArrow => crossterm::event::KeyCode::Left,
                KeyCode::RightArrow => crossterm::event::KeyCode::Right,
                KeyCode::UpArrow => crossterm::event::KeyCode::Up,
                KeyCode::DownArrow => crossterm::event::KeyCode::Down,
                KeyCode::Escape => crossterm::event::KeyCode::Esc,
                KeyCode::Tab => crossterm::event::KeyCode::Tab,
                _ => crossterm::event::KeyCode::Null, // Fallback for unhandled keys
            },
            match key.modifiers {
                Modifiers::SHIFT => crossterm::event::KeyModifiers::SHIFT,
                Modifiers::ALT => crossterm::event::KeyModifiers::ALT,
                Modifiers::CTRL => crossterm::event::KeyModifiers::CONTROL,
                _ => crossterm::event::KeyModifiers::empty(),
            },
        );

        match crossterm_key_event.code {
            crossterm::event::KeyCode::Char('q') => {
                // Check if Ctrl+Q is pressed for universal quit
                if crossterm_key_event.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                    println!("Client {} Ctrl+Q pressed - universal quit", self.client_id);
                    if let Err(e) = quit_terminal().await {
                        eprintln!("Error quitting terminal: {:?}", e);
                    }
                    return Ok(true);
                } else {
                    // Regular 'q' key - only quit from main menu
                    if self.app.state == AppState::MainMenu {
                        println!("Client {} 'q' pressed in main menu - quitting", self.client_id);
                        if let Err(e) = quit_terminal().await {
                            eprintln!("Error quitting terminal: {:?}", e);
                        }
                        return Ok(true);
                    }
                }
            }
            crossterm::event::KeyCode::Up => {
                if self.app.email_input.mode == crate::ui_tooling::text_input::InputMode::Editing {
                    // Do nothing, let text input handle it if needed
                } else {
                    let old_selection = self.app.selected;
                    MenuSystem::handle_up_arrow(&mut self.app.selected, self.app.state, self.app.auth_state);
                    if old_selection != self.app.selected {
                        println!("Client {} Up arrow: {} -> {}", self.client_id, old_selection, self.app.selected);
                    }
                }
            }
            crossterm::event::KeyCode::Down => {
                if self.app.email_input.mode == crate::ui_tooling::text_input::InputMode::Editing {
                    // Do nothing
                } else {
                    let old_selection = self.app.selected;
                    MenuSystem::handle_down_arrow(&mut self.app.selected, self.app.state, self.app.auth_state);
                    if old_selection != self.app.selected {
                        println!("Client {} Down arrow: {} -> {}", self.client_id, old_selection, self.app.selected);
                    }
                }
            }
            crossterm::event::KeyCode::Left => {
                if self.app.state == AppState::Auth && self.app.email_input.mode == crate::ui_tooling::text_input::InputMode::Editing {
                    self.app.email_input.move_cursor_left();
                }
            }
            crossterm::event::KeyCode::Right => {
                if self.app.state == AppState::Auth && self.app.email_input.mode == crate::ui_tooling::text_input::InputMode::Editing {
                    self.app.email_input.move_cursor_right();
                }
            }
            crossterm::event::KeyCode::Enter => {
                println!("Client {} Enter key pressed, state: {:?}, auth_state: {:?}, selected: {}", 
                    self.client_id, self.app.state, self.app.auth_state, self.app.selected);
                match self.app.state {
                    AppState::Auth => {
                        // Check if we're in text input editing mode
                        if self.app.email_input.mode == crate::ui_tooling::text_input::InputMode::Editing {
                            // Handle text input Enter key
                            if self.app.email_input.handle_key(&crossterm_key_event) {
                                auth_menu::input(&mut self.app).await;
                            }
                        } else {
                            // Handle menu selection in auth state
                            match self.app.auth_state {
                                AuthState::InputEmail => {
                                    match self.app.selected {
                                        0 => {
                                            // Send Verification Email
                                            auth_menu::input(&mut self.app).await;
                                        }
                                        1 => {
                                            // Exit Program
                                            crate::quit_terminal().await?;
                                        }
                                        _ => {}
                                    }
                                }
                                AuthState::VerifyEmail => {
                                    match self.app.selected {
                                        0 => {
                                            // Submit
                                            auth_menu::input(&mut self.app).await;
                                        }
                                        1 => {
                                            // Resend Email
                                            auth_menu::resend_email(&mut self.app).await;
                                        }
                                        2 => {
                                            // Change Email
                                            println!("Client {} changing email, going back to InputEmail", self.client_id);
                                            self.app.auth_state = AuthState::InputEmail;
                                            self.app.selected = 0;
                                        }
                                        3 => {
                                            // Exit - go back to main menu
                                            println!("Client {} exiting verify email back to MainMenu", self.client_id);
                                            self.app.state = AppState::MainMenu;
                                            self.app.selected = 0;
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    AppState::MainMenu => {
                        match self.app.selected {
                            0 => {
                                // Start - could transition to game or auth
                                println!("Client {} transitioning from MainMenu to Auth", self.client_id);
                                self.app.state = AppState::Auth;
                                self.app.email_input.set_mode(crate::ui_tooling::text_input::InputMode::Editing);
                            }
                            1 => {
                                // Settings
                                println!("Client {} transitioning from MainMenu to Settings", self.client_id);
                                self.app.state = AppState::Settings;
                                self.app.selected = 0; // Reset selection
                            }
                            2 => {
                                // Pokedex
                                println!("Client {} transitioning from MainMenu to Pokedex", self.client_id);
                                self.app.state = AppState::Pokedex;
                                self.app.selected = 0; // Reset selection
                            }
                            3 => {
                                // Quit
                                return Ok(true);
                            }
                            _ => {}
                        }
                    }
                    AppState::Settings => {
                        // Handle settings menu selections
                        match self.app.selected {
                            3 => {
                                // Back to main menu
                                println!("Client {} transitioning from Settings back to MainMenu", self.client_id);
                                self.app.state = AppState::MainMenu;
                                self.app.selected = 0;
                            }
                            _ => {}
                        }
                    }
                    AppState::Pokedex => {
                        // Handle pokedex menu selections
                        match self.app.selected {
                            3 => {
                                // Back to main menu
                                println!("Client {} transitioning from Pokedex back to MainMenu", self.client_id);
                                self.app.state = AppState::MainMenu;
                                self.app.selected = 0;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            crossterm::event::KeyCode::Tab => {
                if self.app.state == AppState::Auth {
                    // Toggle between text input and menu selection
                    match self.app.email_input.mode {
                        crate::ui_tooling::text_input::InputMode::Editing => {
                            println!("Client {} Tab: switching from text input to menu selection", self.client_id);
                            self.app.email_input.set_mode(crate::ui_tooling::text_input::InputMode::Normal);
                        }
                        crate::ui_tooling::text_input::InputMode::Normal => {
                            println!("Client {} Tab: switching from menu selection to text input", self.client_id);
                            self.app.email_input.set_mode(crate::ui_tooling::text_input::InputMode::Editing);
                        }
                    }
                }
            }
            crossterm::event::KeyCode::Esc => {
                println!("Client {} Escape key pressed", self.client_id);
                
                // Handle Escape based on current state
                match self.app.state {
                    AppState::Auth => {
                        if self.app.email_input.mode == crate::ui_tooling::text_input::InputMode::Editing {
                            // Exit text input mode
                            self.app.email_input.set_mode(crate::ui_tooling::text_input::InputMode::Normal);
                        } else {
                            // Exit auth menu and go to main menu
                            self.app.state = AppState::MainMenu;
                            self.app.selected = 0;
                        }
                    }
                    AppState::MainMenu => {
                        // In main menu, Escape quits the application
                        println!("Client {} quitting from main menu", self.client_id);
                        if let Err(e) = quit_terminal().await {
                            eprintln!("Error quitting terminal: {:?}", e);
                        }
                        return Ok(true);
                    }
                    AppState::Settings | AppState::Pokedex => {
                        // Go back to main menu from sub-menus
                        self.app.state = AppState::MainMenu;
                        self.app.selected = 0;
                    }
                }
            }
            _ => {
                if self.app.state == AppState::Auth && self.app.email_input.mode == crate::ui_tooling::text_input::InputMode::Editing {
                    self.app.email_input.handle_key(&crossterm_key_event);
                }
            }
        }
        
        // Clamp selection to valid range using the centralized menu system
        let old_selection = self.app.selected;
        MenuSystem::clamp_selection(&mut self.app.selected, self.app.state, self.app.auth_state);
        if old_selection != self.app.selected {
            println!("Client {} Selection clamped: {} -> {} (state: {:?}, auth_state: {:?})", 
                self.client_id, old_selection, self.app.selected, self.app.state, self.app.auth_state);
        }
        
        Ok(false) // Continue running
    }

    async fn draw(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Use temporary variables to avoid borrowing conflicts
        let terminal = &mut self.terminal;
        let app = &self.app;

        // Debug: Print current state and selection
        println!("Client {} Drawing - State: {:?}, AuthState: {:?}, Selected: {}", 
            self.client_id, app.state, app.auth_state, app.selected);

        terminal.draw(|f: &mut Frame<'_>| {
            match app.state {
                AppState::MainMenu => main_menu(f, app),
                AppState::Settings => settings(f, app),
                AppState::Pokedex => pokedex(f, app),
                AppState::Auth => {
                    auth_menu::menu(f, app);
                }
            }
        })?;

        // Force flush to ensure updates are sent to the SSH client immediately
        terminal.backend_mut().flush()?;

        Ok(())
    }
}

pub struct TerminalHandle {
    output_sender: mpsc::Sender<CryptoVec>,
    buffer: Vec<u8>,
}

impl TerminalHandle {
    pub async fn start(output_sender: mpsc::Sender<CryptoVec>) -> Self {
        Self { output_sender, buffer: Vec::new() }
    }
}

impl io::Write for TerminalHandle {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Debug: Log when data is being written
        if buf.len() > 0 {
            // Convert bytes to string for debugging (only for printable ASCII)
            let debug_str: String = buf.iter()
                .map(|&b| if b >= 32 && b <= 126 { b as char } else { '.' })
                .collect();
            println!("TerminalHandle::write called with {} bytes: {:?}", buf.len(), debug_str);
        }
        self.buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.buffer.is_empty() {
            return Ok(());
        }
        
        println!("TerminalHandle::flush called with {} bytes in buffer", self.buffer.len());
        
        let data = CryptoVec::from_slice(&self.buffer);
        self.buffer.clear();
        
        // Try to send the data, but handle errors more gracefully
        match self.output_sender.try_send(data) {
            Ok(_) => {
                println!("TerminalHandle::flush successfully sent data");
            }
            Err(tokio::sync::mpsc::error::TrySendError::Full(data)) => {
                // Channel is full, try to send in smaller chunks or drop
                eprintln!("Output channel full, dropping data");
                // For now, we'll drop the data, but in a real implementation
                // you might want to implement a more sophisticated buffering strategy
            }
            Err(tokio::sync::mpsc::error::TrySendError::Closed(data)) => {
                // Channel is closed, the client has disconnected
                eprintln!("Output channel closed, client disconnected");
                return Err(io::Error::new(io::ErrorKind::BrokenPipe, "Client disconnected"));
            }
        }
        
        Ok(())
    }
}

impl Clone for TerminalHandle {
    fn clone(&self) -> Self {
        Self {
            output_sender: self.output_sender.clone(),
            buffer: Vec::new(), // Clone with an empty buffer
        }
    }
}