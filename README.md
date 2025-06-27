# Pokemon Arena - Ratatui Example

A comprehensive example of using [ratatui](https://github.com/ratatui-org/ratatui) to build a terminal user interface in Rust. This project demonstrates key concepts and patterns for building interactive terminal applications.

## Features

This example showcases:

- **Terminal Setup**: Proper initialization and cleanup of terminal state
- **Event Handling**: Keyboard input processing with arrow key navigation
- **State Management**: Application state with counter and menu selection
- **UI Layout**: Complex layouts with headers, content areas, and footers
- **Widgets**: Various ratatui widgets including Paragraph, Block, and Borders
- **Styling**: Text styling with colors, modifiers, and highlighting
- **Error Handling**: Comprehensive error handling with anyhow
- **Async Support**: Tokio integration for async operations

## Project Structure

```
pokemon_arena/
├── Cargo.toml          # Project dependencies and metadata
├── src/
│   └── main.rs         # Main application code with detailed comments
└── README.md           # This file
```

## Dependencies

- **ratatui**: Core terminal UI library
- **crossterm**: Terminal backend for cross-platform support
- **anyhow**: Error handling library
- **tokio**: Async runtime

## How to Run

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone and navigate to the project**:
   ```bash
   cd pokemon_arena
   ```

3. **Build and run**:
   ```bash
   cargo run
   ```

## Controls

- **Arrow Keys**: Navigate through menu options
- **Enter**: Select the highlighted menu option
- **Q or Esc**: Quit the application

## Key Concepts Demonstrated

### 1. Application State Management

The `App` struct holds all application state:
```rust
struct App {
    counter: u32,           // Current counter value
    should_quit: bool,      // Exit flag
    selected_option: usize, // Menu selection
    options: Vec<String>,   // Menu options
    last_tick: Instant,     // Timer for updates
}
```

### 2. Event Loop Pattern

The main event loop handles:
- Input events (keyboard, mouse, resize)
- UI rendering
- State updates
- Cleanup on exit

### 3. Layout System

Ratatui's layout system divides the screen into sections:
```rust
let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),  // Header
        Constraint::Min(0),     // Main content
        Constraint::Length(3),  // Footer
    ])
    .split(size);
```

### 4. Widget Rendering

Widgets are rendered to specific areas:
```rust
f.render_widget(widget, area);
```

## Building Upon This Example

This example provides a solid foundation for building more complex terminal applications. You can extend it by:

1. **Adding More Widgets**: Tables, charts, progress bars, etc.
2. **Implementing Navigation**: Multiple screens/pages
3. **Adding Data Persistence**: Save/load application state
4. **Network Integration**: Fetch data from APIs
5. **Real-time Updates**: Live data updates and animations
6. **Configuration**: User preferences and settings

## Common Patterns

### State Updates
```rust
fn handle_input(&mut self, event: Event) {
    match event {
        Event::Key(key_event) => {
            match key_event.code {
                KeyCode::Up => { /* handle up */ }
                KeyCode::Down => { /* handle down */ }
                // ... more handlers
            }
        }
        // ... other event types
    }
}
```

### Widget Creation
```rust
let widget = Paragraph::new(text)
    .block(Block::default().borders(Borders::ALL).title("Title"))
    .alignment(Alignment::Center);
```

### Error Handling
```rust
fn run_app() -> Result<()> {
    enable_raw_mode().context("Failed to enable raw mode")?;
    // ... rest of setup
    Ok(())
}
```

## Troubleshooting

### Common Issues

1. **Terminal not responding**: Make sure your terminal supports the features used by crossterm
2. **Colors not displaying**: Some terminals may not support all color modes
3. **Input not working**: Check if your terminal is in the correct mode

### Debug Mode

To run with debug information:
```bash
RUST_LOG=debug cargo run
```

## Resources

- [Ratatui Documentation](https://docs.rs/ratatui)
- [Ratatui Examples](https://github.com/ratatui-org/ratatui/tree/main/examples)
- [Crossterm Documentation](https://docs.rs/crossterm)
- [Rust Book](https://doc.rust-lang.org/book/)

## Contributing

Feel free to extend this example and submit improvements! The code is heavily commented to help you understand each concept.

## License

This project is open source and available under the MIT License. 