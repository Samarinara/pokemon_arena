# Centralized Menu System

This directory contains a centralized menu system that handles navigation for all menus consistently, making it easy to add new menus without worrying about navigation issues.

## Overview

The menu system consists of:
- `menu_system.rs` - Centralized menu management
- `auth_menu.rs` - Authentication-specific menu logic
- `mod.rs` - Module exports

## How It Works

### Menu Structure
Each menu is defined by a `Menu` struct containing:
- `items`: Vector of menu item strings
- `title`: Menu title for display

### MenuSystem
The `MenuSystem` provides centralized functions for:
- `get_current_menu()` - Get menu based on app state
- `get_menu_size()` - Get number of menu items
- `handle_up_arrow()` - Handle up navigation
- `handle_down_arrow()` - Handle down navigation
- `clamp_selection()` - Ensure selection is within bounds

## Adding a New Menu

### Step 1: Add AppState
Add your new state to the `AppState` enum in `src/main.rs`:

```rust
#[derive(PartialEq, Clone, Copy)]
pub enum AppState {
    MainMenu,
    Settings,
    Pokedex,
    YourNewMenu,  // Add this
    Auth,
}
```

### Step 2: Define Menu Items
Add your menu to `MenuSystem::get_current_menu()` in `src/menus/menu_system.rs`:

```rust
pub fn get_current_menu(app_state: AppState, auth_state: AuthState) -> Menu {
    match app_state {
        // ... existing cases ...
        AppState::YourNewMenu => Menu::new(
            vec!["Option 1", "Option 2", "Option 3", "Back"],
            "Your New Menu"
        ),
        // ... rest of cases ...
    }
}
```

### Step 3: Add Navigation Logic
Update the Enter key handling in `src/client_handler.rs`:

```rust
AppState::YourNewMenu => {
    match self.app.selected {
        0 => {
            // Handle Option 1
        }
        1 => {
            // Handle Option 2
        }
        2 => {
            // Handle Option 3
        }
        3 => {
            // Back to main menu
            self.app.state = AppState::MainMenu;
            self.app.selected = 0;
        }
        _ => {}
    }
}
```

### Step 4: Add Escape Handling
The escape key handling is already generalized, but you can add specific logic if needed.

### Step 5: Create Menu Function
Add your menu rendering function in `src/main.rs`:

```rust
fn your_new_menu(f: &mut Frame<>, app: &App) {
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
        .block(Block::default().borders(Borders::ALL).title("Your Menu"))
        .style(Style::default().fg(Color::Blue));
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

    let footer = Paragraph::new("Use arrow keys to navigate, Enter to select, Esc to go back")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}
```

### Step 6: Update Draw Function
Add your menu to the draw function in `src/client_handler.rs`:

```rust
terminal.draw(|f: &mut Frame<'_>| {
    match app.state {
        AppState::MainMenu => main_menu(f, app),
        AppState::Settings => settings(f, app),
        AppState::Pokedex => pokedex(f, app),
        AppState::YourNewMenu => your_new_menu(f, app),  // Add this
        AppState::Auth => {
            auth_menu::menu(f, app);
        }
    }
})?;
```

## Benefits

1. **Consistent Navigation**: All menus use the same up/down arrow logic
2. **Automatic Bounds Checking**: Selection is automatically clamped to valid ranges
3. **Easy to Add**: Adding new menus requires minimal code changes
4. **Centralized Management**: All menu definitions are in one place
5. **Type Safety**: Compile-time checking of menu states

## Example: Pokedex Menu

The Pokedex menu demonstrates the complete implementation:
- Added `Pokedex` to `AppState`
- Defined menu items in `MenuSystem::get_current_menu()`
- Added navigation logic in `client_handler.rs`
- Created `pokedex()` rendering function
- Updated draw function to include Pokedex

All navigation (up/down arrows, selection clamping, escape handling) works automatically without additional code. 