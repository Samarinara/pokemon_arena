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
mod pokemon {
    pub mod pokemon_indexer;
}

mod serde_handler; 

use std::{
    io,
    panic,
    time::{Duration, Instant},
};

// Import the main ratatui types we'll need
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

// Import crossterm for terminal control
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

// Import anyhow for error handling
use anyhow::{Context, Result};

// ============================================================================
// APPLICATION STATE
// ============================================================================

/// Represents the current state of our application
/// This is where you'd store all your app's data
#[derive(Debug)]
struct App {
    /// Current counter value - demonstrates state management
    counter: u32,

    pokemon: String,    
    /// Whether the app should exit
    should_quit: bool,
    
    /// Current selected option in the menu
    selected_option: usize,
    
    /// Available menu options
    options: Vec<String>,
    
    /// Timer for animations or periodic updates
    last_tick: Instant,
}

impl App {
    /// Create a new application instance with default state
    fn new() -> App {
        App {
            counter: 0,
            pokemon: "None".to_string(),
            should_quit: false,
            selected_option: 0,
            options: vec![
                "Increment Counter".to_string(),
                "Decrement Counter".to_string(),
                "Reset Counter".to_string(),
                "Quit".to_string(),
            ],
            last_tick: Instant::now(),
        }
    }

    /// Handle user input and update application state
    /// This is where you'd process keyboard/mouse events
    fn handle_input(&mut self, event: Event) {
        match event {
            // Handle key press events
            Event::Key(key_event) => {
                match key_event.code {
                    // Arrow keys for navigation
                    KeyCode::Up => {
                        // Move selection up, wrapping around to bottom
                        self.selected_option = if self.selected_option == 0 {
                            self.options.len() - 1
                        } else {
                            self.selected_option - 1
                        };
                    }
                    KeyCode::Down => {
                        // Move selection down, wrapping around to top
                        self.selected_option = (self.selected_option + 1) % self.options.len();
                    }
                    
                    // Enter key to select current option
                    KeyCode::Enter => {
                        self.handle_selection();
                    }
                    
                    // Q key to quit
                    KeyCode::Char('q') => {
                        self.should_quit = true;
                    }
                    
                    // Escape key to quit
                    KeyCode::Esc => {
                        self.should_quit = true;
                    }
                    
                    // Ignore all other keys
                    _ => {}
                }
            }
            
            // Handle mouse events (if needed)
            Event::Mouse(_) => {
                // Mouse handling would go here
            }
            
            // Handle resize events
            Event::Resize(_, _) => {
                // Terminal resize handling would go here
            }
            
            // Ignore other event types
            _ => {}
        }
    }

    /// Process the currently selected menu option
    fn handle_selection(&mut self) {
        match self.selected_option {
            0 => {
                // Increment counter
                self.counter = self.counter.saturating_add(1);
                // Change pokemon based on new number
                self.pokemon = pokemon::pokemon_indexer::get_pokemon_by_number(self.counter as i32);
            }
            1 => {
                // Decrement counter (but don't go below 0)
                self.counter = self.counter.saturating_sub(1);
                // Change pokemon based on new number
                self.pokemon = pokemon::pokemon_indexer::get_pokemon_by_number(self.counter as i32);
            }
            2 => {
                // Reset counter
                self.counter = 0;
            }
            3 => {
                // Quit application
                self.should_quit = true;
            }
            _ => {
                // This shouldn't happen, but handle it gracefully
                eprintln!("Invalid selection: {}", self.selected_option);
            }
        }
    }

    /// Update application state (called on each tick)
    /// This is where you'd handle animations, timers, etc.
    fn tick(&mut self) {
        // Update the last tick time
        self.last_tick = Instant::now();
        
        // You could add periodic updates here, like:
        // - Updating animations
        // - Checking for external data changes
        // - Processing background tasks
    }
}

// ============================================================================
// UI RENDERING FUNCTIONS
// ============================================================================

