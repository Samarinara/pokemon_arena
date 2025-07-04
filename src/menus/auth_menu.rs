use tracing::{info, error, warn};
use ratatui::{
    backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, 
    style::{Color, Style}, 
    widgets::{Block, Borders, List, ListItem, Paragraph}, 
    Frame, 
    Terminal
};

use crate::user_management::email_auth::{send_auth_email, verify_email};
use crate::{App as MainApp, AppState, AuthState}; // NEW: Import AuthState
use crate::restore_terminal;
use crate::ui_tooling::text_input::draw_text_input;
use crate::pokemon::pokemon_indexer;
use rand::Rng;

use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;


static KEY: std::sync::OnceLock<Arc<Mutex<i32>>> = std::sync::OnceLock::new();

pub fn menu(f: &mut Frame<'_>, app: &MainApp) {
    match app.auth_state {
        AuthState::InputEmail => draw_email_input(f, app),
        AuthState::VerifyEmail => draw_key_input(f, app),
        _ => {}
    }
}

pub async fn input(app: &mut MainApp){
    info!("auth_menu::input called");
    match app.auth_state {
        AuthState::InputEmail => input_email(app).await,
        AuthState::VerifyEmail => input_key(app).await,
        _ => {}
    }
}

pub async fn generate_key() -> String {
    let rng1 = rand::thread_rng().gen_range(0..=151);
    let pokemon_name1 = pokemon_indexer::get_pokemon_by_number(rng1).await;
    let rng2 = rand::thread_rng().gen_range(0..=151);
    let pokemon_name2 = pokemon_indexer::get_pokemon_by_number(rng2).await;
    let key = pokemon_name1 + " " + &pokemon_name2;
    return key;
}

async fn input_email(app: &mut MainApp) {
    info!("input_email called");
    match app.selected {
        0 => { // Send Email
            info!("Sending verification email");
            let pokemon_name = generate_key().await;
            app.user_email = app.email_input.input.to_string();
            app.verification_code = pokemon_name.clone();
            if let Err(e) = send_auth_email(pokemon_name, &app.user_email).await {
                error!("Failed to send email: {}", e);
            }
            app.auth_state = AuthState::VerifyEmail;
            app.selected = 0;
        }
        1 => { // Exit
            info!("Exiting from input_email");
            let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout())).unwrap();
            restore_terminal(&mut terminal).unwrap();
            let _ = quit::with_code(0);
        }
        _ => {}
    }
}

async fn input_key(app: &mut MainApp) {
    info!("input_key called");
    match app.selected {
        0 => { // Submit
            info!("Submit button pressed");

            if verify_email(app.verification_code.as_str(), app.user_email.as_str(), app.email_input.input.as_str()){
                app.state = AppState::MainMenu;
                app.auth_state = AuthState::LoggedIn;
            }else{
                if app.strikes < 3{
                    app.state = AppState::Auth;
                    app.auth_state = AuthState::VerifyEmail;
                    app.strikes += 1;
                }else{
                    app.state = AppState::Auth;
                    app.auth_state = AuthState::InputEmail;
                    app.strikes = 0;
                }
            }
            
        }
        1 => { // Resend Email
            info!("Resend Email button pressed");
             if let Err(e) = send_auth_email(app.verification_code.clone(), &app.email_input.input.to_string()).await {
                error!("Failed to send email: {}", e);
            } 
        }
        2 => { // Change Email
            info!("Change Email button pressed");
            app.auth_state = AuthState::InputEmail;
        }
        3 => { // Exit
            info!("Exiting from input_key");
            let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout())).unwrap();
            restore_terminal(&mut terminal).unwrap();
            let _ = quit::with_code(0);
        }
        _ => {}
    }
}


fn draw_email_input(f: &mut Frame<'_>, app: &MainApp) {
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
            Constraint::Percentage(40),
            Constraint::Percentage(40),
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
    f.render_widget(menu, vert_chunks[1]);

    // Draw the persistent email input widget
    draw_text_input(f, vert_chunks[0], &app.email_input, "Enter your email:");


    let footer = Paragraph::new(" ↑/↓ to navigate\n Enter to select\n Tab to toggle typing\n Esc to quit")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, vert_chunks[2]);
}


fn draw_key_input(f: &mut Frame<'_>, app: &MainApp) {
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
            Constraint::Percentage(10),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ])
        .split(horiz_chunks[1]);

    

    let menu_items: &[&str] = &["Submit", "Resend Email", "Change Email", "Exit"];
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
    draw_text_input(f, vert_chunks[0], &app.email_input, "Verification Code:");

    let strikes = format!("Strikes: {}", app.strikes);
    let strikes_paragraph = Paragraph::new(strikes)
        .block(Block::default().borders(Borders::ALL).title("Strikes"))
        .style(Style::default().fg(Color::Red));
        f.render_widget(strikes_paragraph, vert_chunks[1]);


    let footer = Paragraph::new(" ↑/↓ to navigate\n Enter to select\n Tab to toggle typing\n Esc to quit")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, vert_chunks[3]);
}