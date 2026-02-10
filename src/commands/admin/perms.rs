use irc::client::prelude::Client;
use crate::message::ParsedMessage;
use crate::database::Database;
use crate::commands::utils::is_admin;

pub fn cmd_perms(client: &Client, msg: &ParsedMessage, db: &Database) -> irc::error::Result<()> {
    if let Some(username) = &msg.author {
        if !is_admin(db, username) {
            client.send_privmsg(&msg.channel, "Permission denied. Only admins can view permissions")?;
            return Ok(());
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
