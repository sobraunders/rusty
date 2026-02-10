mod utils;
pub mod public;
pub mod restricted;
pub mod admin;

use irc::client::prelude::Client;
use crate::message::ParsedMessage;
use crate::database::Database;
use crate::games::GameManager;

pub async fn handle_command(client: &Client, msg: &ParsedMessage, db: &Database, games: &GameManager) -> irc::error::Result<()> {
    match msg.command.as_str() {
        // Public commands
        "ping" => public::cmd_ping(client, msg),
        "hello" => public::cmd_hello(client, msg),
        "echo" => public::cmd_echo(client, msg),
        "hangman" => public::cmd_hangman(client, msg, games).await,
        "help" => public::cmd_help(client, msg),
        
        // Restricted commands
        "join" => restricted::cmd_join(client, msg, db),
        "leave" => restricted::cmd_leave(client, msg, db),
        "set" => restricted::cmd_set(client, msg, db),
        "get" => restricted::cmd_get(client, msg, db),
        "del" => restricted::cmd_del(client, msg, db),
        "list" => restricted::cmd_list(client, msg, db),
        
        // Admin commands
        "grant" => admin::cmd_grant(client, msg, db),
        "revoke" => admin::cmd_revoke(client, msg, db),
        "perms" => admin::cmd_perms(client, msg, db),
        
        // Unknown command
        _ => public::cmd_unknown(client, msg),
    }
}
