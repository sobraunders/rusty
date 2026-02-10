use irc::client::prelude::Client;
use crate::message::ParsedMessage;

pub fn cmd_unknown(client: &Client, msg: &ParsedMessage) -> irc::error::Result<()> {
    client.send_privmsg(&msg.channel, &format!("Unknown command: {}", msg.command))
}
