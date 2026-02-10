use irc::client::prelude::Client;
use crate::message::ParsedMessage;
use crate::database::Database;
use crate::commands::utils::has_permission;

pub fn cmd_leave(client: &Client, msg: &ParsedMessage, db: &Database) -> irc::error::Result<()> {
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
