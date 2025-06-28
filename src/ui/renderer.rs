//! # UI Renderer Module
//! 
//! This module handles rendering different UI states to the terminal screen.
//! It provides a unified interface for drawing all application states.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::ui::states::{App, AppState};

/// Draw the main user interface based on current application state
/// This function routes rendering to the appropriate state renderer
pub fn draw_ui(f: &mut Frame, app: &App) {
    // Get the full size of the terminal
    let size = f.size();
     
    // Create the main layout - this divides the screen into sections
    let chunks = Layout::default()
        .direction(Direction::Vertical) // Stack widgets vertically
        .margin(3) // Add some padding around the edges
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
    draw_header(f, chunks[0], app);
    
    // Draw the main content area based on current state
    draw_main_content(f, chunks[1], app);
    
    // Draw the footer
    draw_footer(f, chunks[2], app);
}

/// Draw the header section of the UI
fn draw_header(f: &mut Frame, area: Rect, app: &App) {
    // Create header text based on current state
    let (title, subtitle) = match app.current_state() {
        AppState::MainMenu => (
            "Pokemon Arena - Main Menu",
            "Use arrow keys to navigate, Enter to select, Esc to quit"
        ),
        AppState::Pokedex => (
            "Pokemon Arena - Pokedex",
            "Browse Pokemon: Arrow keys to navigate, Enter for details, / to search"
        ),
        AppState::Settings => (
            "Pokemon Arena - Settings",
            "Configure application: Arrow keys to navigate, Enter to modify"
        ),
        AppState::Help => (
            "Pokemon Arena - Help",
            "Navigation: Arrow keys to browse sections, Enter to go back"
        ),
    };

    // Create a paragraph widget for the title
    let title_widget = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(
                title,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                subtitle,
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
    f.render_widget(title_widget, area);
}

/// Draw the main content area based on current application state
fn draw_main_content(f: &mut Frame, area: Rect, app: &App) {
    match app.current_state() {
        AppState::MainMenu => draw_main_menu_content(f, area, app),
        AppState::Pokedex => draw_pokedex_content(f, area, app),
        AppState::Settings => draw_settings_content(f, area, app),
        AppState::Help => draw_help_content(f, area, app),
    }
}

/// Draw the main menu content
fn draw_main_menu_content(f: &mut Frame, area: Rect, app: &App) {
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
                format!("Counter: {}", app.main_menu.counter),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                format!("Pokemon: {}", app.main_menu.pokemon),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                format!("Last update: {:?} ago", app.state_manager().last_tick.elapsed()),
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
    let menu_items = app.main_menu
        .options
        .iter()
        .enumerate()
        .map(|(index, option)| {
            if index == app.main_menu.selected_option {
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

/// Draw the pokedex content
fn draw_pokedex_content(f: &mut Frame, area: Rect, app: &App) {
    // Split the area into three sections: number input, stats, and sprite
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),  // Number input
            Constraint::Percentage(40),  // Stats
            Constraint::Percentage(30),  // ASCII sprite placeholder
        ].as_ref())
        .split(area);

    // Draw the number input on the left
    draw_pokedex_number_input(f, chunks[0], app);
    
    // Draw the stats in the middle
    draw_pokedex_stats(f, chunks[1], app);
    
    // Draw the ASCII sprite placeholder on the right
    draw_pokedex_sprite_placeholder(f, chunks[2], app);
}

/// Draw the Pokemon number input widget
fn draw_pokedex_number_input(f: &mut Frame, area: Rect, app: &App) {
    let number_text = vec![
        Line::from(vec![
            Span::styled(
                format!("Pokemon #{}", app.pokedex.get_pokemon_number()),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                format!("Name: {}", app.pokedex.get_pokemon_name()),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Use +/- or arrow keys",
                Style::default().fg(Color::Gray),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "to change Pokemon",
                Style::default().fg(Color::Gray),
            ),
        ]),
    ];

    let number_widget = Paragraph::new(number_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Pokemon Selector"),
        )
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(number_widget, area);
}

/// Draw the Pokemon stats widget
fn draw_pokedex_stats(f: &mut Frame, area: Rect, app: &App) {
    let stats_text = if let Some(stats) = app.pokedex.get_pokemon_stats() {
        vec![
            Line::from(vec![
                Span::styled(
                    format!("Pokemon: {} (#{})", app.pokedex.get_pokemon_name(), app.pokedex.get_pokemon_number()),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("Type: {} {}", stats.type1, if stats.type2 != "none" { &stats.type2 } else { "" }),
                    Style::default().fg(Color::Blue),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("HP: {}", stats.hp),
                    Style::default().fg(Color::Green),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("Attack: {}", stats.attack),
                    Style::default().fg(Color::Red),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("Defense: {}", stats.defense),
                    Style::default().fg(Color::Cyan),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("Sp. Attack: {}", stats.sp_attack),
                    Style::default().fg(Color::Magenta),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("Sp. Defense: {}", stats.sp_defense),
                    Style::default().fg(Color::Yellow),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("Speed: {}", stats.speed),
                    Style::default().fg(Color::White),
                ),
            ]),
        ]
    } else {
        vec![
            Line::from(vec![
                Span::styled(
                    "Error loading Pokemon stats",
                    Style::default().fg(Color::Red),
                ),
            ]),
        ]
    };

    let stats_widget = Paragraph::new(stats_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Pokemon Stats"),
        )
        .alignment(ratatui::layout::Alignment::Left);

    f.render_widget(stats_widget, area);
}

/// Draw the ASCII sprite placeholder widget
fn draw_pokedex_sprite_placeholder(f: &mut Frame, area: Rect, app: &App) {
    let sprite_text = vec![
        Line::from(vec![
            Span::styled(
                "ASCII Sprite",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Placeholder",
                Style::default().fg(Color::Gray),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Coming Soon!",
                Style::default().fg(Color::Gray),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "",
                Style::default().fg(Color::Gray),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "   ___",
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  /   \\",
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                " |     |",
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  \\___/",
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "",
                Style::default().fg(Color::Gray),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Future ASCII art",
                Style::default().fg(Color::Gray),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "will go here",
                Style::default().fg(Color::Gray),
            ),
        ]),
    ];

    let sprite_widget = Paragraph::new(sprite_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Sprite"),
        )
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(sprite_widget, area);
}

/// Draw the settings content
fn draw_settings_content(f: &mut Frame, area: Rect, app: &App) {
    // Create settings items with current values
    let settings_items = app.settings
        .options
        .iter()
        .enumerate()
        .map(|(index, option)| {
            let value_text = match index {
                0 => format!(": {}", app.settings.get_theme_name()),
                1 => format!(": {}", if app.settings.show_animations { "On" } else { "Off" }),
                2 => format!(": {}ms", app.settings.refresh_rate),
                _ => String::new(),
            };

            if index == app.settings.selected_option {
                // Highlight the selected option
                Line::from(vec![
                    Span::styled(
                        format!("> {}{}", option, value_text),
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
                        format!("  {}{}", option, value_text),
                        Style::default().fg(Color::White),
                    ),
                ])
            }
        })
        .collect::<Vec<_>>();

    let settings_widget = Paragraph::new(settings_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Settings"),
        );

    f.render_widget(settings_widget, area);
}

/// Draw the help content
fn draw_help_content(f: &mut Frame, area: Rect, app: &App) {
    // Split area into sections list and content
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(area);

    // Draw sections list on the left
    draw_help_sections(f, chunks[0], app);
    
    // Draw help content on the right
    draw_help_content_area(f, chunks[1], app);
}

/// Draw the help sections list
fn draw_help_sections(f: &mut Frame, area: Rect, app: &App) {
    let section_items = app.help
        .sections
        .iter()
        .enumerate()
        .map(|(index, section)| {
            if index == app.help.selected_section {
                // Highlight the selected section
                Line::from(vec![
                    Span::styled(
                        format!("> {}", section),
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])
            } else {
                // Regular section styling
                Line::from(vec![
                    Span::styled(
                        format!("  {}", section),
                        Style::default().fg(Color::White),
                    ),
                ])
            }
        })
        .collect::<Vec<_>>();

    let sections_widget = Paragraph::new(section_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help Sections"),
        );

    f.render_widget(sections_widget, area);
}

/// Draw the help content area
fn draw_help_content_area(f: &mut Frame, area: Rect, app: &App) {
    let content_lines = app.help
        .get_current_content()
        .iter()
        .map(|line| {
            if line.is_empty() {
                Line::from(vec![Span::raw("")])
            } else {
                Line::from(vec![Span::styled(line, Style::default().fg(Color::White))])
            }
        })
        .collect::<Vec<_>>();

    let content_widget = Paragraph::new(content_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help Content"),
        );

    f.render_widget(content_widget, area);
}

/// Draw the footer section
fn draw_footer(f: &mut Frame, area: Rect, app: &App) {
    let footer_text = match app.current_state() {
        AppState::MainMenu => vec![
            Line::from(vec![
                Span::styled(
                    "Use arrow keys to navigate, Enter to select, Esc to quit",
                    Style::default().fg(Color::Blue),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "Press F1 for help, or select Help from the menu",
                    Style::default().fg(Color::Gray),
                ),
            ]),
        ],
        AppState::Pokedex => vec![
            Line::from(vec![
                Span::styled(
                    "Arrow keys: Navigate | +/-: Change Pokemon | Esc: Back",
                    Style::default().fg(Color::Blue),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("Viewing: {} of 151", app.pokedex.get_pokemon_number()),
                    Style::default().fg(Color::Gray),
                ),
            ]),
        ],
        AppState::Settings => vec![
            Line::from(vec![
                Span::styled(
                    "Arrow keys: Navigate | Enter: Modify | +/-: Adjust values | Esc: Back",
                    Style::default().fg(Color::Blue),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "Changes are applied immediately",
                    Style::default().fg(Color::Gray),
                ),
            ]),
        ],
        AppState::Help => vec![
            Line::from(vec![
                Span::styled(
                    "Arrow keys: Browse sections | Enter: Go back | Esc: Go back",
                    Style::default().fg(Color::Blue),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("Section: {} of {}", app.help.selected_section + 1, app.help.section_count()),
                    Style::default().fg(Color::Gray),
                ),
            ]),
        ],
    };

    let footer_widget = Paragraph::new(footer_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Info"),
        );

    f.render_widget(footer_widget, area);
}