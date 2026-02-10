use irc::client::prelude::Client;
use crate::message::ParsedMessage;

pub fn cmd_hello(client: &Client, msg: &ParsedMessage) -> irc::error::Result<()> {
    let greeting = if let Some(author) = &msg.author {
        format!("Hello, {}!", author)
    } else {
        "Hello there!".to_string()
    };
    client.send_privmsg(&msg.channel, &greeting)
}
