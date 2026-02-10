use irc::client::prelude::Client;
use crate::message::ParsedMessage;
use crate::database::Database;
use crate::commands::utils::is_admin;

pub fn cmd_revoke(client: &Client, msg: &ParsedMessage, db: &Database) -> irc::error::Result<()> {
    if let Some(username) = &msg.author {
        if !is_admin(db, username) {
            client.send_privmsg(&msg.channel, "Permission denied. Only admins can revoke permissions")?;
            return Ok(());
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
