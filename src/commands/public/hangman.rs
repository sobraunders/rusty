use irc::client::prelude::Client;
use crate::message::ParsedMessage;
use crate::games::GameManager;
use crate::games::hangman::GuessResult;

pub async fn cmd_hangman(client: &Client, msg: &ParsedMessage, games: &GameManager) -> irc::error::Result<()> {
    if msg.args.is_empty() {
        client.send_privmsg(&msg.channel, "Usage: !hangman <start|guess|status|quit>")?;
        return Ok(());
    }

    let subcommand = msg.args[0].as_str();

    match subcommand {
        "start" => hangman_start(client, msg, games).await,
        "guess" => hangman_guess(client, msg, games).await,
        "status" => hangman_status(client, msg, games).await,
        "quit" => hangman_quit(client, msg, games).await,
        _ => {
            client.send_privmsg(&msg.channel, "Unknown hangman subcommand. Use: start, guess, status, quit")?;
            Ok(())
        }
    }
}

async fn hangman_start(
    client: &Client,
    msg: &ParsedMessage,
    games: &GameManager,
) -> irc::error::Result<()> {
    // Check if a game is already running
    if games.hangman_game(&msg.channel).await.is_some() {
        client.send_privmsg(&msg.channel, "A hangman game is already running in this channel! Use !hangman quit to end it.")?;
        return Ok(());
    }

    games.new_hangman(&msg.channel).await;
    if let Some(game) = games.hangman_game(&msg.channel).await {
        client.send_privmsg(&msg.channel, "🎮 Hangman game started! Everyone can play along!")?;
        client.send_privmsg(&msg.channel, &format!("Word: {}", game.display()))?;
        client.send_privmsg(&msg.channel, &format!("Remaining guesses: {}", game.remaining()))?;
        client.send_privmsg(&msg.channel, "Use !hangman guess <letter> to guess a letter")?;
    }
    Ok(())
}

async fn hangman_guess(
    client: &Client,
    msg: &ParsedMessage,
    games: &GameManager,
) -> irc::error::Result<()> {
    if msg.args.len() < 2 {
        client.send_privmsg(&msg.channel, "Usage: !hangman guess <letter>")?;
        return Ok(());
    }

    let letter_str = &msg.args[1];
    if letter_str.len() != 1 {
        client.send_privmsg(&msg.channel, "Please guess a single letter")?;
        return Ok(());
    }

    let letter = letter_str.chars().next().unwrap();
    let author = msg.author.as_ref().map(|s| s.as_str()).unwrap_or("someone");

    match games.hangman_guess(&msg.channel, letter).await {
        None => {
            client.send_privmsg(&msg.channel, "No game is running in this channel. Use !hangman start")?;
        }
        Some(GuessResult::AlreadyGuessed) => {
            client.send_privmsg(&msg.channel, &format!("{} already guessed '{}' 🤔", author, letter))?;
        }
        Some(GuessResult::Correct) => {
            if let Some(game) = games.hangman_game(&msg.channel).await {
                client.send_privmsg(&msg.channel, &format!("{} guessed '{}' - ✓ Correct! Word: {}", author, letter, game.display()))?;
                client.send_privmsg(&msg.channel, &format!("Remaining guesses: {}", game.remaining()))?;
            }
        }
        Some(GuessResult::Wrong) => {
            if let Some(game) = games.hangman_game(&msg.channel).await {
                client.send_privmsg(&msg.channel, &format!("{} guessed '{}' - ✗ Wrong! Word: {}", author, letter, game.display()))?;
                client.send_privmsg(&msg.channel, &format!("Remaining guesses: {}", game.remaining()))?;
            }
        }
        Some(GuessResult::Won) => {
            if let Some(game) = games.hangman_game(&msg.channel).await {
                client.send_privmsg(&msg.channel, &format!("🎉 {} solved it! The word was: {}", author, game.word()))?;
                games.hangman_quit(&msg.channel).await;
            }
        }
        Some(GuessResult::Lost(word)) => {
            client.send_privmsg(&msg.channel, &format!("☠️ Game Over! The word was: {} 😢", word))?;
            games.hangman_quit(&msg.channel).await;
        }
    }
    Ok(())
}

async fn hangman_status(
    client: &Client,
    msg: &ParsedMessage,
    games: &GameManager,
) -> irc::error::Result<()> {
    match games.hangman_game(&msg.channel).await {
        None => {
            client.send_privmsg(&msg.channel, "No game is running in this channel. Use !hangman start")?;
        }
        Some(game) => {
            client.send_privmsg(&msg.channel, &format!("Word: {}", game.display()))?;
            client.send_privmsg(&msg.channel, &format!("Guessed: {}", game.guessed()))?;
            client.send_privmsg(&msg.channel, &format!("Wrong: {}/{}", game.wrong_count(), 6))?;
        }
    }
    Ok(())
}

async fn hangman_quit(
    client: &Client,
    msg: &ParsedMessage,
    games: &GameManager,
) -> irc::error::Result<()> {
    if games.hangman_quit(&msg.channel).await {
        client.send_privmsg(&msg.channel, "Hangman game ended.")?;
    } else {
        client.send_privmsg(&msg.channel, "No game is running in this channel")?;
    }
    Ok(())
}
