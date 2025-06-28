//! # Pokemon Arena - Ratatui Example
//! 
//! This is a comprehensive example of using ratatui to build a terminal user interface.
//! It demonstrates key concepts like:
//! - Setting up the terminal backend
//! - Creating a main application loop
//! - Handling user input and events
//! - Drawing widgets and layouts
//! - Managing application state
//! - Error handling

// Import modules
mod pokemon {
    pub mod pokemon_indexer;
}

mod serde_handler; 
mod ui;

use std::{
    io,
    panic,
    time::Duration,
};

// Import the main ratatui types we'll need
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

// Import crossterm for terminal control
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

// Import anyhow for error handling
use anyhow::{Context, Result};

// Import our UI modules
use ui::{draw_ui, handle_input};
use ui::states::App;

// ============================================================================
// MAIN APPLICATION LOOP
// ============================================================================

/// The main function - entry point of our application
#[tokio::main]
async fn main() -> Result<()> {
    // Set up panic handling to restore terminal state
    panic::set_hook(Box::new(|panic_info| {
        // Restore terminal state on panic
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        eprintln!("\nApplication panicked: {:?}", panic_info);
        eprintln!("Terminal state has been restored.");
    }));

    // Run the main application
    run_app().await
}

/// Main application loop
async fn run_app() -> Result<()> {
    // ========================================================================
    // TERMINAL SETUP
    // ========================================================================
    
    // Enable raw mode - this gives us direct control over the terminal
    // Without this, the terminal would handle input buffering and echo
    enable_raw_mode().context("Failed to enable raw mode")?;

    // Enter alternate screen - this creates a new screen buffer
    // When we exit, the original screen content will be restored
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)
        .context("Failed to enter alternate screen")?;

    // Create the terminal backend
    // CrosstermBackend handles the low-level terminal operations
    let backend = CrosstermBackend::new(io::stdout());
    
    // Create the terminal instance
    // This is our interface for drawing to the screen
    let mut terminal = Terminal::new(backend).context("Failed to create terminal")?;

    // ========================================================================
    // APPLICATION STATE INITIALIZATION
    // ========================================================================
    
    // Create our application state with main menu as default
    let mut app = App::new();
    // The main menu is already the default state when App::new() is called

    // ========================================================================
    // MAIN EVENT LOOP
    // ========================================================================
    
    // This is the heart of our application - it runs until we want to quit
    loop {
        // Clear the screen and draw the current UI state
        if let Err(e) = terminal.draw(|f| draw_ui(f, &app)) {
            eprintln!("Failed to draw UI: {}", e);
            break;
        }

        // Check if we should quit
        if app.should_quit() {
            break;
        }

        // Handle input events with a timeout
        // This prevents the app from blocking indefinitely
        match crossterm::event::poll(Duration::from_millis(250)) {
            Ok(true) => {
                // Get the next event from the terminal
                match event::read() {
                    Ok(event) => {
                        // Process the event and update application state
                        handle_input(&mut app, event);
                    }
                    Err(e) => {
                        eprintln!("Failed to read event: {}", e);
                        break;
                    }
                }
            }
            Ok(false) => {
                // No events available, continue
            }
            Err(e) => {
                eprintln!("Failed to poll for events: {}", e);
                break;
            }
        }

        // Update application state (called on each iteration)
        app.tick();
    }

    // ========================================================================
    // CLEANUP
    // ========================================================================
    
    // Restore terminal state
    disable_raw_mode().context("Failed to disable raw mode")?;
    execute!(
        io::stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .context("Failed to restore terminal state")?;

    Ok(())
}