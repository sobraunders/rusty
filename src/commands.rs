use irc::client::prelude::Client;
use crate::message::ParsedMessage;
use crate::database::Database;

pub fn handle_command(client: &Client, msg: &ParsedMessage, db: &Database) -> irc::error::Result<()> {
    match msg.command.as_str() {
        "ping" => cmd_ping(client, msg),
        "hello" => cmd_hello(client, msg),
        "echo" => cmd_echo(client, msg),
        "join" => cmd_join(client, msg, db),
        "leave" => cmd_leave(client, msg, db),
        "set" => cmd_set(client, msg, db),
        "get" => cmd_get(client, msg, db),
        "del" => cmd_del(client, msg, db),
        "list" => cmd_list(client, msg, db),
        "grant" => cmd_grant(client, msg, db),
        "revoke" => cmd_revoke(client, msg, db),
        "perms" => cmd_perms(client, msg, db),
        "help" => cmd_help(client, msg),
        _ => cmd_unknown(client, msg),
    }
}

fn has_permission(db: &Database, username: &str) -> bool {
    match db.get_permission_level(username) {
        Ok(level) => level > 0,
        Err(_) => false,
    }
}

fn cmd_ping(client: &Client, msg: &ParsedMessage) -> irc::error::Result<()> {
    client.send_privmsg(&msg.channel, "pong!")
}

fn cmd_hello(client: &Client, msg: &ParsedMessage) -> irc::error::Result<()> {
    let greeting = if let Some(author) = &msg.author {
        format!("Hello, {}!", author)
    } else {
        "Hello there!".to_string()
    };
    client.send_privmsg(&msg.channel, &greeting)
}

fn cmd_echo(client: &Client, msg: &ParsedMessage) -> irc::error::Result<()> {
    if msg.args.is_empty() {
        client.send_privmsg(&msg.channel, "Usage: !echo <message>")?;
    } else {
        let echo_text = msg.args.join(" ");
        client.send_privmsg(&msg.channel, &echo_text)?;
    }
    Ok(())
}

fn cmd_join(client: &Client, msg: &ParsedMessage, db: &Database) -> irc::error::Result<()> {
    if let Some(username) = &msg.author {
        if !has_permission(db, username) {
            client.send_privmsg(&msg.channel, "Permission denied. You don't have permission to use this command")?;
            return Ok(());
        }
    } else {
        client.send_privmsg(&msg.channel, "You must be logged in to use this command")?;
        return Ok(());
    }

    if msg.args.is_empty() {
        client.send_privmsg(&msg.channel, "Usage: !join <#channel>")?;
    } else {
        let channel = &msg.args[0];
        if !channel.starts_with('#') {
            client.send_privmsg(&msg.channel, "Channel name must start with #")?;
        } else {
            client.send_join(channel)?;
            client.send_privmsg(&msg.channel, &format!("Joining {}", channel))?;
        }
    }
    Ok(())
}

fn cmd_leave(client: &Client, msg: &ParsedMessage, db: &Database) -> irc::error::Result<()> {
    if let Some(username) = &msg.author {
        if !has_permission(db, username) {
            client.send_privmsg(&msg.channel, "Permission denied. You don't have permission to use this command")?;
            return Ok(());
        }
    } else {
        client.send_privmsg(&msg.channel, "You must be logged in to use this command")?;
        return Ok(());
    }

    let channel = if msg.args.is_empty() {
        msg.channel.clone()
    } else {
        msg.args[0].clone()
    };
    
    client.send_part(&channel)?;
    Ok(())
}

fn cmd_set(client: &Client, msg: &ParsedMessage, db: &Database) -> irc::error::Result<()> {
    if let Some(username) = &msg.author {
        if msg.args.len() < 2 {
            client.send_privmsg(&msg.channel, "Usage: !set <key> <value>")?;
        } else {
            let key = &msg.args[0];
            let value = msg.args[1..].join(" ");
            
            match db.set_user_data(username, key, &value) {
                Ok(_) => {
                    client.send_privmsg(&msg.channel, &format!("Saved: {} = {}", key, value))?;
                }
                Err(e) => {
                    client.send_privmsg(&msg.channel, &format!("Error saving data: {}", e))?;
                }
            }
        }
    } else {
        client.send_privmsg(&msg.channel, "You must be logged in to use this command")?;
    }
    Ok(())
}

fn cmd_get(client: &Client, msg: &ParsedMessage, db: &Database) -> irc::error::Result<()> {
    if let Some(username) = &msg.author {
        if msg.args.is_empty() {
            client.send_privmsg(&msg.channel, "Usage: !get <key>")?;
        } else {
            let key = &msg.args[0];
            
            match db.get_user_data(username, key) {
                Ok(Some(value)) => {
                    client.send_privmsg(&msg.channel, &format!("{} = {}", key, value))?;
                }
                Ok(None) => {
                    client.send_privmsg(&msg.channel, &format!("Key not found: {}", key))?;
                }
                Err(e) => {
                    client.send_privmsg(&msg.channel, &format!("Error retrieving data: {}", e))?;
                }
            }
        }
    } else {
        client.send_privmsg(&msg.channel, "You must be logged in to use this command")?;
    }
    Ok(())
}

fn cmd_del(client: &Client, msg: &ParsedMessage, db: &Database) -> irc::error::Result<()> {
    if let Some(username) = &msg.author {
        if msg.args.is_empty() {
            client.send_privmsg(&msg.channel, "Usage: !del <key>")?;
        } else {
            let key = &msg.args[0];
            
            match db.delete_user_data(username, key) {
                Ok(true) => {
                    client.send_privmsg(&msg.channel, &format!("Deleted: {}", key))?;
                }
                Ok(false) => {
                    client.send_privmsg(&msg.channel, &format!("Key not found: {}", key))?;
                }
                Err(e) => {
                    client.send_privmsg(&msg.channel, &format!("Error deleting data: {}", e))?;
                }
            }
        }
    } else {
        client.send_privmsg(&msg.channel, "You must be logged in to use this command")?;
    }
    Ok(())
}

