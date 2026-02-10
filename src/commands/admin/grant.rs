use irc::client::prelude::Client;
use crate::message::ParsedMessage;
use crate::database::Database;
use crate::commands::utils::is_admin;

pub fn cmd_grant(client: &Client, msg: &ParsedMessage, db: &Database) -> irc::error::Result<()> {
    if let Some(username) = &msg.author {
        if !is_admin(db, username) {
            client.send_privmsg(&msg.channel, "Permission denied. Only admins can grant permissions")?;
            return Ok(());
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
