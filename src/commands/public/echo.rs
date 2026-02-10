use irc::client::prelude::Client;
use crate::message::ParsedMessage;

pub fn cmd_echo(client: &Client, msg: &ParsedMessage) -> irc::error::Result<()> {
    if msg.args.is_empty() {
        client.send_privmsg(&msg.channel, "Usage: !echo <message>")?;
    } else {
        let echo_text = msg.args.join(" ");
        client.send_privmsg(&msg.channel, &echo_text)?;
    }
    Ok(())
}