fn cmd_list(client: &Client, msg: &ParsedMessage, db: &Database) -> irc::error::Result<()> {
    if let Some(username) = &msg.author {
        match db.list_user_data(username) {
            Ok(data) => {
                if data.is_empty() {
                    client.send_privmsg(&msg.channel, "No stored data")?;
                } else {
                    for (key, value) in data {
                        client.send_privmsg(&msg.channel, &format!("{}: {}", key, value))?;
                    }
                }
            }
            Err(e) => {
                client.send_privmsg(&msg.channel, &format!("Error listing data: {}", e))?;
            }
        }
    } else {
        client.send_privmsg(&msg.channel, "You must be logged in to use this command")?;
    }
    Ok(())
}

fn cmd_help(client: &Client, msg: &ParsedMessage) -> irc::error::Result<()> {
    client.send_privmsg(&msg.channel, "Public: !ping, !hello, !echo <msg>, !help")?;
    client.send_privmsg(&msg.channel, "Restricted: !join <#ch>, !leave <#ch>, !set <k> <v>, !get <k>, !del <k>, !list")?;
    client.send_privmsg(&msg.channel, "Admin: !grant <user> <level>, !revoke <user>, !perms")?;
    Ok(())
}

fn cmd_grant(client: &Client, msg: &ParsedMessage, db: &Database) -> irc::error::Result<()> {
    // Only allow users with permission level >= 10 (admin)
    if let Some(username) = &msg.author {
        match db.get_permission_level(username) {
            Ok(level) => {
                if level < 10 {
                    client.send_privmsg(&msg.channel, "Permission denied. Only admins can grant permissions")?;
                    return Ok(());
                }
            }
            Err(e) => {
                client.send_privmsg(&msg.channel, &format!("Error checking permissions: {}", e))?;
                return Ok(());
            }
        }
    } else {
        client.send_privmsg(&msg.channel, "You must be logged in to use this command")?;
        return Ok(());
    }

    if msg.args.len() < 2 {
        client.send_privmsg(&msg.channel, "Usage: !grant <user> <level>")?;
    } else {
        let target_user = &msg.args[0];
        let level_str = &msg.args[1];
        
        match level_str.parse::<i32>() {
            Ok(level) => {
                match db.grant_permission(target_user, level) {
                    Ok(_) => {
                        client.send_privmsg(&msg.channel, &format!("Granted permission level {} to {}", level, target_user))?;
                    }
                    Err(e) => {
                        client.send_privmsg(&msg.channel, &format!("Error granting permission: {}", e))?;
                    }
                }
            }
            Err(_) => {
                client.send_privmsg(&msg.channel, "Permission level must be a number")?;
            }
        }
    }
    Ok(())
}

fn cmd_revoke(client: &Client, msg: &ParsedMessage, db: &Database) -> irc::error::Result<()> {
    // Only allow users with permission level >= 10 (admin)
    if let Some(username) = &msg.author {
        match db.get_permission_level(username) {
            Ok(level) => {
                if level < 10 {
                    client.send_privmsg(&msg.channel, "Permission denied. Only admins can revoke permissions")?;
                    return Ok(());
                }
            }
            Err(e) => {
                client.send_privmsg(&msg.channel, &format!("Error checking permissions: {}", e))?;
                return Ok(());
            }
        }
    } else {
        client.send_privmsg(&msg.channel, "You must be logged in to use this command")?;
        return Ok(());
    }

    if msg.args.is_empty() {
        client.send_privmsg(&msg.channel, "Usage: !revoke <user>")?;
    } else {
        let target_user = &msg.args[0];
        
        match db.revoke_permission(target_user) {
            Ok(true) => {
                client.send_privmsg(&msg.channel, &format!("Revoked permissions for {}", target_user))?;
            }
            Ok(false) => {
                client.send_privmsg(&msg.channel, &format!("User {} has no permissions", target_user))?;
            }
            Err(e) => {
                client.send_privmsg(&msg.channel, &format!("Error revoking permission: {}", e))?;
            }
        }
    }
    Ok(())
}

fn cmd_perms(client: &Client, msg: &ParsedMessage, db: &Database) -> irc::error::Result<()> {
    if let Some(username) = &msg.author {
        match db.get_permission_level(username) {
            Ok(level) => {
                if level < 10 {
                    client.send_privmsg(&msg.channel, "Permission denied. Only admins can view permissions")?;
                    return Ok(());
                }
            }
            Err(e) => {
                client.send_privmsg(&msg.channel, &format!("Error checking permissions: {}", e))?;
                return Ok(());
            }
        }
    } else {
        client.send_privmsg(&msg.channel, "You must be logged in to use this command")?;
        return Ok(());
    }

    match db.list_users_with_permissions() {
        Ok(users) => {
            if users.is_empty() {
                client.send_privmsg(&msg.channel, "No users with permissions")?;
            } else {
                client.send_privmsg(&msg.channel, "Users with permissions:")?;
                for (user, level) in users {
                    client.send_privmsg(&msg.channel, &format!("  {} - Level {}", user, level))?;
                }
            }
        }
        Err(e) => {
            client.send_privmsg(&msg.channel, &format!("Error listing permissions: {}", e))?;
        }
    }
    Ok(())
}

fn cmd_unknown(client: &Client, msg: &ParsedMessage) -> irc::error::Result<()> {
    client.send_privmsg(&msg.channel, &format!("Unknown command: {}", msg.command))
}
