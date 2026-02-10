use irc::client::prelude::Client;
use crate::message::ParsedMessage;

pub fn cmd_help(client: &Client, msg: &ParsedMessage) -> irc::error::Result<()> {
    client.send_privmsg(&msg.channel, "Public: !ping, !hello, !echo <msg>, !help")?;
    client.send_privmsg(&msg.channel, "Restricted: !join <#ch>, !leave <#ch>, !set <k> <v>, !get <k>, !del <k>, !list")?;
    client.send_privmsg(&msg.channel, "Admin: !grant <user> <level>, !revoke <user>, !perms")?;
    Ok(())
}
