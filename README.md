# Rusty

A feature-rich IRC bot written, using AI slop generation, in Rust with SSL/TLS support, SQLite database integration, and permission-based command access.

## Features

- ✅ SSL/TLS encrypted IRC connections
- ✅ SQLite database for persistent user data and permissions
- ✅ Permission-based command access (3-tier system)
- ✅ User data storage (key-value pairs)
- ✅ Dynamic channel joining/leaving
- ✅ Clean modular architecture

## Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- SQLite3

## Quick Start

### 1. Install and Configure

```bash
chmod +x install.sh
./install.sh
```

This script will:
- Initialize the SQLite database
- Ask for your IRC username
- Grant you admin permissions

### 2. Configure the Bot

Edit `src/main.rs` and update:
- `server`: IRC server address (e.g., `irc.libera.chat`)
- `nickname`: Bot's nickname
- `channels`: Initial channels to join (e.g., `vec!["#channel1", "#channel2"]`)

### 3. Build and Run

```bash
cargo build --release
cargo run --release
```

## Commands

### Public Commands (No permission required)
- `!ping` - Responds with "pong!"
- `!hello` - Greets you
- `!echo <message>` - Echoes back your message
- `!help` - Shows available commands

### Restricted Commands (Permission level >= 1)
- `!join <#channel>` - Bot joins a channel
- `!leave [#channel]` - Bot leaves a channel (current if not specified)
- `!set <key> <value>` - Store personal data
- `!get <key>` - Retrieve stored data
- `!del <key>` - Delete stored data
- `!list` - List all your stored data

### Admin Commands (Permission level >= 10)
- `!grant <username> <level>` - Grant permissions to a user
- `!revoke <username>` - Remove user permissions
- `!perms` - List all users with permissions

## Permission System

- **Level 0** (default): No access, can only use public commands
- **Level 1-9**: Restricted commands access (channel management, data storage)
- **Level 10+**: Admin access (user permission management)

### Grant Permissions Example

```
!grant alice 5      # Give alice restricted access
!grant bob 10       # Make bob an admin
!revoke charlie     # Remove charlie's permissions
```

## Project Structure

```
src/
├── main.rs       - Entry point and event loop
├── message.rs    - Message parsing logic
├── database.rs   - SQLite database operations
└── commands.rs   - Command handlers
bot_data.db      - SQLite database (created on first run)
```

## Configuration

Edit `src/main.rs` to customize:

```rust
let config = Config {
    nickname: Some("rusty".to_string()),
    server: Some("irc.libera.chat".to_string()),
    port: Some(6697),
    use_tls: Some(true),
    channels: vec!["#test".to_string()],
    ..Default::default()
};
```

## Building

Development build:
```bash
cargo build
```

Optimized release build:
```bash
cargo build --release
```

## Database Management

View permissions:
```bash
sqlite3 bot_data.db "SELECT * FROM permissions;"
```

View user data:
```bash
sqlite3 bot_data.db "SELECT * FROM users_data;"
```

Reset admin for a user:
```bash
sqlite3 bot_data.db "UPDATE permissions SET permission_level = 10 WHERE username = 'alice';"
```

## Troubleshooting

**Bot doesn't connect:**
- Check server address and port are correct
- Ensure port 6697 (or your configured port) is not blocked
- Verify TLS is enabled for secure connections

**Database errors:**
- Delete `bot_data.db` and run `./install.sh` again
- Ensure you have write permissions in the project directory

**Permission denied:**
- Use `!perms` (if admin) to check your permission level
- Ask another admin to grant you permissions

## Contributing

Feel free to extend the bot with new commands by adding functions to `src/commands.rs` and wiring them in the `handle_command()` function.

## License

MIT