/// Draw the main user interface
/// This function is called on each frame to render the current state
fn draw_ui(f: &mut Frame, app: &App) {
    // Get the full size of the terminal
    let size = f.size();
     
    // Create the main layout - this divides the screen into sections
    let chunks = Layout::default()
        .direction(Direction::Vertical) // Stack widgets vertically
        .margin(2) // Add some padding around the edges
        .constraints(
            [
                Constraint::Length(3),  // Header section
                Constraint::Min(0),     // Main content area (takes remaining space)
                Constraint::Length(3),  // Footer section
            ]
            .as_ref(),
        )
        .split(size);

    // Draw the header
    draw_header(f, chunks[0]);
    
    // Draw the main content area
    draw_main_content(f, chunks[1], app);
    
    // Draw the footer
    draw_footer(f, chunks[2]);
}

/// Draw the header section of the UI
fn draw_header(f: &mut Frame, area: Rect) {
    // Create a paragraph widget for the title
    let title = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(
                "Pokemon Arena - Ratatui Example",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Use arrow keys to navigate, Enter to select, Q or Esc to quit",
                Style::default().fg(Color::Gray),
            ),
        ]),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Controls"),
    );

    // Render the widget to the specified area
    f.render_widget(title, area);
}

/// Draw the main content area
fn draw_main_content(f: &mut Frame, area: Rect, app: &App) {
    // Split the main area into two columns
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    // Draw the counter display on the left
    draw_counter(f, chunks[0], app);
    
    // Draw the menu on the right
    draw_menu(f, chunks[1], app);
}

/// Draw the counter display widget
fn draw_counter(f: &mut Frame, area: Rect, app: &App) {
    // Create the counter text with styling
    let counter_text = vec![
        Line::from(vec![
            Span::styled(
                format!("Counter: {}", app.counter),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                format!("Pokemon: {}", app.pokemon),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                format!("Last update: {:?} ago", app.last_tick.elapsed()),
                Style::default().fg(Color::Gray),
            ),
        ]),
    ];

    // Create a paragraph widget with a border
    let counter_widget = Paragraph::new(counter_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Counter"),
        )
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(counter_widget, area);
}

/// Draw the menu widget
fn draw_menu(f: &mut Frame, area: Rect, app: &App) {
    // Create menu items with selection highlighting
    let menu_items = app
        .options
        .iter()
        .enumerate()
        .map(|(index, option)| {
            if index == app.selected_option {
                // Highlight the selected option
                Line::from(vec![
                    Span::styled(
                        format!("> {}", option),
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])
            } else {
                // Regular option styling
                Line::from(vec![
                    Span::styled(
                        format!("  {}", option),
                        Style::default().fg(Color::White),
                    ),
                ])
            }
        })
        .collect::<Vec<_>>();

    // Create the menu widget
    let menu_widget = Paragraph::new(menu_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Menu"),
        );

    f.render_widget(menu_widget, area);
}

/// Draw the footer section
fn draw_footer(f: &mut Frame, area: Rect) {
    let footer_text = vec![
        Line::from(vec![
            Span::styled(
                "Built with Ratatui - A Rust library for terminal user interfaces",
                Style::default().fg(Color::Blue),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Press Q or Esc to quit",
                Style::default().fg(Color::Gray),
            ),
        ]),
    ];

    let footer_widget = Paragraph::new(footer_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Info"),
        );

    f.render_widget(footer_widget, area);
}

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
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        eprintln!("Application panicked: {:?}", panic_info);
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
    
    // Create our application state
    let mut app = App::new();

    // ========================================================================
    // MAIN EVENT LOOP
    // ========================================================================
    
    // This is the heart of our application - it runs until we want to quit
    loop {
        // Clear the screen and draw the current UI state
        terminal
            .draw(|f| draw_ui(f, &app))
            .context("Failed to draw UI")?;

        // Check if we should quit
        if app.should_quit {
            break;
        }

        // Handle input events with a timeout
        // This prevents the app from blocking indefinitely
        if crossterm::event::poll(Duration::from_millis(250))
            .context("Failed to poll for events")?
        {
            // Get the next event from the terminal
            if let Ok(event) = event::read() {
                // Process the event and update application state
                app.handle_input(event);
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