use irc::client::prelude::Client;
use crate::message::ParsedMessage;

pub fn cmd_ping(client: &Client, msg: &ParsedMessage) -> irc::error::Result<()> {
    client.send_privmsg(&msg.channel, "pong!")
}
