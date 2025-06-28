//! # UI Module
//! 
//! This module handles different UI states and provides a clean interface
//! for switching between different screens in the application.
//! 
//! ## Structure
//! - `states/` - Contains different UI state implementations
//! - `renderer.rs` - Handles the main UI rendering logic
//! - `input_handler.rs` - Processes user input for different states

pub mod states;
pub mod renderer;
pub mod input_handler;

// Re-export commonly used types
pub use states::App;
pub use renderer::draw_ui;
pub use input_handler::handle_input; 